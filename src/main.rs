use std::env;
use std::path::PathBuf;
use std::process;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        eprintln!("Usage: {} <target-path>", args[0]);
        process::exit(1);
    }

    let target_path = PathBuf::from(&args[1]);

    if !target_path.exists() {
        eprintln!("Error: Path '{}' does not exist", target_path.display());
        process::exit(1);
    }

    println!("Target path: {}", target_path.display());
    
    // TODO: Implement translation logic here
}
