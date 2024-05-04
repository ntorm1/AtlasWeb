use std::{fs::read_dir, path::Path};

use serde::Serialize;

use crate::{assumption_error, standard::*};

use super::Asset;

#[derive(Serialize, Clone)]
pub struct Exchange {
    name: String,

    source: String,

    datetime_format: String,

    asset_id_map: AtlasMap<String, usize>,

    headers: AtlasMap<String, usize>,

    #[serde(skip_serializing)]
    assets: Vec<Asset>,

    timestamps: Vec<i64>,

    current_timestamp: i64,

    #[serde(skip_serializing)]
    data: DMatrix<f64>,

    #[serde(skip_serializing)]
    returns: DMatrix<f64>,

    #[serde(skip_serializing)]
    returns_scalar: DVector<f64>,

    #[serde(skip_serializing)]
    col_count: usize,

    #[serde(skip_serializing)]
    close_index: usize,

    current_index: usize,
}

impl Exchange {
    pub fn new(name: &str, source: &str, datetime_format: &str) -> AtlasResult<Exchange> {
        let mut exchange = Exchange {
            datetime_format: datetime_format.to_string(),
            name: name.to_string(),
            source: source.to_string(),
            asset_id_map: AtlasMap::new(),
            headers: AtlasMap::new(),
            assets: Vec::new(),
            timestamps: Vec::new(),
            current_timestamp: 0,
            data: DMatrix::zeros(0, 0),
            returns: DMatrix::zeros(0, 0),
            returns_scalar: DVector::zeros(0),
            col_count: 0,
            close_index: 0,
            current_index: 0,
        };
        exchange.init()?;
        exchange.validate()?;
        exchange.build()?;
        Ok(exchange)
    }

    fn init(&mut self) -> AtlasResult<()> {
        let path = Path::new(&self.source);
        if !path.exists() {
            return assumption_error!("exchange source does not exist: {}", self.source);
        }
        if !path.is_dir() {
            return assumption_error!("exchange source is not a directory");
        }
        let files = read_dir(path)?;
        for file in files {
            let file = file?;
            let stem = file.file_name().to_string_lossy().to_string();
            let stem = stem.split('.').collect::<Vec<&str>>()[0];
            let asset = Asset::from_csv(
                self.assets.len(),
                &self.datetime_format,
                &stem,
                &file.path().to_string_lossy(),
            )?;
            self.asset_id_map
                .insert(asset.get_name().to_string(), asset.get_id());
            if self.headers.is_empty() {
                self.headers = asset.get_headers().clone();
            }
            self.assets.push(asset);
        }
        if self.assets.len() == 0 {
            return assumption_error!("no assets found in exchange");
        }
        Ok(())
    }

    fn  validate(&mut self) -> AtlasResult<()> {
        for asset in self.assets.iter() {
            // validate all assets have the same headers
            let asset_headers = asset.get_headers();
            if asset.get_id() == 0 {
                let mut found_close = false;
                for (header, index) in asset_headers.iter() {
                    self.headers.insert(header.to_string(), *index);
                    let header_lower = header.to_lowercase();
                    if header_lower == "close" {
                        self.close_index = *index;
                        found_close = true;
                    }
                }
                if !found_close {
                    return assumption_error!("close header not found in exchange");
                }
                self.col_count = self.headers.len();
            } else {
                for (header, _) in asset_headers.iter() {
                    if !self.headers.contains_key(header) {
                        return assumption_error!("asset header not found in exchange");
                    }
                    if self.headers.get(header).unwrap() != asset_headers.get(header).unwrap() {
                        return assumption_error!("asset header index does not match exchange");
                    }
                    if !self.headers.len() == asset_headers.len() {
                        return assumption_error!("asset header count does not match exchange");
                    }
                }
            }
            //validate asset timestamps are in ascending order
            let asset_timestamps = asset.get_timestamps();
            for i in 1..asset_timestamps.len() {
                if asset_timestamps[i] <= asset_timestamps[i - 1] {
                    return assumption_error!("asset timestamps are not in ascending order");
                }
            }
            self.timestamps = sorted_nion(&self.timestamps, &asset_timestamps);
        }
        // validate each asset's timestamps are a contiguous subset of the exchange timestamps
        for asset in self.assets.iter() {
            if !is_continuous_subset(&self.timestamps, &asset.get_timestamps()) {
                return assumption_error!(
                    "asset timestamps are not a contiguous subset of exchange timestamps"
                );
            }
        }
        Ok(())
    }

    fn build(&mut self) -> Result<(), AtlasError> {
        self.data = DMatrix::zeros(self.assets.len(), self.col_count * self.timestamps.len());
        self.returns = DMatrix::zeros(self.assets.len(), self.timestamps.len());
        self.returns_scalar = DVector::zeros(self.assets.len());
        for asset in self.assets.iter() {
            let mut asset_index = 0;
            let asset_id = asset.get_id();
            let asset_datetime_index = asset.get_timestamps();
            let asset_data = asset.get_data();
            for exchange_index in 0..self.timestamps.len() {
                let exchange_datetime = self.timestamps[exchange_index];
                let mut asset_datetime = 0;
                if asset_index < asset_datetime_index.len() {
                    asset_datetime = asset_datetime_index[asset_index];
                }
                // asset datetime is out of bounds or does not match exchange datetime
                if asset_datetime == 0 || exchange_datetime != asset_datetime {
                    // fill data matrix with NAN
                    for i in 0..self.headers.len() {
                        let data_index = exchange_index * self.col_count + i;
                        self.data[(asset_id, data_index)] = f64::NAN;
                    }
                    self.returns[(asset_id, exchange_index)] = 0.0;
                } else {
                    // copy asset data into exchange data matrix
                    for i in 0..self.headers.len() {
                        let asset_data_index = asset_index * self.col_count + i;
                        let value = asset_data[asset_data_index];
                        let data_index = exchange_index * self.col_count + i;
                        self.data[(asset_id, data_index)] = value;
                    }
                    // calculate returns
                    if asset_index == 0 {
                        self.returns[(asset_id, exchange_index)] = 0.0;
                    } else {
                        let prev_close = self.data[(
                            asset_id,
                            (exchange_index - 1) * self.col_count + self.close_index,
                        )];
                        let curr_close = self.data
                            [(asset_id, exchange_index * self.col_count + self.close_index)];
                        let mut ret = (curr_close - prev_close) / prev_close;
                        if ret.is_nan() {
                            ret = 0.0;
                        }
                        self.returns[(asset_id, exchange_index)] = ret;
                    }
                    asset_index += 1;
                }
            }
        }
        //self.assets.clear();
        Ok(())
    }

    pub fn get_timestamps(&self) -> &Vec<i64> {
        &self.timestamps
    }

    pub fn get_asset(&self, name: &str) -> Option<&Asset> {
        match self.asset_id_map.get(name) {
            Some(id) => Some(&self.assets[*id]),
            None => None,
        }
    }
}

mod exchange_tests {

    use super::*;

    use crate::handle_result;

    #[test]
    fn test_exchange_new() {
        let exchange_result = Exchange::new(EXCHANGE1_NAME, EXCHANGE1_PATH, DATETIME_FORMAT);
        handle_result!(exchange_result, "Failed to create Exchange");
        let exchange = exchange_result.unwrap();
        let timestamps = exchange.get_timestamps();
        assert!(timestamps.len() == 6);
    }
}
