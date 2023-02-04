use std::fs;

pub struct Config {
    pub filename: String,
    pub query: String,
}

impl Config {
    pub fn new(args: &[String]) -> Config {
        let filename = args[1].clone();
        let query = args[2].clone();

        Config { filename, query }
    }
}

pub fn run(config: Config) {
    let content = fs::read_to_string(config.filename).expect("The file could not be read");
    println!("{}", &content[..30]);
}
