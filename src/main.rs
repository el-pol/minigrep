use minigrep::Config;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::new(&args);

    println!("arxiu: {}", config.filename);
    println!("buscar: {}", config.query);

    minigrep::run(config)
}
