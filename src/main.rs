// #86442 <https://github.com/rust-lang/rust/issues/86442>
#![feature(io_error_more)]

use clap::{Parser, Subcommand};
use std::io::{Read, Write};

#[derive(Parser)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Cat { files: Option<Vec<String>> },
}

fn main() {
    let cli = Cli::parse();

    match &cli.command {
        Commands::Cat { files } => {
            if let Some(files) = files {
                // stdout のロックを取得
                let stdout = std::io::stdout();
                let mut stdout = stdout.lock();

                // files を入力順に stdout に出力する
                for file in files {
                    let content = match get_file_data(file) {
                        Ok(content) => content,
                        Err(e) => match e.kind() {
                            std::io::ErrorKind::NotFound => {
                                format!("ztb cat: {}: No such file or directory\n", file)
                            }
                            std::io::ErrorKind::IsADirectory => {
                                format!("ztb cat: {}: Is a directory\n", file)
                            }
                            _ => format!("ztb cat: {}: {}", file, e),
                        },
                    };

                    write!(&mut stdout, "{}", content).unwrap();
                }
            } else {
                // files の指定がない場合は標準入力から読み取る
                let mut buffer = String::new();
                let stdin = std::io::stdin();
                let mut stdin = stdin.lock();
                stdin.read_to_string(&mut buffer).unwrap();

                let stdout = std::io::stdout();
                let mut stdout = stdout.lock();
                write!(&mut stdout, "{}", buffer).unwrap();
            }
        }
    }
}

/// 与えたら path のファイル内容を返します。
fn get_file_data(path: &str) -> std::io::Result<String> {
    std::fs::read_to_string(path)
}
