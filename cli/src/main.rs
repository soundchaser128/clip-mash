use clap::{Parser, Subcommand};

#[derive(Debug, Subcommand)]
enum TopLevelCommand {
    Video,
    Marker,
}

#[derive(Parser, Debug)]
struct Args {
    #[clap(subcommand)]
    command: TopLevelCommand,
}

fn main() {
    let args = Args::parse();
    println!("{:?}", args);
}
