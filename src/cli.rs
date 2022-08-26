use anyhow::{Result, bail};
use clap::Parser;
use crate::cfg;
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
    #[clap(name = "stat", about = "Show stat expired sectors info")]
    Stat(Stat),
    #[clap(name = "miners", about = "Show miners")]
    Miners(Miners),

}

impl Command {
    pub fn parse(self) -> Result<String> {
        match self {
            Self::Stat(command) => command.parse(),
            Self::Miners(command) => command.parse(),
        }
    }
}

#[derive(Debug, Parser)]
pub struct Miners {}
impl Miners {
    pub fn parse(self) -> Result<String> {
        cfg::get_miners();
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





