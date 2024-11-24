use plotters::prelude::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 创建绘制区域，保存为图像
    let root = BitMapBackend::new("output.png", (800, 600)).into_drawing_area();
    root.fill(&WHITE)?;

    // 创建坐标系
    let mut chart = ChartBuilder::on(&root)
        .caption("Example Line Chart", ("sans-serif", 50))
        .x_label_area_size(30)
        .y_label_area_size(30)
        .build_cartesian_2d(0..10, 0..100)?;

    chart.configure_mesh().draw()?;

    // 绘制折线
    chart.draw_series(LineSeries::new(
        (0..10).map(|x| (x, x * x)),
        &BLUE,
    ))?;

    root.present()?;
    println!("Chart saved to output.png");

    Ok(())
}
