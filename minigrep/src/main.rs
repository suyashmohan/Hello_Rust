use std::env;
use std::error::Error;
use std::process;

use hello_rust::Config;

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();
    let conf = Config::new(&args).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    if let Err(e) = hello_rust::run(conf) {
        eprintln!("Application error: {}", e);
        process::exit(1);
    }
    Ok(())
}
