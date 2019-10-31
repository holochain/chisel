#[macro_use]
extern crate structopt;
extern crate regex;

mod tryorama;

use std::io::{self};
use structopt::StructOpt;
use tryorama::TryoramaCmd;

#[derive(Debug, StructOpt)]
#[structopt(
    name = "chisel",
    about = "Stream-based CLI tool for parsing Holochain logs"
)]
struct Opt {
    #[structopt(subcommand)]
    cmd: Option<Subcommand>,
}

#[derive(Debug, StructOpt)]
enum Subcommand {
    Tryorama(tryorama::TryoramaCmd),
}

fn main() -> Result<(), String> {
    let opt = Opt::from_args();
    let stdin = io::stdin();
    let stdin = stdin.lock();

    let result = if let Some(Subcommand::Tryorama(cmd)) = opt.cmd {
        match cmd {
            TryoramaCmd::Demux(demux) => demux.run(stdin),
        }
    } else {
        Ok(())
    };
    match result {
        Err(err) => {
            eprintln!("{}", err);
            std::process::exit(1);
        }
        Ok(()) => Ok(()),
    }
}
