use std::fs::File;
use serde_derive::{Serialize, Deserialize};
use std::fmt::Error;

const MINER_JSON:&str = "miners.json";

#[derive(Debug,Clone,Serialize, Deserialize)]
pub struct Miner {
    pub miner: String,
    pub city: String,
    pub bucket: String,
}

pub fn get_cfg_miners()->Result<Vec<Miner>, Error>{
    let f = File::open(MINER_JSON).expect("open miners.json failed");
    let miners: Vec<Miner> = serde_json::from_reader(f).expect("read json file err");
    Ok(miners)
}

