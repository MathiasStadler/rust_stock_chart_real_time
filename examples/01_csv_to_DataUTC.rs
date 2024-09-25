// read csv and convert to Date<UTC>

// use chrono::NaiveDate;
use chrono::{ NaiveDate, NaiveTime };

fn main() {
    // read csv
    let mut reader = csv::Reader::from_path("stock_data/stock_trex_data.csv").unwrap();

    for record in reader.deserialize() {
        #[allow(unused_variables)]
        let (date, open, high, low, close, volume): (
            String,
            f64,
            f64,
            f64,
            f64,
            f64,
        ) = record.unwrap();

        let _csv_date = NaiveDate::parse_from_str(&date, "%Y-%m-%d")
            .unwrap()
            .and_time(NaiveTime::parse_from_str("00:00:00", "%H:%M:%S").unwrap())
            .and_utc();
        println!("{},{},{}", date, _csv_date, close);
    }
}

// cargo run --example 01_csv_to_DataUTC
