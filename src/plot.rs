use std::{f32::EPSILON, time};

use anyhow::Result;
use ndarray::Array2;
use plotters::{
    chart::ChartBuilder,
    prelude::{BitMapBackend, IntoDrawingArea, Rectangle},
    style::{Color, RGBColor, WHITE},
};

pub fn plot(data: &Array2<f32>, output_path: &str) -> Result<()> {
    let date = time::SystemTime::now()
        .duration_since(time::UNIX_EPOCH)
        .unwrap()
        .as_secs()
        .to_string();

    //let output_path = format!("./output/heatmap-{}.png", date);

    let root = BitMapBackend::new(
        &output_path,
        (data.ncols() as u32 * 2, data.nrows() as u32 * 2),
    )
    .into_drawing_area();

    root.fill(&WHITE).unwrap();

    let cols = data.ncols();
    let rows = data.nrows();

    let mut chart = ChartBuilder::on(&root)
        .caption("Visualize output", ("sans-serif", 20))
        .margin(10)
        .build_cartesian_2d(0..cols, 00..rows)?;

    chart.configure_mesh().draw()?;

    chart.draw_series((0..rows).flat_map(|r| {
        (0..cols).map(move |c| {
            let val = data[[r, c]];
            // Map value to color: low (blue) to high (red)
            //let color = RGBColor((val * 255.0) as u8, 0, ((1.0 - val) * 255.0) as u8);
            let color = RGBColor(
                255,
                ((1.0 - val) * 255.0 / 1.04) as u8,
                ((1.0 - val) * 255.0 / 1.04) as u8,
            );
            Rectangle::new([(c, r), (c + 1, r + 1)], color.filled())
        })
    }))?;

    root.present()?;
    println!("Heatmap saved to {}", output_path);

    Ok(())
}
