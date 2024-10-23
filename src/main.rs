use std::path::PathBuf;

use clap::Parser;

#[derive(Parser)]
#[command(name = "TrainingMode-More Tool Kit")]
#[command(version = "0.1")]
#[command(about = "Build tool for TrainingMode-More", long_about = None)]
struct Cli {
    #[arg(short = 'l', value_name = "FILE", help = "Link file")]
    link: PathBuf,

    #[arg(short = 't', value_name = "FILE", help = "Symbol table file")]
    symbol_table: PathBuf,

    #[arg(short = 'd', value_name = "FILE", help = ".dat file")]
    dat: PathBuf,

    #[arg(short = 's', value_name = "SYMBOL", help = "Symbol name")]
    symbol: String,

    files: Vec<PathBuf>,

    #[arg(last = true)]
    gcc_flags: Vec<String>,
}

fn main() {
    let args = Cli::parse();

    println!("link: {:?}", args.link);
    println!("symbol_table: {:?}", args.symbol_table);
    println!("dat: {:?}", args.dat);
    println!("symbol: {:?}", args.symbol);
    println!("files: {:?}", args.files);
    println!("gcc flags: {:?}", args.gcc_flags);
}
