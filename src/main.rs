use clap::Parser;
use rand::distributions::Alphanumeric;
use rand::Rng;
use rust_map_gen::generator::Generator;

#[derive(Parser, Debug)]
struct GeneratorArgs {
    #[arg(long, default_value_t = String::new())]
    seed: String,

    #[arg(long, default_value_t = 250)]
    size: u32,

    #[arg(long, default_value_t = 2)]
    rivers: u32,

    #[arg(long, default_value_t = 250)]
    steppers: u32,

    #[arg(long, default_value_t = 300)]
    steps: u32,

    #[arg(long, default_value_t = String::new())]
    output_file: String,
}

#[tokio::main]
async fn main() {
    let mut args = GeneratorArgs::parse();

    if args.seed.trim().is_empty() {
        args.seed = rand::thread_rng()
            .sample_iter(&Alphanumeric)
            .take(32)
            .map(char::from)
            .collect();
    }

    println!("Generating map of size {} with {} rivers.", args.size, args.rivers);

    let mut generator = Generator::new(args.seed, args.size, args.rivers, args.steppers, args.steps);

    generator.generate().await;
    // generator.output_file();
    generator.output_image();
}
