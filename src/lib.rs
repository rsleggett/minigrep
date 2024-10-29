use std::{error::Error, fs};

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let file = fs::read_to_string(config.file_path)?;

    Ok(())
}

pub struct Config {
    query: String,
    file_path: String
}

impl Config {
    pub fn build(args: &[String]) -> Result<Config, &'static str> {
        if args.len() != 3 {
            return Err("Not enough or too many arguments, expected query and file_path")
        }

        let query = args[1].clone();
        let file_path = args[2].clone();

        Ok(Config { query, file_path })
    }
}

#[cfg(test)]
mod tests
{
    use super::*;

    #[test]
    fn one_result() {
        let query = "duct";

        let contents = "\
Rust:
safe, fast, productive.
Pick three.
        ";

        assert!(vec!["safe", "fast", "productive."], search(query, contents));
    }
}