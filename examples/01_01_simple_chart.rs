// FROM HERE
// https://plotters-rs.github.io/book/basic/basic_data_plotting.html

use plotters::prelude::*;
use chrono::{ Utc, TimeZone };

fn main() {
    let root_area = BitMapBackend::new(
        "images/01_1_simple_chart.png",
        (600, 400)
    ).into_drawing_area();
    root_area.fill(&WHITE).unwrap();

    let start_date = Utc.ymd(2019, 10, 1);
    let end_date = Utc.ymd(2019, 10, 18);

    let mut ctx = ChartBuilder::on(&root_area)
        .set_label_area_size(LabelAreaPosition::Left, 40)
        .set_label_area_size(LabelAreaPosition::Bottom, 40)
        .caption("MSFT daily close price", ("sans-serif", 40))
        .build_cartesian_2d(start_date..end_date, 130.0..145.0)
        .unwrap();

    ctx.configure_mesh().draw().unwrap();

    let data_vec = [
        ("2019 - 10 - 01Z", 137.24),
        ("2019 - 10 - 02Z", 136.37),
        ("2019 - 10 - 03Z", 138.43),
        ("2019 - 10 - 04Z", 137.41),
        ("2019 - 10 - 05Z", 139.69),
        ("2019 - 10 - 08Z", 140.41),
        ("2019 - 10 - 09Z", 141.58),
        ("2019 - 10 - 10Z", 139.55),
        ("2019 - 10 - 11Z", 139.68),
        ("2019 - 10 - 12Z", 139.1),
        ("2019 - 10 - 15Z", 138.24),
        ("2019 - 10 - 16Z", 135.67),
        ("2019 - 10 - 17Z", 137.12),
        ("2019 - 10 - 18Z", 138.12),
    ];

    ctx.draw_series(LineSeries::new(data_vec, &BLUE)).unwrap();
}
const DATA: [f64; 14] = [
    137.24, 136.37, 138.43, 137.41, 139.69, 140.41, 141.58, 139.55, 139.68, 139.1, 138.24, 135.67,
    137.12, 138.12,
];

// cargo run --example
// cargo run --example 01_simple_chart
