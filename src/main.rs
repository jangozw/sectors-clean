use clap::Parser;
use cli::CLI;
use tracing::*;
mod cli;
mod cli_handler;

fn main() {
    util::initialize_logger(3);
    let app = CLI::parse();
    match app.start() {
        Ok(_) => {
            // info!("process end")
        }
        Err(e) => {
            error!("process exit with err: {:?}", e)
        }
    }
}
