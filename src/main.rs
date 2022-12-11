use clap::Parser;
use rand::distributions::Alphanumeric;
use rand::Rng;
use rust_map_gen::generator::Generator;

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

    let mut generator = Generator::new(args.debug, args.seed, args.size, args.rivers, args.steppers, args.steps);

    generator.generate();
    generator.output_image("output.png".to_string(), 4);
    generator.output_file("output.txt".to_string());
}
