//! # minigrep
//!
//! Basic functionality of minigrep allows the user to
//! search a specified text file for a certain query (string) to match.
//! It also allows the user to create an environment variable IGNORE_CASE
//! to determine if searches should be case-insensitive.
//! Additional functionality is refactoring to use clap, as well as adding
//! a count flag which the user can enable to only return counts as opposed to
//! the specific lines that matched.

use minigrep::{search, search_case_insensitive, search_case_insensitive_count, search_count};
use std::error::Error;
use std::fs;
use std::process;

use clap::Parser;

#[derive(Parser)]
#[command(author, version, long_about = None)]
#[command(name = "minigrep")]
#[command(about = "Search for patterns in some text")]
struct Config {
    pub query: String,
    pub file_path: String,
    #[arg(long = "ignore_case", env = "IGNORE_CASE")]
    pub ignore_case: bool,
    #[arg(short = 'c', long = "count")]
    pub count: bool,
}

fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.file_path)?;
    if config.count {
        println!("{}", {
            if config.ignore_case {
                search_case_insensitive_count(&config.query, &contents)
            } else {
                search_count(&config.query, &contents)
            }
        })
    } else {
        let results = {
            if config.ignore_case {
                search_case_insensitive(&config.query, &contents)
            } else {
                search(&config.query, &contents)
            }
        };

        for line in results {
            println!("{line}");
        }
    };

    Ok(())
}

fn main() {
    let config = Config::parse();

    if let Err(e) = run(config) {
        eprintln!("Application error: {e}");
        process::exit(1);
    }
}
