use anyhow::{anyhow, Result};
use clap::Parser;
use regex::{Regex, RegexBuilder};

#[derive(Debug, Parser)]
#[command(version, about, long_about = None)]
pub struct Args {
    #[arg(help = "Search pattern")]
    pattern: String,

    #[arg(value_name = "FILE", help = "Input file(s)", default_value = "-")]
    files: Vec<String>,

    #[arg(short, long, help = "Case insensitive")]
    insensitive: bool,

    #[arg(short, long, help = "Recursive search")]
    recursive: bool,

    #[arg(short, long, help = "Count occurrences")]
    count: bool,

    #[arg(short = 'v', long, help = "Invert match")]
    invert_match: bool,
}

#[derive(Debug)]
pub struct Config {
    pattern: Regex,
    files: Vec<String>,
    recursive: bool,
    count: bool,
    invert_match: bool,
}

pub fn get_args() -> Result<Config> {
    let args = Args::parse();
    let pattern = &args.pattern;
    let pattern = RegexBuilder::new(pattern)
        .case_insensitive(args.insensitive)
        .build()
        .map_err(|_| anyhow!("Invalid pattern \"{pattern}\""))?;
    Ok(Config {
        pattern,
        files: args.files,
        recursive: args.recursive,
        count: args.count,
        invert_match: args.invert_match,
    })
}

pub fn run(config: Config) -> Result<()> {
    dbg!(config);
    Ok(())
}
