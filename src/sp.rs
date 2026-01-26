use std::collections::HashSet;

use ndarray::{Array1, Array2, Array3, ArrayView1};
use ndarray_rand::RandomExt;
use rand::distr::Uniform;

pub struct SpatialPooler {
    pub num_columns: usize,
    threshold: f32,
    boost_strength: f32,
    inhibition_radius: usize,
    num_active_cols_per_area: usize,

    num_connections: usize,
    syn_perm_active_lr: f32,
    period: u32,
    duty_cycle_decay: f32,

    pub connections: Array2<usize>,
    pub permanences: Array2<f32>,
    boosted_overlaps: Array1<f32>,
    stimilus_threshold: f32,
    overlap_scores: Array1<f32>,
    active_duty_cycles: Array1<f32>,
    overlap_duty_cycles: Array1<f32>,
}

impl SpatialPooler {
    pub fn new(
        num_cols: usize,
        _cells: usize,
        threshold: f32,
        _boost_strength: f32,
        lr: f32,
    ) -> Self {
        let inhibition_radius = 6;
        let num_connections = 300;
        Self {
            num_columns: num_cols,
            boost_strength: 3.0,
            syn_perm_active_lr: lr,
            duty_cycle_decay: 0.001,
            threshold,
            period: 1000,
            inhibition_radius,
            num_connections,
            num_active_cols_per_area: 2,
            stimilus_threshold: 2.0,

            boosted_overlaps: Array1::zeros(num_cols),
            overlap_scores: Array1::zeros(num_cols),
            active_duty_cycles: Array1::zeros(num_cols),
            overlap_duty_cycles: Array1::zeros(num_cols),

            connections: Array2::random(
                (num_cols, num_connections),
                Uniform::new(0, num_cols - 1).unwrap(),
            ),
            permanences: Array2::random(
                (num_cols, num_connections),
                Uniform::new(threshold - 0.4, threshold + 0.4).unwrap(),
            ),
        }
    }

    pub fn compute_overlap(&mut self, input: &Array1<usize>) {
        for (col, c) in self.connections.rows().into_iter().enumerate() {
            let mut score = 0.0;
            for (i, idx) in c.iter().enumerate() {
                let bit = input[*idx];
                let permanence = self.permanences[[col, i]];

                if bit == 1 && permanence > self.threshold {
                    score += permanence;
                }
            }
            self.overlap_scores[col] = score;
        }

        // Update overlap_duty_cycle for each col
        for col in 0..self.num_columns {
            let was_good = self.overlap_scores[col] >= self.stimilus_threshold;

            self.overlap_duty_cycles[col] = (1.0 - self.duty_cycle_decay)
                * self.overlap_duty_cycles[col]
                + self.duty_cycle_decay * (was_good as u32 as f32);
        }
    }

    pub fn apply_boost(&mut self) {
        for c in 0..self.num_columns {
            let target = 0.02;
            let boost = if self.active_duty_cycles[c] < target {
                1.0 + self.boost_strength * (target - self.active_duty_cycles[c])
            } else {
                1.0
            };
            self.boosted_overlaps[c] = self.overlap_scores[c] * boost;
        }
    }
    pub fn compute_winners(&mut self) -> Vec<usize> {
        let mut winners = HashSet::new();

        for col in 0..self.num_columns {
            let neighbours = self.get_neighbours(col);

            if neighbours.is_empty() {
                continue;
            };

            let mut scores: Vec<f32> = neighbours
                .iter()
                .map(|n| self.boosted_overlaps[*n])
                .collect();

            scores.push(self.boosted_overlaps[col]);

            scores.sort_by(|a, b| b.partial_cmp(a).unwrap());

            let k = self.num_active_cols_per_area;

            let min_activity = if scores.len() >= k {
                scores[k - 1]
            } else {
                0.0
            };

            let overlap = self.boosted_overlaps[col];
            if overlap > self.stimilus_threshold && overlap >= min_activity {
                winners.insert(col);
            };
        }

        println!("winners: {}", winners.len());

        for &c in &winners {
            self.active_duty_cycles[c] = (1.0 - self.duty_cycle_decay) * self.active_duty_cycles[c]
                + self.duty_cycle_decay * 1.0
        }

        winners.into_iter().collect()
    }

    fn get_neighbours(&self, col_idx: usize) -> Vec<usize> {
        let min = col_idx.saturating_sub(self.inhibition_radius);
        let max = (col_idx + self.inhibition_radius).min(self.num_columns - 1);

        let neighbours = (min..=max).into_iter().filter(|v| *v != col_idx).collect(); // Don't compete with self

        neighbours
    }

    pub fn update(&mut self, input: &Array1<usize>, winners: &[usize]) {
        for &col in winners.iter() {
            let conns = self.connections.row(col);
            let mut perms = self.permanences.row_mut(col);

            for (input_idx, permanence) in conns.iter().zip(perms.iter_mut()) {
                let bit = input[*input_idx];

                if bit == 1 {
                    *permanence += self.syn_perm_active_lr;
                    *permanence = f32::min(1.0, *permanence); // Constrain between 0 and 1
                } else {
                    *permanence -= self.syn_perm_active_lr;
                    *permanence = f32::max(0.0, *permanence); // Constrain between 0 and 1
                }
            }
        }

        let mut active_cols = vec![false; self.num_columns];
        for &winner in winners {
            active_cols[winner] = true;
        }

        for i in 0..self.num_columns {
            let alpha = 1.0 / self.period as f32; // e.g., period = 1000
            self.active_duty_cycles[i] =
                (1.0 - alpha) * self.active_duty_cycles[i] + alpha * (active_cols[i] as u8 as f32);
        }

        for i in 0..self.num_columns {
            let neighbour_idxs = self.get_neighbours(i);
            let max_duty_cycle = neighbour_idxs
                .iter()
                .map(|&idx| self.active_duty_cycles[idx])
                .fold(0.0f32, |a, b| a.max(b));

            let min_duty_cycle = 0.01 * max_duty_cycle;
            let active_duty_cycle = self.active_duty_cycles[i];

            // Boost columns below minimum activity
            //if active_duty_cycle < min_duty_cycle && min_duty_cycle > 0.0 {
            //    self.boost_factors[i] =
            //        (self.boost_factors[i] + self.boost_strength * 0.1).min(10.0);
            //} else {
            //    self.boost_factors[i] =
            //        (self.boost_factors[i] - self.boost_strength * 0.1).max(1.0);
            //}
        }
    }
}
