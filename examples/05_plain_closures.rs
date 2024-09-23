fn main() {
    // FROM HERE
    // https://www.linkedin.com/pulse/rust-closure-cookbook-patterns-tips-best-practices-luis-soares-m-sc--44wsf

    let increment_by = 3;
    let add = |num| num + increment_by;

    println!("4 incremented by 3 is: {}", add(4)); // Outputs: 4 incremented
}
