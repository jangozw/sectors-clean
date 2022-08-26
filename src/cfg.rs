use std::fs::File;
use serde_derive::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Miner {
    pub miner: String,
    pub city: String,
    pub bucket: String,
}


pub fn get_miners(){
    println!("current: {:?}", std::env::current_dir());
    let f = File::open("miners.json").expect("could not open miners.json");
    let miners: Vec<Miner> = serde_json::from_reader(f).expect("could not parse miner by json file");

    println!("miners: {:?}", miners);



    println!("get miners");
}

