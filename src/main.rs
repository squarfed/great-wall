use anyhow::Result;
use argon2::{Argon2, ParamsBuilder};
use clap::Parser;
use hotwatch::{Event, EventKind, Hotwatch};
use std::env;
use std::io::Write;
use std::process::Command;
use std::time::Duration;
use std::{fs::File, path::Path, path::PathBuf};

use indicatif::{ProgressBar, ProgressStyle};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    /// Wallet seed value
    seed: String,
}
fn xaos_config_path() -> PathBuf {
    let home_dir = env::home_dir().expect("home directory not set");
    Path::new(&home_dir).join(".XaoSrc")
}

fn write_xaos_config(
    xaos_config_path: &PathBuf,
    x: f32,
    y: f32,
    sx: f32,
    sy: f32,
) -> Result<(), std::io::Error> {
    let mut xaos_config: File = File::create(&xaos_config_path).unwrap();
    write!(
        xaos_config,
        "(initstate)
(defaultpalette 0)
(formula 'mandel)
(letterspersec 15)
(cyclingspeed 30)
(maxiter 1000)
(view {} {} {} {} )


(usleep 0)
(letterspersec 15)",
        x, y, sx, sy
    )
}

fn main() -> Result<()> {
    let cli = Cli::parse();
    let iteration_number = 50;
    let seed = cli.seed;
    println!("Value for seed: {seed}");
    let password: &[u8] = seed.as_bytes();
    let salt: &[u8; 12] = b"example salt"; // Todo: ?
    let mut output_key_material: [u8; 32] = [0u8; 32]; // Can be any desired size
    let pb = ProgressBar::new_spinner();
    pb.enable_steady_tick(Duration::from_millis(120));
    pb.set_style(
        ProgressStyle::with_template("{spinner:.blue} {msg}")
            .unwrap()
            .tick_strings(&[
                "▹▹▹▹▹",
                "▸▹▹▹▹",
                "▹▸▹▹▹",
                "▹▹▸▹▹",
                "▹▹▹▸▹",
                "▹▹▹▹▸",
                "▪▪▪▪▪",
            ]),
    );
    pb.set_message("hashing...");
    let params = ParamsBuilder::new()
        .t_cost(iteration_number)
        .build()
        .unwrap();
    Argon2::from(params)
        .hash_password_into(password, salt, &mut output_key_material)
        .expect("Failed to hash passord");
    pb.finish_with_message("done");
    println!("{:#?}", output_key_material);
    let xaos_config_path = xaos_config_path();
    let mut hotwatch: Hotwatch = Hotwatch::new().expect("hotwatch failed to initialize!");
    let _ = write_xaos_config(&xaos_config_path, -1.0, -0.3, 0.8, 0.8)?;
    hotwatch
        .watch(&xaos_config_path, |event: Event| {
            if let EventKind::Modify(_) = event.kind {
                println!("Configuration has changed.");
            }
        })
        .expect("failed to watch file!");
    Command::new("xaos").output()?; // .arg("...").arg("..").output()?;
    Ok(())
}
