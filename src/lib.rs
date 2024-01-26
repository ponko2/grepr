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
    println!("pattern \"{}\"", config.pattern);

    let entries = find_files(&config.files, config.recursive);
    for entry in entries {
        match entry {
            Err(err) => eprintln!("{err}"),
            Ok(filename) => println!("file \"{filename}\""),
        }
    }
    Ok(())
}

fn find_files(paths: &[String], recursive: bool) -> Vec<Result<String>> {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::find_files;
    use rand::{distributions::Alphanumeric, Rng};

    #[test]
    fn test_find_files() {
        // 存在することがわかっているファイルを見つけられることを確認する
        let files = find_files(&["./tests/inputs/fox.txt".to_string()], false);
        assert_eq!(files.len(), 1);
        assert_eq!(files[0].as_ref().unwrap(), "./tests/inputs/fox.txt");

        // recursiveなしの場合、ディレクトリを拒否する
        let files = find_files(&["./tests/inputs".to_string()], false);
        assert_eq!(files.len(), 1);
        if let Err(e) = &files[0] {
            assert_eq!(e.to_string(), "./tests/inputs is a directory");
        }

        // ディレクトリ内の4つのファイルを再帰的に検索できることを確認する
        let res = find_files(&["./tests/inputs".to_string()], true);
        let mut files: Vec<String> = res
            .iter()
            .map(|r| r.as_ref().unwrap().replace('\\', "/"))
            .collect();
        files.sort();
        assert_eq!(files.len(), 4);
        assert_eq!(
            files,
            vec![
                "./tests/inputs/bustle.txt",
                "./tests/inputs/empty.txt",
                "./tests/inputs/fox.txt",
                "./tests/inputs/nobody.txt",
            ]
        );

        // 存在しないファイルを表すランダムな文字列を生成する
        let bad: String = rand::thread_rng()
            .sample_iter(&Alphanumeric)
            .take(7)
            .map(char::from)
            .collect();

        // エラーとして不正なファイルを返すことを確認する
        let files = find_files(&[bad], false);
        assert_eq!(files.len(), 1);
        assert!(files[0].is_err());
    }
}
