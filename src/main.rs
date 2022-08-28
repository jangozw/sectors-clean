use clap::Parser;

use cli::CLI;

mod cli;

fn main() {
    let app = CLI::parse();
    let res = app.start();
    match res {
        Ok(_) => {
            // println!("process exit with output: {:?}", value)
        }
        Err(e) => {
            println!("process exit with err: {:?}", e)
        }
    }
}
