use chrono::Utc;
use chrono::TimeZone;
#[allow(unused_imports)]
use log::{ debug, error, log_enabled, info, Level };

const DATA: [f64; 14] = [
    137.24, 136.37, 138.43, 137.41, 139.69, 140.41, 141.58, 139.55, 139.68, 139.1, 138.24, 135.67,
    137.12, 138.12,
];

// fn main() {
fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::init();
    info!("simple closures {}", 1);
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

    for v in one_closures.clone().into_iter() {
        #[allow(deprecated)]
        let tmp: (chrono::Date<Utc>, f64) = v;
        println!("t1 => {}", tmp.0);
        println!("t2 => {}", tmp.1);
    }

    let output = one_closures.into_iter().map(|(idx, price)| {
        //println!("c1 => {}", idx);
        debug!("date =>  {}, price => {}", idx, price);
        //println!("c1 => {}", price);
        (idx, price)
    });

    println!("output=> {:?}", output);

    for i in output.clone().into_iter() {
        println!("i1 => {} , {}", i.0,i.1);
    }

    //wihout for loop
    // The answer lies in the fact that the for…in syntax, by default, calls the .into_iter() method,
    // which returns an iterator that takes ownership of values being iterated.
    let first_item = output.into_iter().next();
    
    // hanling optuion
    // https://stackoverflow.com/questions/64996954/how-can-i-pull-data-out-of-an-option-for-independent-use
    
    if let Some(field) = first_item {
        println!("hi {}",field.0);
        // more logic involving name
    }
    // println!("next => {}", first_item.get0);
    // println!("next => {}", first_item.1);
    Ok(())
}

//  RUST_LOG=info  cargo run --example 02_simple_closures
