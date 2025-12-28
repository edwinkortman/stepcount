use crate::stepcount::stepcount::stepcount;
use crate::memstep::memstep::memstep;
use clap::Parser;
use std::time::{Instant};

mod stepcount;
mod memstep;

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

    let start = Instant::now();
    match args.method.as_str() {
        "stepcount" => {
            if args.stepcount > 40 {
                println!("Error: stepcount ({}) is too large (> 40). Naive implementation will be too slow.", args.stepcount);
            } else {
                println!("Result: {}", stepcount(args.stepcount, args.steps))
            }
        },
        "memstep" => println!("Result: {}", memstep(args.stepcount, args.steps)),
        _ => println!("Unsupported method: {}", args.method),
    }
    let duration = start.elapsed();
    println!("Time elapsed: {:?}", duration);
}
