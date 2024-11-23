use std::path::Path;
use abrc::*;
fn main() {
  let path = Path::new("measurements.txt");
  let file_content = read_file(path);
  let city_recordings = cluster_values(&file_content);
  let results: Vec<String> = city_recordings
    .iter()
    .map(|city| format!("{}={:?}/{}/{:?}", city.city, city.min(), city.avg(), city.max()))
      .collect();
  println!("{{{}}}", results.join(", "))
}
