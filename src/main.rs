use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();

    let filename = &args[1];
    let query = &args[2];

    println!("arxiu: {}", filename);
    println!("buscar: {}", query);

    let content = fs::read_to_string(filename).expect("The file could not be read");
    println!("{}", &content[..30]);
}
