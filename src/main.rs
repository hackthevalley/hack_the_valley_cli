#[macro_use]
extern crate clap;
use clap::App;

// Include all commands
include!("api_ver.rs");

fn main() {

    // Initialize CLI
    let yaml = load_yaml!("cli.yml");
    let matches = App::from_yaml(yaml).get_matches();

    // Unwrap arguments
    let url = matches.value_of("url").unwrap_or("https://api.hackthevalley.io");
    let key = matches.value_of("key");
    println!("Running command on endpoint: {}", url);
    match key {
        Some(key) => println!("Using key: {}", key),
        None => println!("Not using key.")
    }

    // Decide which subcommand to run
    if let Some(matches) = matches.subcommand_matches("api-ver") {
        api_ver(url);
    }

    if let Some(matches) = matches.subcommand_matches("hackers") {
        println!("hackers...");
    }

}
