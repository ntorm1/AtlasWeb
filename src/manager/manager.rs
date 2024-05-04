use std::sync::{Arc, RwLock};

use crate::{
    assumption_error,
    exchange::{Asset, Exchange},
    standard::*,
};

pub struct AtlasManager {
    exchanges: AtlasMap<String, Arc<RwLock<Exchange>>>,
}

impl AtlasManager {
    pub fn new() -> AtlasResult<Self> {
        Ok(AtlasManager {
            exchanges: AtlasMap::new(),
        })
    }

    pub fn add_exchange(
        &mut self,
        name: &str,
        source: &str,
        datetime_format: &str,
    ) -> AtlasResult<()> {
        let exchange = Arc::new(RwLock::new(Exchange::new(name, source, datetime_format)?));
        self.exchanges.insert(name.to_string(), exchange);
        Ok(())
    }

    pub fn get_exchange(&self, name: &str) -> AtlasResult<Arc<RwLock<Exchange>>> {
        match self.exchanges.get(name) {
            Some(exchange) => Ok(exchange.clone()),
            None => assumption_error!("exchange not found: {}", name),
        }
    }

    pub fn get_exchange_ids(&self) -> Vec<String> {
        self.exchanges.keys().cloned().collect()
    }
}
