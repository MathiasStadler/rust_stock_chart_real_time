// FROM HERE
// https://plotters-rs.github.io/book/basic/basic_data_plotting.html

use plotters::prelude::*;
use chrono::{ Utc, TimeZone };

fn main() {
    let root_area = BitMapBackend::new(
        "images/09_simple_chart.png",
        (600, 400)
    ).into_drawing_area();
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

    let v = (0..).zip(DATA.iter()).map(|(idx, price)| {
        // ignore dummy/pseudo  weekends
        let day = (idx / 5) * 7 + (idx % 5) + 1;
        println!("idx => {} day => {}", idx, day);
        #[allow(deprecated)]
        let date = Utc.ymd(2019, 10, day);
        println!{"{},{}",date,price};
        (date, *price)
    });

    ctx.draw_series(LineSeries::new(v, &BLUE)).unwrap();
}
const DATA: [f64; 14] = [
    137.24, 136.37, 138.43, 137.41, 139.69, 140.41, 141.58, 139.55, 139.68, 139.1, 138.24, 135.67,
    137.12, 138.12,
];

// cargo run --example
// cargo run --example 09_simple_chart
