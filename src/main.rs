use clap::{Command, Arg};

fn main() {
    let matches = Command::new("Great Wall")
    .version("0.0.1")
    .author("Federico Squartini <squarfed@bigfi.re>")
    .about("Great Wall bip39 fractal store")
    .arg(
        Arg::new("seed")
            .num_args(1)
            .value_name("SEED")
            .help("Input seed")
            .required(true)
    ).get_matches();

println!("{:#?}", matches);

}
