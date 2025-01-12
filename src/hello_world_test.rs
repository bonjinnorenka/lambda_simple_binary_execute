use std::env;
use std::io::{self, Read};

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.contains(&"--panic".to_string()) {
        panic!("Panic triggered by --panic argument");
    }

    if args.contains(&"--use-stdin".to_string()) {
        let mut input = String::new();
        io::stdin().read_to_string(&mut input).expect("Failed to read from stdin");
        println!("{}", input);
    } else {
        println!("Hello, world!");
    }
}
