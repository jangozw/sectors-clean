use anyhow::{Result, bail};
use clap::Parser;
use tracing::*;

#[derive(Debug, Parser)]
#[clap(name = "lotus-sector-clean", author = "The Aleo Team <hello@aleo.org>")]
pub struct CLI {
    #[clap(default_value = "root", long = "username")]
    pub username: String,

    #[clap(default_value = "1", long = "network")]
    pub network: u16,

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
            None => match self.network {
                1=> {
                    info!("start without subcommand,  network is {}", self.network);
                    Ok(())
                }
                _ => bail!("invalid network"),
            },
        }
    }
}


#[derive(Debug, Parser)]
pub enum Command {
    #[clap(name = "export", about = "Export expired data")]
    Export(Export),
    #[clap(name = "stat", about = "Show stat expired sectors info")]
    Stat(Stat),
    #[clap(name = "miner-info", about = "Show miners")]
    MinerInfo(MinerInfo),
    #[clap(name = "update-height", hide =true,  about = "update last export height")]
    Update(Update),

}

impl Command {
    pub fn parse(self) -> Result<String> {
        match self {
            Self::Export(command) => command.parse(),
            Self::Stat(command) => command.parse(),
            Self::MinerInfo(command) => command.parse(),
            Self::Update(command) => {command.parse()},
        }
    }
}



#[derive(Debug, Parser)]
pub struct Export {
    #[clap(long = "miner")]
    pub miner: Option<String>,
    #[clap(long = "all")]
    pub all : bool
}

impl Export {
    pub fn parse(self) -> Result<String> {
        info!("entry Export cmd");

        if self.all == false && self.miner.is_none() {
            bail!("invalid args, miner or all arg must be specified");
        }
        let mut export_miners = Vec::new();
        let cfg_miners = match  util::cfg::get_cfg_miners(){
            Ok(v) => { v}
            Err(e) => {bail!(e.to_string())}
        };
        if self.miner.is_some() {
            let miners = self.miner.unwrap();
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

        println!("export miners: {:?}", export_miners.clone());
        println!("doing next {}", export_miners.len());

        for m in export_miners {
            let begin_height = util::store::get_miner_export_height(&m.miner).expect("get height failed");
            let now_height = 2000000;
            let job = format!("lotus state sectors-exp --stat=true --epoch_begin={} --prefix=0 {} > {}_{}_expire_{}_{}.txt",
                            begin_height, m.miner, m.city, m.bucket, begin_height, now_height);
            println!("doing job: {}", job);
            let output = std::process::Command::new("").output().expect("cmd failed");
            let output = String::from_utf8(output.stdout).expect("from_utf8 failed");
            println!("{}", output);
        }
        Ok("".to_string())
    }
}


#[derive(Debug, Parser)]
pub struct MinerInfo {}
impl MinerInfo {
    pub fn parse(self) -> Result<String> {
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
        info!("start command Stat,  city: {:?} miner: {:?}", self.city, self.miner);
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
        let res = util::store::set_miner_export_height(&self.miner, self.height);
        match res {
            Ok(_) => {}
            Err(e) => {
                bail!("set_miner_export_height failed! {}", e.into_string())
            }
        }
        let height = util::store::get_miner_export_height(&self.miner).unwrap();
        Ok(format!("Ok! {} new height: {}", self.miner, height))
    }
}




