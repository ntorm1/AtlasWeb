use csv::ReaderBuilder;
use serde::Serialize;

use crate::standard::*;

#[derive(Debug, Serialize, Clone)]
pub struct Asset {
    timestamps: Vec<i64>,
    data: Vec<f64>,
    source: Option<String>,
    datetime_format: String,
    name: String,
    id: usize,
    rows: usize,
    cols: usize,
    headers: AtlasMap<String, usize>,
}

impl Asset {
    pub fn new(id: usize, name: &str, data: Vec<f64>, timestamps: Vec<i64>) -> Self {
        let rows = data.len();
        let cols = 1;
        let mut headers = AtlasMap::new();
        headers.insert(name.to_string(), 0);
        Asset {
            timestamps,
            data,
            source: None,
            name: name.to_string(),
            datetime_format: "".to_string(),
            id: id,
            rows,
            cols,
            headers,
        }
    }

    pub(crate) fn from_csv(
        id: usize,
        datetime_format: &str,
        name: &str,
        path: &str,
    ) -> AtlasResult<Self> {
        let mut timestamps = Vec::new();
        let mut data = Vec::new();
        let mut reader = ReaderBuilder::new().has_headers(false).from_path(path)?;
        for result in reader.records().skip(1) {

            for &record[0], datetime_format)?);
            for field in record.iter().skip(1) {
                data.push(field.parse::<f64>()?);
            }
        }
        let rows = timestamps.len();
        let cols = data.len() / rows;
        Ok(Asset {
            timestamps,
            data,
            source: Some(path.to_string()),
            name: name.to_string(),
            id,
            rows,
            cols,
            datetime_format: datetime_format.to_string(),
            headers: reader
                .headers()
                .unwrap()
                .iter()
                .skip(1)
                .enumerate()
                .map(|(i, s)| (s.to_string(), i))
                .collect(),
        })
    }

    pub fn get(&self, row: usize, col: usize) -> f64 {
        self.data[row * self.cols + col]
    }

    pub fn get_data(&self) -> &Vec<f64> {
        &self.data
    }

    pub fn get_id(&self) -> usize {
        self.id
    }

    pub fn get_timestamps(&self) -> &Vec<i64> {
        &self.timestamps
    }

    pub fn get_timestamp(&self, row: usize) -> i64 {
        self.timestamps[row]
    }

    pub fn get_name(&self) -> &str {
        &self.name
    }

    pub fn get_rows(&self) -> usize {
        self.rows
    }

    pub fn get_cols(&self) -> usize {
        self.cols
    }

    pub fn get_headers(&self) -> &AtlasMap<String, usize> {
        &self.headers
    }

    pub fn set_headers(&mut self, headers: AtlasMap<String, usize>) {
        self.headers = headers;
    }

    pub fn set_source(&mut self, source: &str) {
        self.source = Some(source.to_string());
    }

    pub fn get_source(&self) -> Option<&str> {
        match &self.source {
            Some(s) => Some(s),
            None => None,
        }
    }
}
