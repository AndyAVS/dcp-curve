/* Copyright © 2025 AndyAVS */

use std::error::Error;
use plotters::prelude::*;

const IMG_SIZE: (u32,u32) = (4096, 2160);
const LABEL_TEXT: &str = "DNG Profile Tone Curve";

pub fn plot_tone_curve(
    h_values: &[f64],
    v_values: &[f64],
    output_path: &str,
) -> Result<(), Box<dyn Error>> {
    if h_values.is_empty() || v_values.len() != h_values.len() {
        return Err("Нет данных для построения графика".into());
    }

    // Находим максимальные значения для масштабирования осей
    let max_h = h_values.iter().fold(f64::NEG_INFINITY, |a, &b| a.max(b));
    let max_v = v_values.iter().fold(f64::NEG_INFINITY, |a, &b| a.max(b));

    let root = BitMapBackend::new(output_path, IMG_SIZE).into_drawing_area();
    root.fill(&WHITE)?;

    let mut chart = ChartBuilder::on(&root)
        .caption(output_path, ("sans-serif", 40).into_font())
        .margin(50)
        .x_label_area_size(30)
        .y_label_area_size(30)
        .build_cartesian_2d(0.0..max_h * 1.1, 0.0..max_v * 1.1)?;

    chart
        .configure_mesh()
        .x_desc("h")
        .y_desc("v")
        .axis_desc_style(("sans-serif", 20))
        .draw()?;

    // Собираем точки для графика
    let points: Vec<(f64, f64)> = h_values
        .iter()
        .zip(v_values.iter())
        .map(|(&h, &v)| (h, v))
        .collect();

    // Рисуем линию и точки
    chart
        .draw_series(LineSeries::new(
            points.iter().map(|&(h, v)| (h, v)),
            &RED.mix(0.7),
        ))?
        .label(LABEL_TEXT)
        .legend(|(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], &RED.mix(0.7)));

    chart.draw_series(
        points
            .iter()
            .map(|&(h, v)| Circle::new((h, v), 3, RED.filled())),
    )?;

    // Добавляем легенду
    chart
        .configure_series_labels()
        .background_style(&WHITE.mix(0.8))
        .border_style(&BLACK)
        .draw()?;

    root.present()?;
    Ok(())
}
