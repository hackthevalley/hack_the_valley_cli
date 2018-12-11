#[macro_use]
extern crate clap;
use clap::App;

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
        println!("api-ver...");
    }

    if let Some(matches) = matches.subcommand_matches("hackers") {
        println!("hackers...");
    }

}
