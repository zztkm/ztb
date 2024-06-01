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
                // 最後のファイルは改行をしない
                let mut iter = files.iter().peekable();
                while let Some(file) = iter.next() {
                    let content = std::fs::read_to_string(file).unwrap();
                    if iter.peek().is_none() {
                        write!(&mut stdout, "{}", content).unwrap();
                    } else {
                        writeln!(&mut stdout, "{}", content).unwrap();
                    }
                }
            } else {
                // files の指定がない場合は標準入力から読み取る
                let mut buffer = String::new();
                let stdin = std::io::stdin();
                let mut stdin = stdin.lock();
                stdin.read_to_string(&mut buffer).unwrap();

                // 標準出力に書き込む
                let stdout = std::io::stdout();
                let mut stdout = stdout.lock();
                write!(&mut stdout, "{}", buffer).unwrap();
            }
        }
    }
}
