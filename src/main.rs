use my_minigrep::search;
use my_minigrep::search_case_insensitive;
use std::env;
use std::error::Error;
use std::fs;
use std::process;

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::build(&args).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {err}");
        process::exit(1)
    });

    if let Err(e) = run(&config) {
        eprintln!("Application error: {e}");
        process::exit(1)
    }
}

struct Config<'a> {
    query: &'a str,
    file_path: &'a str,
    ignore_case: bool,
}

impl Config<'_> {
    fn build(args: &[String]) -> Result<Config<'_>, &'static str> {
        if args.len() < 3 {
            return Err("Not enough arguments.");
        }
        let query = &args[1];
        let file_path = &args[2];
        let mut ignore_case = false;
        if let Ok(env_value) = env::var("IGNORE_CASE") && env_value == "Yes" {
                ignore_case = true
        };

        Ok(Config {
            query,
            file_path,
            ignore_case,
        })
    }
}

fn run(parameters: &Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(parameters.file_path)?;

    let result = if parameters.ignore_case {
        search_case_insensitive(parameters.query, &contents)
    } else {
        search(parameters.query, &contents)
    };

    for line in result {
        println!("{line}");
    }

    Ok(())
}
