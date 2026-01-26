use image::GenericImageView;
use ndarray::{Array1, Array2};

pub fn square(path: &str) {}

pub fn process_image(path: &str) -> Array2<(u8, u8, u8)> {
    let img = image::open(path).unwrap();

    let len = img.width() * img.height();
    let input: Array1<(u8, u8, u8)> = (0..len).map(|i| (0, 0, 0)).collect();

    let mut res: Array2<(u8, u8, u8)> = input
        .to_shape((img.width() as usize, img.height() as usize))
        .unwrap()
        .to_owned();

    for pixel in img.pixels() {
        let (x, y, color) = pixel;
        let [r, g, b, _] = color.0;

        let x = x as usize;
        let y = y as usize;

        res[[x, y]] = (r, g, b);
    }

    assert!(res.ncols() == img.height() as usize);
    assert!(res.nrows() == img.width() as usize);
    res
}
