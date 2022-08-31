use std::fs::File;
use anyhow::{Result};
use serde_derive::{Deserialize, Serialize};

const MINER_JSON: &str = "miners.json";

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Miner {
    pub miner: String,
    pub city: String,
    pub bucket: String,
}

pub fn get_cfg_miners() -> Result<Vec<Miner>> {
    let f = File::open(MINER_JSON)?;
    let miners: Vec<Miner> = serde_json::from_reader(f)?;
    Ok(miners)
}

