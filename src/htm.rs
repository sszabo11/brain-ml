use ndarray::{Array1, Array2};
use ndarray_rand::RandomExt;
use rand::distr::Uniform;

pub struct HTM {
    pub columns: Vec<Column>,
    threshold: f32,
    boost_strength: f32,
    inhibition_radius: usize,
    num_active_cols_per_area: usize,

    num_connections: usize,
    syn_perm_active_lr: f32,
    period: u32,
    duty_cycle_decay: f32,
}

#[derive(Debug, Clone)]
pub struct Column {
    pub connections: Array1<u16>,
    pub permanences: Array1<f32>,

    pub boost: f32,
    overlap_score: f32,
    active_duty_cycle: f32, // How often column has been active after inhibition
    overlap_duty_cycle: f32, // How often column has had significant overlap
}

impl Column {
    pub fn new(cells: usize, num_connections: usize, threshold: f32) -> Self {
        Self {
            overlap_score: 0.0,
            connections: Array1::random(
                num_connections,
                Uniform::new(0, cells as u16 - 1).unwrap(),
            ),
            permanences: Array1::random(
                num_connections,
                Uniform::new(threshold - 0.5, threshold + 0.5).unwrap(),
            ),
            active_duty_cycle: 0.0,
            boost: 0.0,
            overlap_duty_cycle: 0.0,
        }
    }
}

impl HTM {
    pub fn new(dim: usize, cells: usize, threshold: f32, boost_strength: f32, lr: f32) -> Self {
        let inhibition_radius = 6;
        let num_connections = 300;
        Self {
            columns: (0..dim)
                .map(|_| Column::new(cells, num_connections, threshold))
                .collect(),
            boost_strength,
            syn_perm_active_lr: lr,
            duty_cycle_decay: 0.001,
            threshold,
            period: 4,
            inhibition_radius,
            num_connections,
            num_active_cols_per_area: 6,
        }
    }

    pub fn compute_overlap(&mut self, input: &Array1<u16>) {
        println!("{}", self.columns[0].permanences.len());
        for c in self.columns.iter_mut() {
            for (i, pos) in c.connections.iter().enumerate() {
                let permanence = c.permanences[i];
                let bit = input.get(*pos as usize).unwrap();
                if *bit == 1 && permanence >= self.threshold {
                    c.overlap_score += 1.0;
                }
                //c.overlap_score *= c.boosted_overlap;
            }
        }

        //for c in self.columns.iter_mut() {
        //    let was_good_overlap = c.overlap_score >= self.threshold;
        //    c.overlap_duty_cycle = (1.0 - self.duty_cycle_decay) * c.overlap_duty_cycle
        //        + self.duty_cycle_decay * (if was_good_overlap { 1.0 } else { 0.0 });
        //}
    }

    fn get_neighbours(&self, col_idx: usize) -> Vec<usize> {
        let mut neighbours = Vec::new();

        let num_cols = self.columns.len();

        neighbours.extend(col_idx..(usize::min(num_cols, col_idx + self.inhibition_radius)));
        neighbours.extend(
            col_idx..(isize::max(0, col_idx as isize - self.inhibition_radius as isize) as usize),
        );

        neighbours
    }

    pub fn compute_winner(&mut self) -> Vec<Column> {
        let mut winners: Vec<Column> = Vec::new();

        //for (i, c) in self.columns.iter().enumerate() {
        for i in 0..self.columns.len() {
            let neighbours: Vec<usize> = self.get_neighbours(i);
            println!("n len: {}", neighbours.len());

            let mut scores: Vec<&Column> = neighbours
                .into_iter()
                .map(|idx| self.columns.get(idx).unwrap())
                .collect();

            scores.sort_by(|a, b| b.overlap_score.total_cmp(&a.overlap_score));

            let wins: Vec<Column> = scores
                .into_iter()
                .take(self.num_active_cols_per_area)
                .filter(|w| w.overlap_score > 0.0)
                .cloned()
                .collect();

            wins.iter().enumerate().for_each(|(i, w)| {
                self.columns[i].active_duty_cycle = (1.0 - self.duty_cycle_decay)
                    * self.columns[i].active_duty_cycle
                    + self.duty_cycle_decay * (1.0);
            });
            winners.extend(wins);
        }

        println!(
            "winner len: {} spars: {:?}",
            winners.len(),
            self.columns.len() / winners.len()
        );

        winners
    }

    fn update_active_duty_cycle(&mut self, active_columns: Vec<usize>) {
        for (i, c) in self.columns.iter_mut().enumerate() {
            c.active_duty_cycle = (c.active_duty_cycle * (self.period - 1) as f32
                + active_columns[i] as u8 as f32)
                / self.period as f32;
        }
    }

    fn update_overlap_duty_cycle(&mut self, overlap: &[f32]) {
        for (i, c) in self.columns.iter_mut().enumerate() {
            c.overlap_duty_cycle =
                (c.overlap_duty_cycle * (self.period - 1) as f32 + overlap[i]) / self.period as f32;
        }
    }

    pub fn update(&mut self, input: &Array1<u16>, winners: &mut [Column]) {
        for c in winners.iter_mut() {
            for (i, pos) in c.connections.iter().enumerate() {
                let bit = input.get(*pos as usize).unwrap();
                let permanence = c.permanences.get_mut(i).unwrap();

                if *bit == 1 {
                    //println!("1");
                    *permanence += self.syn_perm_active_lr;
                    *permanence = f32::min(1.0, *permanence); // Constrain between 0 and 1
                } else {
                    //println!("0");
                    *permanence -= self.syn_perm_active_lr;
                    *permanence = f32::max(0.0, *permanence); // Constrain between 0 and 1
                }
            }
        }

        for i in 0..self.columns.len() {
            let neighbour_idxs = self.get_neighbours(i);

            let mut cycles: Vec<f32> = neighbour_idxs
                .into_iter()
                .map(|idx| self.columns.get(idx).unwrap().active_duty_cycle)
                .collect();

            cycles.sort_by(|a, b| b.total_cmp(a));

            let min_duty_cycle = 0.01 * cycles[0];

            let target_activity = 0.02;

            let c = self.columns.get_mut(i).unwrap();
            let boost_factor = if c.active_duty_cycle < target_activity {
                1.0 + self.boost_strength * (target_activity - c.active_duty_cycle)
            } else {
                1.0
            };
            c.overlap_score *= boost_factor;
        }
    }
}
