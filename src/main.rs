use clap::Parser;
use rand::{thread_rng, Rng};


#[derive(Parser)]
struct Args {
    file: String,
}

fn main() {
    let args = Args::parse();

    let content = std::fs::read_to_string(&args.file).unwrap();
    let items: Vec<&str> = content.split("\n\n").collect();

    let mut rng = thread_rng();
    let random_index = rng.gen_range(0..items.len());

    println!("{}", items[random_index]);
}
