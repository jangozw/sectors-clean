use std::path::PathBuf;

use dirs::home_dir;
use rocksdb::DB;

// use anyhow::{Result, bail};
const DB_NAME: &str = ".lotus-sector-clean-db1";

pub fn get_db_dir() -> PathBuf {
    let mut path = home_dir().expect("couldn't get home dir in your system!");
    path.push(DB_NAME);
    path
}

pub fn test_db() {
    // NB: db is automatically closed at end of lifetime
    let path = get_db_dir();
    let db = DB::open_default(path).expect("open db failed");
    db.put(b"yourname", b"Django").expect("db put value failed");
    println!("db put value yourname:Django");
    match db.get(b"yourname") {
        Ok(Some(value)) => println!("db get value yourname: {}", String::from_utf8(value).unwrap()),
        Ok(None) => println!("db get value not found"),
        Err(e) => println!("db get value err operational problem encountered: {}", e),
    }
    // db.delete(b"my key").unwrap();

    // let _ = DB::destroy(&Options::default(), path);
}


pub fn set_miner_export_height(miner: &str, height: u64) -> Result<(), rocksdb::Error> {
    let db = DB::open_default(get_db_dir())?;
    let key = format!("miner_export_height_{}", miner);
    db.put(key, height.to_string())
}

pub fn get_miner_export_height(miner: &str) -> Result<u64, rocksdb::Error> {
    let db = DB::open_default(get_db_dir())?;
    let key = format!("miner_export_height_{}", miner);
    let height = db.get(key).expect("db get key failed");
    let height: u64 = match height {
        None => { 0 }
        Some(value) => {
            let value = String::from_utf8(value).expect("to string err");
            let height: u64 = str::parse::<u64>(&*value).expect("failed");
            height
        }
    };
    Ok(height)
}


#[test]
fn test_set_height() {
    set_miner_export_height("f0100", 145).expect("set height ok");
    let height = get_miner_export_height("f0100").expect("get height error");
    println!("get Height: {}", height);
}
