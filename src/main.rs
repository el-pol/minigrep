use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::new(&args);

    println!("arxiu: {}", config.filename);
    println!("buscar: {}", config.query);

    let content = fs::read_to_string(config.filename).expect("The file could not be read");
    println!("{}", &content[..30]);
}

struct Config {
    filename: String,
    query: String,
}

impl Config {
    fn new(args: &[String]) -> Config {
        let filename = args[1].clone();
        let query = args[2].clone();

        Config { filename, query }
    }
}
