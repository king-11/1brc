use abrc::*;
use std::path::Path;
use rayon::prelude::*;

fn main() {
    let path = Path::new("measurements.txt");
    let city_state = read_file(path);
    let results: Vec<String> = city_state
        .par_iter()
        .map(|(city_name, city)| format!("{}={}", city_name, city))
        .collect();
    println!("{{{}}}", results.join(", "))
}
