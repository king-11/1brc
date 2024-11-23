use std::{
    collections::HashMap, fs::File, io::{BufRead, BufReader}, path::Path
};

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

pub fn create_buffered_reader(path: &Path) -> BufReader<File> {
    let file = File::open(path).expect("file open");
    BufReader::new(file)
}

pub fn read_file(path: &Path) -> HashMap<String, CityState> {
    let mut reader = create_buffered_reader(path);
    let mut current_line = String::with_capacity(200); // 2 bytes * 100 characters UTF-8
    let mut map = HashMap::new();
    loop {
        let bytes_read = reader.read_line(&mut current_line).expect("read line");
        if bytes_read == 0 {
            break;
        }
        let record: Vec<&str> = current_line.splitn(2, ';').collect();
        let city_name = record.get(0).expect("city name missing").to_string();
        let temperature: f32 = record
            .get(1)
            .expect("reading missing")
            .trim()
            .parse()
            .expect("invalid float");

        map.entry(city_name).or_insert_with(|| CityState::new(temperature)).update(temperature);

        current_line.clear();
    }
    map
}
