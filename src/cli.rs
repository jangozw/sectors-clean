use std::collections::HashMap;
use std::path::PathBuf;
use std::time::Instant;

use anyhow::{bail, Result};
use chrono::prelude::*;
use clap::Parser;
use regex::Regex;
use tracing::*;

use util::cfg::get_cfg_miners;

#[derive(Debug, Parser)]
#[clap(name = "lotus-sector-clean", author = "The Aleo Team <hello@aleo.org>")]
pub struct CLI {
    /// Specify an optional subcommand.
    #[clap(subcommand)]
    commands: Option<Command>,
}

impl CLI {
    /// Starts the node.
    pub fn start(self) -> Result<()> {

        // Parse optional subcommands first.
        match self.commands {
            Some(command) => {
                println!("{}", command.parse()?);
                Ok(())
            }
            _ => {
                Ok(())
            }
        }
    }
}


#[derive(Debug, Parser)]
pub enum Command {
    #[clap(name = "export", about = "Export expired data")]
    Export(Export),
    #[clap(name = "stat", hide = true, about = "Show stat expired sectors info")]
    Stat(Stat),
    #[clap(name = "miner-info", about = "Show miners")]
    MinerInfo(MinerInfo),
    #[clap(name = "update", hide = false, about = "update last export height")]
    Update(Update),

}

impl Command {
    pub fn parse(self) -> Result<String> {
        match self {
            Self::Export(command) => command.parse(),
            Self::Stat(command) => command.parse(),
            Self::MinerInfo(command) => command.parse(),
            Self::Update(command) => { command.parse() }
        }
    }
}


#[derive(Debug, Parser)]
pub struct Export {
    #[clap(long = "miner")]
    pub miner: Option<String>,
    #[clap(long = "all")]
    pub all: bool,
}

impl Export {
    pub fn parse(self) -> Result<String> {
        if self.all == false && self.miner.is_none() {
            bail!("invalid args, miner or all arg must be specified");
        }
        let mut export_miners = Vec::new();

        let cfg_miners = util::cfg::get_cfg_miners()?;

        // if self.miner.is_some() {
        if let Some(miners) = self.miner {
            // let miners = self.miner.unwrap();
            let miners: Vec<String> = miners.split(",").map(|s| s.to_string()).collect();
            for mid in miners {
                let mut has = false;
                for cm in &cfg_miners {
                    if mid == cm.miner {
                        has = true;
                        export_miners.push(cm.clone());
                        break;
                    }
                }
                if !has {
                    bail!("arg miner {} not in cfg miners", mid);
                }
            }
        } else if self.all == true {
            export_miners = cfg_miners;
        }
        let export_dir = get_export_dir()?;
        util::file::check_create_path(export_dir.clone());
        let now_height = util::lotus::mainnet_height_now();
        let mut total_exp :f64 = 0.0;
        let start = Instant::now();
        for m in &export_miners {
            let begin_height = util::store::get_miner_export_height(&m.miner)?;
            let export_file = format!("{}/{}_{}_expire_{}_{}.txt", export_dir.to_str().unwrap(), m.city, m.bucket, begin_height, now_height);
            let job = format!("lotus state sectors-exp --stat=true --epoch_begin={} --prefix=0 {} > {}",
                              begin_height, m.miner, export_file);
            info!("doing {}: {}", m.miner, job);
            // let output = std::process::Command::new("sh").arg("-c").arg("ls -G -alF > a.txt").output().expect("sh exec error!");
            let output = std::process::Command::new("sh").arg("-c").arg(job).output()?;
            let _ = String::from_utf8(output.stdout)?;
            // check
            let ck_arg = format!("head -n 14 {}", export_file);
            let ck_output = std::process::Command::new("sh").arg("-c").arg(ck_arg).output()?;
            let ck_output = String::from_utf8(ck_output.stdout)?;
            let (ok, exp_power) = match_exp_power(&ck_output)?;
            if ok {
                total_exp += exp_power;
                util::store::set_miner_export_height(&m.miner, now_height as u64)?;
                info!("done! {} exp power: {}P,  updated new height: {} cost: {:?}", m.miner, exp_power, now_height, start.elapsed());
                if exp_power > 0.0 {
                    let arg = format!("sed -i '1,14d' {}", export_file);
                    std::process::Command::new("sh").arg("-c").arg(arg).output()?;
                } else if exp_power == 0.0 {
                    let arg = format!("rm -rf {}", export_file);
                    std::process::Command::new("sh").arg("-c").arg(arg).output()?;
                }
            } else {
                error!("error for exec");
            }
        }
        info!("total exp-power: {}P cost: {:?}", total_exp, start.elapsed());
        Ok("".to_string())
    }
}


