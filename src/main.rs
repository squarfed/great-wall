use anyhow::Result;
use argon2::Argon2;
use clap::Parser;
use hotwatch::{Event, EventKind, Hotwatch};
use std::process::Command;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    /// Wallet seed value
    seed: String,
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    let seed = cli.seed;
    println!("Value for seed: {seed}");

    let password: &[u8] = seed.as_bytes();
    let salt: &[u8; 12] = b"example salt"; // Todo: ?

    let mut output_key_material: [u8; 32] = [0u8; 32]; // Can be any desired size
    Argon2::default()
        .hash_password_into(password, salt, &mut output_key_material)
        .expect("Failed to hash passord");
    println!("{:#?}", output_key_material);
    let mut hotwatch: Hotwatch = Hotwatch::new().expect("hotwatch failed to initialize!");
    hotwatch
        .watch("~/.XaoSrc", |event: Event| {
            if let EventKind::Modify(_) = event.kind {
                println!("Configuration has changed.");
            }
        })
        .expect("failed to watch file!");
    Command::new("xaos").output()?; // .arg("...").arg("..").output()?;
    Ok(())
}
