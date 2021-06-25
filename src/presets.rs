use super::{characters, config};
use std::io;
use std::process;
pub fn default() -> (u32, String) {
    println!("Select a default preset:");
    let mut input = String::new();
    loop {
        println!("#1 Google");
        println!("#2 Discord");
        println!("#3 Custom\n");
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read input!");
        match input.trim().to_lowercase().as_str() {
            "1" | "#1" | "google" | "gmail" | "youtube" => break (100, characters::all()),
            "2" | "#2" | "discord" => break (72, characters::all()),
            "3" | "#3" | "custom" => break self::custom(),
            "exit" | "quit" => process::exit(0),
            _ => {
                println!("\nPlease choose one of the options, using its name or its corresponding number:\n");
                continue;
            }
        }
    }
}
pub fn custom() -> (u32, String) {
    let len: u32 = config::u32("How many characters?", 32);
    let mut characters = String::new();
    if config::bool("Include lowercase letters?", true) {
        characters.push_str(characters::low().as_str());
    };
    if config::bool("Include uppercase letters?", true) {
        characters.push_str(characters::upp().as_str());
    };
    if config::bool("Include numbers?", true) {
        characters.push_str(characters::num().as_str());
    };
    if config::bool("Include special characters?", true) {
        characters.push_str(characters::spe().as_str());
    };
    (len, characters)
}
