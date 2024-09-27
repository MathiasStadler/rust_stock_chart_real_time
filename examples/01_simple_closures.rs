use chrono::Utc;
use chrono::TimeZone;

const DATA: [f64; 14] = [
    137.24, 136.37, 138.43, 137.41, 139.69, 140.41, 141.58, 139.55, 139.68, 139.1, 138.24, 135.67,
    137.12, 138.12,
];

fn main() {
    println!("simple closures");

    let one_closures = (0..).zip(DATA.iter()).map(|(idx, price)| {
        let day = (idx / 5) * 7 + (idx % 5) + 1;
        #[allow(deprecated)]
        let date = Utc.ymd(2019, 10, day);

        println!("DEBUG 2: {},{}", date, price);

        (date, *price)
    });

    // println!("{}",one_closures.into_iter());

    for v in DATA.into_iter() {
        println!("v => {:?}", v);
    }

    for v in one_closures.into_iter() {
        #[allow(deprecated)]
        let tmp: (chrono::Date<Utc>, f64) = v;
        println!("t1 => {}", tmp.0);
        println!("t2 => {}", tmp.1);
    }
}
