use anyhow::Result;
mod blur;
mod eye;
mod plot;
use ndarray::{Array1, Array2};
mod data;
mod encoder;
mod htm;
mod image;
mod sp;
use ndarray_rand::RandomExt;
use rand::{Rng, distr::Uniform};

use crate::{
    blur::process_image, encoder::encode, eye::Eye, htm::HTM, image::decode_image, plot::plot,
    sp::SpatialPooler,
};

fn main() -> Result<()> {
    //let sdr = SDR::new(50, 0.02);

    //let mut rng = rand::rng();
    //let input: Array1<(u8, u8, u8)> = (0..len)
    //    .map(|i| {
    //        let r: u8 = rng.random_range(0..=255);
    //        let g: u8 = rng.random_range(0..=255);
    //        let b: u8 = rng.random_range(0..=255);
    //        (r, g, b)
    //    })
    //    .collect();

    //let input: Array2<(u8, u8, u8)> = input.to_shape((100, 100)).unwrap().to_owned();

    //assert!(input.nrows() == 100);
    //assert!(input.ncols() == 100);

    let input = process_image("./images/bird-small.jpg");
    let cols = input.ncols();
    let rows = input.nrows();
    //for i in 1..=100 {

    let mut eye = Eye::new(cols, rows, 1);

    let start = std::time::Instant::now();
    let data = eye.process(&input);

    let path = format!("./output/heatmap-bss.png");
    println!(
        "elapsed: {} | pxs: {}",
        start.elapsed().as_millis(),
        input.len()
    );
    plot(&data, &path).unwrap();
    //}

    return Ok(());
    let images = decode_image().unwrap();

    let image_sdrs: Vec<Array1<usize>> = images
        .into_iter()
        .map(|i| encode(i, 2352).unwrap())
        .collect();

    //let input = sdr.values;

    //let mut htm = HTM::new(2500, 32, 0.2, 0.5, 0.03);

    let mut sp = SpatialPooler::new(2352, 32, 0.2, 0.5, 0.03);

    for (i, image) in image_sdrs.iter().enumerate() {
        if i.is_multiple_of(100) {
            println!("Image: {}", i);
        };

        assert!(image.len() == 2352);

        sp.compute_overlap(image);
        sp.apply_boost();

        let mut winners = sp.compute_winners();
        sp.update(image, &mut winners);
    }

    Ok(())
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
