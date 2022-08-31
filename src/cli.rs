use anyhow::{Result};
use clap::Parser;
use tracing::*;
use crate::cli_handler::CLIHandler;

#[derive(Debug, Parser)]
#[clap(name = "lotus-sector-clean", author = "Django")]
pub struct CLI {
    /// Specify an optional subcommand.
    #[clap(subcommand)]
    commands: Option<Command>,
}

impl CLI {
    pub fn start(self) -> Result<()> {
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
        let handler = CLIHandler::new();
        handler.export(self.miner, self.all)
    }
}


#[derive(Debug, Parser)]
pub struct MinerInfo {}
impl MinerInfo {
    pub fn parse(self) -> Result<String> {
        let handler = CLIHandler::new();
        handler.get_miner_info()
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
        let handler = CLIHandler::new();
        handler.update_export_height(self.miner, self.height)
    }
}
