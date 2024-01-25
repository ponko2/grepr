use anyhow::Result;
use clap::Parser;

#[derive(Debug, Parser)]
#[command(version, about, long_about = None)]
pub struct Args {}

pub fn get_args() -> Result<Args> {
    Ok(Args::parse())
}

pub fn run(args: Args) -> Result<()> {
    dbg!(args);
    Ok(())
}
