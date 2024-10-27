use std::env;
use std::fs;
use std::process;

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::build(&args).unwrap_or_else(|err| {
            println!("Problem parsing argument {err}");
            process::exit(1);
    });

    println!("Searching for {} in {}", config.query, config.file_path);

    let file = fs::read_to_string(config.file_path).expect("File not accessible");

    println!("File contents: {file}");
}

struct Config {
    query: String,
    file_path: String
}

impl Config {
    fn build(args: &[String]) -> Result<Config, &'static str> {
        if args.len() != 3 {
            return Err("Not enough or too many arguments, expected query and file_path")
        }

        let query = args[1].clone();
        let file_path = args[2].clone();

        Ok(Config { query, file_path })
    }
}