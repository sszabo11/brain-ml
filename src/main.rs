use ndarray::{Array1, Array2};
mod htm;
use ndarray_rand::RandomExt;
use rand::{Rng, distr::Uniform};

use crate::htm::HTM;

//use crate::data::decode_images;

fn main() {
    let sdr = SDR::new(64, 0.02);

    let input = sdr.values;

    let mut htm = HTM::new(4096, 32, 0.2, 0.5, 0.03);

    let f = input[0].flatten().to_owned();

    assert!(f.len() == 4096);

    htm.compute_overlap(&f);

    let mut winners = htm.compute_winner();

    htm.update(&f, &mut winners);

    //let images = decode_images().unwrap();
    //println!("{}", images);
    //sdr.draw();
}

struct SDR {
    values: Vec<Array2<u16>>,
}

impl SDR {
    pub fn new(len: usize, sparsity: f32) -> Self {
        let on_bits = (len as f32 * len as f32 * sparsity).round() as u16;

        let mut values: Array2<u16> = Array2::zeros((len, len));

        let mut rand = rand::rng();
        for _i in 0..on_bits {
            let row: u16 = rand.random_range(0..len as u16);
            let col: u16 = rand.random_range(0..len as u16);
            values.row_mut(row as usize)[col as usize] = 1;
        }

        Self {
            values: vec![values],
        }
    }

    pub fn train(&mut self, data: &[u8]) {
        for d in data {}
    }

    pub fn draw(&self) {
        for item in self.values.iter() {
            for row in item.rows() {
                for col in row.into_iter() {
                    if *col == 1 {
                        print!("■") //●
                    } else {
                        print!("□")
                    }
                }
                println!()
            }
        }
    }
}

struct Network {
    weights: Array2<f32>,
    biases: Array2<f32>,

    num_layers: usize,
    num_neurons: usize,
}

impl Network {
    pub fn new(num_layers: usize, num_neurons: usize) -> Self {
        Self {
            num_neurons,
            num_layers,
            weights: Array2::random((num_layers, num_neurons), Uniform::new(-1.0, 1.0).unwrap()),
            biases: Array2::random((num_layers, num_neurons), Uniform::new(-1.0, 1.0).unwrap()),
        }
    }
}
