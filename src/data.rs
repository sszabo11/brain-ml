use std::fs;

use anyhow::Result;
use ndarray::{Array1, Array2};

pub fn decode_images() -> Result<Array2<u8>> {
    let bytes = fs::read("./data/minst/train-images.idx3-ubyte")?;

    //println!("{:?}", header);

    //for byte in bytes.iter() {
    //    println!("{}", byte);
    //}

    let data: Vec<&[u8]> = bytes.chunks(784).collect();

    //for image in data.iter() {
    //    for (i, pixel) in image.iter().enumerate() {
    //        if *pixel > 160 {
    //            print!("■")
    //        } else {
    //            print!("□")
    //        }
    //        if i.is_multiple_of(27) && i != 1 {
    //            println!();
    //        }
    //    }
    //    println!("-----");
    //}

    let rows = data.len();
    let cols = data[0].len();

    let flattened: Vec<u8> = data.into_iter().flatten().cloned().collect();

    println!("{} {} {}", rows, cols, flattened.len());

    let images = Array2::from_shape_vec((rows, cols), flattened)?;

    Ok(images)
}