#[derive(Debug, Parser)]
pub struct MinerInfo {}

impl MinerInfo {
    pub fn parse(self) -> Result<String> {
        println!("------------------------setting-info--------------------------------");
        println!("export dir: {:?}", get_export_dir());
        println!("store dir: {:?}", util::store::get_db_dir());
        println!("-------------------------miner info---------------------------------");
        let list = get_cfg_miners().expect("get cfg miners err");
        for m in list.iter(){
            print!("{},", m.miner);
        }
        println!("\n");
        println!("{:<10} {:<10} {:<10} {:<12} {:<20}", "miner", "city", "bucket", "exportHeight", "exportTime");
        for m in list.iter(){
            let height = util::store::get_miner_export_height(&m.miner).expect("get height failed");
            let export_time = util::lotus::mainnet_height_to_datetime(height as i64);
            println!("{:<10} {:<10} {:<10} {:<12} {:<20}", m.miner, m.city, m.bucket, height, export_time);
        }
        Ok("".to_string())
    }
}


#[derive(Debug, Parser)]
pub struct Stat {
    #[clap(long = "miner")]
    pub miner: Option<String>,
    #[clap(long = "city")]
    pub city: Option<String>,

}

impl Stat {
    pub fn parse(self) -> Result<String> {
        debug!("start command Stat,  city: {:?} miner: {:?}", self.city, self.miner);
        Ok("".to_string())
    }
}

#[derive(Debug, Parser)]
pub struct Update {
    #[clap(long = "miner")]
    pub miner: String,
    #[clap(long = "height")]
    pub height: u64,
}

impl Update {
    pub fn parse(self) -> Result<String> {
        let cfg_miners = get_cfg_miners()?;
        let mut mp = HashMap::new();
        for m in &cfg_miners {
            mp.insert(m.miner.clone(), "");
        }
        let arg_miners: Vec<String> = self.miner.split(",").map(|s| s.to_string()).collect();
        for mid in &arg_miners {
            if ! mp.contains_key(mid) {
                bail!("miner {} not exists in cfg", mid);
            }
        }
        for mid in &arg_miners {
            let _ = util::store::set_miner_export_height(mid, self.height)?;
            let height = util::store::get_miner_export_height(mid)?;
            info!("update export height ok! {} now height: {}", mid, height);
        }
        Ok("".to_string())
    }
}


//################################################
fn get_export_dir() -> Result<PathBuf> {
    if let Some(mut path) = dirs::home_dir() {
        path.push("expired_sectors");
        let now = Local::now();
        let name = format!("{}{}{}", now.year(), now.month(), now.day());
        path.push(name);
        return Ok(path);
    }
    Err(anyhow::Error::msg("could not get export dir"))
}

fn match_exp_power(text: &str) -> Result<(bool, f64)> {
    let reg = Regex::new(r"expired sectors: \d+ power: (\d+\.\d+)P").unwrap();
    let mut power: Option<String> = None;
    let mut ok: bool = false;
    for cap in reg.captures_iter(text) {
        let a = cap.get(1).unwrap().as_str();
        ok = true;
        power = Some(a.to_string());
        break;
    }
    if power.is_none() {
        return Err(anyhow::Error::msg("could not get exp power by output"));
    }
    let power = power.unwrap();
    let exp_power: f64 = power.parse::<f64>().unwrap();
    Ok((ok, exp_power))
}

#[test]
fn get_power(){
    let str = "load tipset cost: 3.177576ms
StateMinerActiveSectors cost: 316.428474ms
StateMinerSectors cost: 1.952006844s
sort all sectors cost: 5.350952ms
calc expired cost: 4.525ms
miner: f089380
sector size: 32GiB
now height: 2103484
total sectors: 60724 power: 1.8531p
active sectors: 8371 power: 0.2555p
expired sectors: 49948 power: 1.5243P begin epoch:0
expired max expiration sector: 78392 , expiation: 2100116
before output cost: 2.281666128s";

    match match_exp_power(str) {
        Ok((ok, power)) => {
            println!("ok: {} power: {}", ok, power);
        }
        Err(e) => { println!("error: {}", e) }
    }
}