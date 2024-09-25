// FROM HERE
// https://plotters-rs.github.io/book/basic/basic_data_plotting.html

// use plotters::prelude::*;

use std::path::Path;

use plotters::prelude::BLUE;
use plotters::prelude::WHITE;
use plotters::backend::BitMapBackend;
use plotters::prelude::LineSeries;
use plotters::prelude::LabelAreaPosition;
use plotters::prelude::ChartBuilder;
use plotters::drawing::IntoDrawingArea;

use chrono::TimeZone;
use chrono::Utc;

use csv::Writer;

// use serde::Deserialize;

use chrono::NaiveDate;

// #[derive(Deserialize)]
// struct Record {
//     year: u16,
//     make: String,
//     model: String,
//     description: String,
// }

fn main() {
    let this_file = file!();
    println!("filename: {}", this_file);

    let name_only = Path::new(this_file)
        .file_name()
        .and_then(|s| s.to_str())
        .unwrap();

    let output_png = name_only.replace("rs", "png");
    let output_csv = name_only.replace("rs", "csv");

    let output_png_filename: String = format!("images/{output_png}");
    let output_csv_filename: String = format!("csv/{output_csv}");

    println!("target png filename: {}", output_png_filename);
    println!("target csv filename: {}", output_csv_filename);

    let mut wtr = Writer::from_path(&output_csv_filename).unwrap();

   

    let root_area = BitMapBackend::new(&output_png_filename, (1920, 1080)).into_drawing_area();
    root_area.fill(&WHITE).unwrap();

    #[allow(deprecated)]
    let start_date = Utc.ymd(2019, 10, 1);
    #[allow(deprecated)]
    let end_date = Utc.ymd(2019, 10, 18);

    let mut ctx = ChartBuilder::on(&root_area)
        .set_label_area_size(LabelAreaPosition::Left, 40)
        .set_label_area_size(LabelAreaPosition::Bottom, 40)
        .caption("MSFT daily close price", ("sans-serif", 40))
        .build_cartesian_2d(start_date..end_date, 130.0..145.0)
        .unwrap();

    ctx.configure_mesh().draw().unwrap();

    let my_closures = (0..).zip(DATA.iter()).map(|(idx, price)| {
        let day = (idx / 5) * 7 + (idx % 5) + 1;
        #[allow(deprecated)]
        let date = Utc.ymd(2019, 10, day);
        // parse timestamp to UTC
        //DateTime::parse_from_str
        //https://docs.rs/chrono/latest/chrono/
        println!("DEBUG 1: idx => {}, day => {}, price => {}", idx, day, price);
        println!("DEBUG 2: {},{}", date, price);
        wtr.write_record(&[date.to_string(), price.to_string()]).unwrap();
        (date, *price)
    });

    let line_series_data = LineSeries::new(my_closures, &BLUE);

    ctx.draw_series(line_series_data).unwrap();
}
const DATA: [f64; 14] = [
    137.24, 136.37, 138.43, 137.41, 139.69, 140.41, 141.58, 139.55, 139.68, 139.1, 138.24, 135.67,
    137.12, 138.12,
];

// cargo run --example
// cargo run --example 10_simple_chart
