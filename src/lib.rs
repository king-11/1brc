use std::{collections::BTreeMap, path::Path};

pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

pub struct TemperatureRecording {
    pub city: String,
    pub value: f32,
}

impl TemperatureRecording {
    fn new(city_name: String, temperature: f32) -> Self {
        TemperatureRecording {
            city: city_name,
            value: temperature,
        }
    }
}

pub struct CityRecording {
    pub city: String,
    pub readings: Vec<f32>,
}

impl CityRecording {
    fn new(city_name: String, readings: Vec<f32>) -> Self {
        let mut readings = readings;
        readings.sort_by(|a, b| a.partial_cmp(b).expect("uncomparable floats"));
        CityRecording {
            city: city_name,
            readings,
        }
    }

    pub fn min(&self) -> f32 {
        *self.readings.get(0).expect("empty readings")
    }

    pub fn max(&self) -> f32 {
        *self
            .readings
            .get(self.readings.len() - 1)
            .expect("empty readings")
    }

    pub fn avg(&self) -> f32 {
        if self.readings.len() == 0 {
            return 0f32;
        }

        self.readings.iter().sum::<f32>() / self.readings.len() as f32
    }
}

pub fn read_file(path: &Path) -> Vec<TemperatureRecording> {
    let file_content = std::fs::read_to_string(path).expect("file read failed");
    let file_lines: Vec<&str> = file_content.split('\n').collect();
    file_lines
        .iter()
        .map(|&line| line.trim())
        .filter(|&line| !line.is_empty())
        .map(|line| {
            let record: Vec<&str> = line.splitn(2, ';').collect();
            // println!("{:?}", record);
            let city_name = record.get(0).expect("city name missing").to_string();
            let temperature = record
                .get(1)
                .expect("reading missing")
                .parse()
                .expect("invaluid float");
            TemperatureRecording::new(city_name, temperature)
        })
        .collect()
}

pub fn cluster_values(temperature_recordings: &Vec<TemperatureRecording>) -> Vec<CityRecording> {
    let mut map = BTreeMap::new();
    temperature_recordings.iter().for_each(|reading| {
        map.entry(reading.city.clone())
            .or_insert_with(Vec::new)
            .push(reading.value)
    });

    // map.iter().for_each(|(key, value)| println!("{}: {:?}", key, value));

    map.iter()
        .map(|(key, values)| CityRecording::new(key.clone(), values.clone()))
        .collect()
}
