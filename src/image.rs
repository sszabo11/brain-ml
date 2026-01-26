use anyhow::Result;
use ndarray::{Array2, Array3, ArrayView2, s};
use rust_mnist::Mnist;

pub fn decode_image() -> Result<Vec<Vec<u8>>> {
    let m = Mnist::new("./data/minst/");

    let imgs: Vec<Vec<u8>> = m.train_data.iter().take(1000).map(|v| v.to_vec()).collect();

    //for image in &imgs {
    //    let img = Array2::from_shape_vec((28, 28), image.to_vec()).unwrap();

    //    for row in img.rows() {
    //        for pixel in row.iter() {
    //            if *pixel > 170 {
    //                print!("██") // "■"█
    //            } else {
    //                print!("  ")
    //            }
    //        }
    //        println!();
    //        //println!()
    //    }

    //    println!("\n------------------")
    //}

    println!("{:?} {}", imgs[0].len(), imgs.len());
    Ok(imgs)
}
