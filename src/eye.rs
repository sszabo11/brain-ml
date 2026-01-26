use std::{
    collections::{HashMap, HashSet},
    env::consts::ARCH,
};

use ndarray::{Array1, Array2};
use ndarray_rand::RandomExt;
use rand::{Rng, distr::Uniform};
use rayon::iter::{
    IndexedParallelIterator, IntoParallelIterator, IntoParallelRefMutIterator, ParallelBridge,
    ParallelIterator,
};

pub struct Eye {
    receptors: Array2<f32>,
    bipolars: Array2<f32>,
    radius: usize,
}

#[derive(Debug, Clone)]
enum ReceptorType {
    Rod,
    SCone,
    MCone,
    LCone,
}

#[derive(Debug, Clone)]
struct Photoreceptor {}

#[derive(Debug, Clone)]
enum BipolarType {
    ON,
    OFF,
}

#[derive(Debug, Clone)]
struct BipolarCell {
    cell_type: BipolarType,
}

fn random_receptor(n: f32) {}

impl Eye {
    pub fn new(input_width: usize, input_height: usize, radius: usize) -> Self {
        let mut rng = rand::rng();

        let pxs = input_width * input_height;

        //let r: Array1<Photoreceptor> = (0..pxs)
        //    .map(|_| {
        //        return Photoreceptor {
        //            receptor_type: ReceptorType::Rod,
        //        };
        //    })
        //    .collect();

        //let receptors: Array2<Photoreceptor> = r.to_shape((input_width, input_height)).unwrap();
        //let bipolars: Array2<BipolarCell> = r.to_shape((input_width, input_height)).unwrap();

        Self {
            receptors: Array2::zeros((input_width, input_height)),
            bipolars: Array2::zeros((input_width, input_height)),
            radius,
        }
    }

    pub fn process(&mut self, input: &Array2<(u8, u8, u8)>) -> Array2<f32> {
        //let pixels = input.fl
        //let mut output2 = Array2::zeros(input.dim());
        let mut output = vec![0.0f32; input.nrows() * input.ncols()];

        let width = input.nrows();

        output.par_iter_mut().enumerate().for_each(|(i, pixel)| {
            let x = i % width;
            let y = ((i / width) as f32).floor() as usize;

            //println!("{} {}", x, y);
            let (r, g, b) = input[[x, y]]; // RGB

            let neighbours = self.get_neighbours(x, y); // All 1

            let luminance = 0.299 * r as f32 + 0.587 * g as f32 + 0.114 * b as f32;
            let percant = luminance / 255.0;
            let excitation = 1.0 - percant;

            let sum: f32 = neighbours
                .iter()
                .map(|&(x, y)| {
                    let (r, g, b) = input[[x, y]];
                    let luminance = 0.299 * r as f32 + 0.587 * g as f32 + 0.114 * b as f32;
                    let percant = luminance / 255.0;

                    1.0 - percant
                })
                .sum();

            let avg = sum / neighbours.len() as f32;
            let res = excitation - avg;
            //println!(
            //    "excitatopm: {} | {} | {} | {} {}",
            //    res,
            //    avg,
            //    sum,
            //    excitation,
            //    neighbours.len()
            //);

            *pixel = res;
        });

        //for y in 0..input.ncols() {
        //    //let mut temp = HashMap::new();
        //    //        for x in 0..input.nrows() {
        //    (0..input.nrows()).into_par_iter().for_each(|x| {
        //        let (r, g, b) = input[[x, y]]; // RGB

        //        let neighbours = self.get_neighbours(x, y); // All 1

        //        let luminance = 0.299 * r as f32 + 0.587 * g as f32 + 0.114 * b as f32;
        //        let percant = luminance / 255.0;
        //        let excitation = 1.0 - percant;

        //        let sum: f32 = neighbours
        //            .iter()
        //            .map(|&(x, y)| {
        //                let (r, g, b) = input[[x, y]];
        //                let luminance = 0.299 * r as f32 + 0.587 * g as f32 + 0.114 * b as f32;
        //                let percant = luminance / 255.0;

        //                1.0 - percant
        //            })
        //            .sum();

        //        let avg = sum / neighbours.len() as f32;
        //        let res = excitation - avg;
        //        //println!(
        //        //    "excitatopm: {} | {} | {} | {} {}",
        //        //    res,
        //        //    avg,
        //        //    sum,
        //        //    excitation,
        //        //    neighbours.len()
        //        //);

        //        output[x][y] = res;
        //    })
        //}
        Array2::from_shape_vec((input.ncols(), input.nrows()), output).unwrap()
    }

    pub fn get_neighbours(&self, center_x: usize, center_y: usize) -> Vec<(usize, usize)> {
        let mut neighbours: Vec<(usize, usize)> = Vec::new();

        for x in -(self.radius as isize)..=self.radius as isize {
            for y in -(self.radius as isize)..=self.radius as isize {
                let x =
                    (center_x as isize - x).clamp(0, self.receptors.ncols() as isize - 1) as usize;
                let y =
                    (center_y as isize - y).clamp(0, self.receptors.nrows() as isize - 1) as usize;

                neighbours.push((x, y));
            }
        }

        neighbours
            .into_iter()
            .filter(|n| n.0 != center_x && n.1 != center_y)
            .collect()
    }
}
