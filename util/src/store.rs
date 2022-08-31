use std::path::PathBuf;

use anyhow::{Error, Result};
use rocksdb::{DB, DBWithThreadMode, SingleThreaded};

const DB_NAME: &str = ".lotus-sector-clean-db1";

pub fn get_db_dir() -> Result<PathBuf> {
    if let Some(mut path) = dirs::home_dir() {
        path.push(DB_NAME);
        return Ok(path);
    }
    // Err(anyhow::anyhow!("wrong {}", 1))
    Err(Error::msg("could not get home dir"))
}

pub fn init_db() -> Result<DBWithThreadMode<SingleThreaded>> {
    let db = DB::open_default(get_db_dir()?)?;
    Ok(db)
}

pub fn set_miner_export_height(db: &DBWithThreadMode<SingleThreaded>, miner: &str, height: u64) -> Result<()> {
    let key = format!("miner_export_height_{}", miner);
    db.put(key, height.to_string())?;
    Ok(())
}

pub fn get_miner_export_height(db: &DBWithThreadMode<SingleThreaded>, miner: &str) -> Result<u64> {
    let key = format!("miner_export_height_{}", miner);
    let height = db.get(key)?;
    let height: u64 = match height {
        None => { 0 }
        Some(value) => {
            let value = String::from_utf8(value)?;
            let height: u64 = str::parse::<u64>(&*value)?;
            height
        }
    };
    Ok(height)
}


/*pub fn test_db() {
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
*/