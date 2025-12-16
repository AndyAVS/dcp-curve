/* Copyright © 2025 AndyAVS */

use std::error::Error;
use std::{env, fs};

mod curve_parser;
mod img_creator;

use curve_parser::parse_tone_curve_data;
use curve_parser::print_values;
use img_creator::plot_tone_curve;

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();

    let input_file = if args.len() > 1 {
        &args[1]
    } else {
        println!("Укажите имя входного xml файла, полученного утилитой dcpTool");
        return Ok(());
    };

    println!("Читаем {}", input_file);
    let content = fs::read_to_string(input_file)?;

    println!("Парсим {}", input_file);
    let (h_values, v_values) = parse_tone_curve_data(&content)?;

    println!("Найдено {} точек", h_values.len());
    print_values(&h_values, &v_values);

    // Построение графика
    let jpeg_name = format!("{input_file}.curve.jpg");
    println!("Рисуем...");
    plot_tone_curve(&h_values, &v_values, &jpeg_name)?;
    println!("График сохранен в {jpeg_name}");

    Ok(())
}

