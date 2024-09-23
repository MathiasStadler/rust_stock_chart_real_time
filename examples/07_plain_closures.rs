
use chrono::{ Utc, TimeZone };
fn main() {
    // FROM HERE
    // https://www.linkedin.com/pulse/rust-closure-cookbook-patterns-tips-best-practices-luis-soares-m-sc--44wsf

    // Iterating Over a Collection

    const DATA: [f64; 14] = [
        137.24, 136.37, 138.43, 137.41, 139.69, 140.41, 141.58, 139.55, 139.68, 139.1, 138.24, 135.67,
        137.12, 138.12,
    ];

    let doubled: Vec<_> = (0..)
        .zip(DATA.iter())
        .map(|(idx, price)| {
            let day = (idx / 5) * 7 + (idx % 5) + 1;
            let date = Utc.with_ymd_and_hms(2019, 10, day,0,0,0);
            (date, *price)
        })
        .collect();

    println!("Doubled numbers: {:?}", doubled); // Outputs: Doubled numbers:
}
