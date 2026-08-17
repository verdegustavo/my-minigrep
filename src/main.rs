use std::env;
use std::fs;
use std::process;

fn main() {
    let args: Vec<String> = env::args().collect();
    let config = Config::build(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {err}");
        process::exit(1)
    });

    println!("Searching for {}", config.query);
    println!("Searching for {}", config.file_path);

    let contents =
        fs::read_to_string(config.file_path).expect("Should have been able to read the file.");

    println!("With text:\n{contents}")
}

struct Config<'a> {
    query: &'a str,
    file_path: &'a str,
}

impl Config<'_> {
    fn build(args: &[String]) -> Result<Config<'_>, &'static str> {
        if args.len() < 3 {
            return Err("Not enough arguments.");
        }
        let query = &args[1];
        let file_path = &args[2];

        Ok(Config { query, file_path })
    }
}
