use anyhow::Result;
use ndarray::{Array1, Array2};

pub fn encode(image: Vec<u8>, sdr_shape: usize) -> Result<Array1<usize>> {
    assert!(image.len() == 784);

    //let mut sdr = Array2::zeros((sdr_shape, sdr_shape));

    //let img = Array2::from_shape_vec((28, 28), image)?;

    let kernal_size = 5;

    let n = 2;
    let w = 10;

    //for x in img.rows() {
    //    for (y, pixel) in x.iter().enumerate() {
    //

    //
    //    }
    //}

    let sdr: Array1<usize> = mnist_to_sdr_2352(&image).into();

    Ok(sdr)
}

fn mnist_to_sdr_2352(image: &[u8]) -> Vec<usize> {
    const BITS_PER_PIXEL: usize = 3;
    const ACTIVE_BITS: usize = 1;

    let t: Vec<usize> = image
        .iter()
        .flat_map(|&pixel| {
            let mut enc = vec![0; BITS_PER_PIXEL];
            if pixel > 0 {
                let intensity_bin = (pixel as usize * BITS_PER_PIXEL) / 256;
                enc[intensity_bin.min(BITS_PER_PIXEL - 1)] = 1;
            }
            enc
        })
        .collect();

    //println!("{:?} {}", t, t.len());

    //for (i, pixel) in t.iter().enumerate() {
    //    if *pixel == 1 {
    //        print!("■") // "■"█
    //    } else {
    //        print!(" ")
    //    }
    //    if i.is_multiple_of(28 * 3 - 1) {
    //        println!();
    //    }

    //    //println!()
    //}
    t
}
