use argon2::{Argon2, Error};
use clap::Parser;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    /// Wallet seed value
    seed: String,
}

fn main() -> Result<(), Error> {
    let cli = Cli::parse();

    let seed = cli.seed;
    println!("Value for seed: {seed}");

    let password: &[u8] = seed.as_bytes();
    let salt: &[u8; 12] = b"example salt"; // Todo: ?

    let mut output_key_material: [u8; 32] = [0u8; 32]; // Can be any desired size
    Argon2::default().hash_password_into(password, salt, &mut output_key_material)?;
    println!("{:#?}", output_key_material);
    Ok(())
}
