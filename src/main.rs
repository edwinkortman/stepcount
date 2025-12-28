use crate::stepcount::stepcount::stepcount;
use clap::Parser;

mod stepcount;

#[derive(Parser)]
struct Cli {
    method: String,
    stepcount: i32,
    #[arg(value_delimiter = ',')]
    steps: Vec<i32>,
}

fn main() {
    let args = Cli::parse();

    println!("Determining routes using '{}' with stepcount {}, and steps {:?}", args.method, args.stepcount, args.steps);

    match args.method.as_str() {
        "stepcount" => println!("Result: {}", stepcount(args.stepcount, args.steps)),
        _ => println!("Unsupported method: {}", args.method),
    }
}
