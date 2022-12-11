use clap::Parser;
use rand::distributions::Alphanumeric;
use rand::Rng;
use whittaker_map_generator::generator::Generator;
use whittaker_map_generator::generator::Config::{
    Debugging,
    Steppers,
    Steps,
};

#[derive(Parser, Debug)]
struct GeneratorArgs {
    #[arg(long, default_value_t = String::new())]
    seed: String,

    #[arg(long, default_value_t = 300)]
    size: u32,

    #[arg(long, default_value_t = 2)]
    rivers: u32,

    #[arg(long, default_value_t = false)]
    debug: bool,

    #[arg(long, default_value_t = 350)]
    steppers: u32,

    #[arg(long, default_value_t = 300)]
    steps: u32,

    #[arg(long, default_value_t = 4)]
    output_multiplier: u32,

    #[arg(long, default_value_t = String::new())]
    output_file: String,
}

fn main() {
    let mut args = GeneratorArgs::parse();

    if args.seed.trim().is_empty() {
        args.seed = rand::thread_rng()
            .sample_iter(&Alphanumeric)
            .take(32)
            .map(char::from)
            .collect();
    }

    Generator::new(args.seed, args.size)
        .set(Debugging(args.debug))
        .set(Steppers(args.steppers))
        .set(Steps(args.steps))
        .generate()
        .output_image("output.png".to_string(), args.output_multiplier)
        .output_file("output.txt".to_string());
}
