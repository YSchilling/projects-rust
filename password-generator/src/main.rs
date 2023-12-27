// Password Generator in Rust

use rand::{thread_rng, Rng};
use clap::Parser;

#[derive(Parser)]
struct CLIArguments {
    #[arg(default_value_t = 32)]
    length: u32,
    #[arg(short, long)]
    big_letters: bool,
    #[arg(short, long)]
    numbers: bool,
    #[arg(short, long)]
    symbols: bool
}

fn gen_pw(args: CLIArguments) -> String {
    const BIG_LETTERS: &str= "ABCDEFGHIJKLMNOPQRSTUVWXYZ";
    const NUMBERS: &str = "0123456789";
    const SYMBOLS: &str = "!?()$&-_@";

    let mut chars = String::from("abcdefghijklmnopqrstuvwxyz");

    if args.big_letters {
        chars.push_str(BIG_LETTERS);
    }

    if args.numbers {
        chars.push_str(NUMBERS);
    }

    if args.symbols {
        chars.push_str(SYMBOLS);
    } 
    
    (0..args.length).map(|_| {
        let i = thread_rng().gen_range(0..chars.len());
        chars.chars().nth(i).unwrap()
    }).collect()
}

fn main() {
    let args = CLIArguments::parse();

    let password = gen_pw(args);

    println!("{password}");
}