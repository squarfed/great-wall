use clap::{Parser};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    /// Wallet seed value
    seed: String,

}


fn main() {
    let cli = Cli::parse();

    if let seed = cli.seed {
        println!("Value for seed: {seed}");
    }
}
