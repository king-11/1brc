use std::{
  collections::HashMap, fmt::Display, fs::File, io::{BufRead, BufReader}, path::Path
};

use ahash::RandomState;

pub struct CityState {
    pub min: f32,
    pub max: f32,
    sum: f32,
    count: usize,
}

impl CityState {
    pub fn new(initial_recording: f32) -> Self {
        CityState {
            min: initial_recording,
            max: initial_recording,
            sum: initial_recording,
            count: 1,
        }
    }

    pub fn mean(&self) -> f32 {
      return self.sum / (self.count as f32)
    }

    pub fn update(&mut self, recording: f32) {
      self.min = self.min.min(recording);
      self.max = self.max.max(recording);
      self.sum += recording;
      self.count += 1;
    }
}

impl Display for CityState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
      write!(f, "{:.1}/{:.1}/{:.1}", self.min, self.mean(), self.max)
    }
}

pub fn create_buffered_reader(path: &Path) -> BufReader<File> {
    let file = File::open(path).expect("file open");
    BufReader::new(file)
}

pub fn read_file(path: &Path) -> HashMap<String, CityState, RandomState> {
    let mut reader = create_buffered_reader(path);
    let mut current_line = String::with_capacity(200); // 2 bytes * 100 characters UTF-8
    let mut map: HashMap<String, CityState, RandomState> = HashMap::default();
    loop {
        let bytes_read = reader.read_line(&mut current_line).expect("read line");
        if bytes_read == 0 {
            break;
        }
        let (city_name, value) = current_line.split_once(';').expect("invalid format");
        let temperature = fast_float::parse(value.trim()).expect("invalid float");

        map.entry(city_name.to_string()).or_insert_with(|| CityState::new(temperature)).update(temperature);

        current_line.clear();
    }
    map
}
