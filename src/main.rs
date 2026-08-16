use std::env;
use std::process::exit;

fn main() {
    let args: Vec<String> = env::args().collect();

    if (args.len() < 2) {
        println!("Specify the target executable");
        exit(1);
    }

    println!("The args: {}", args[1]);
}
