#[cfg(test)]
mod tests {
    mod correct_length {
        use crate::gen;
        #[test]
        fn custom() {
            assert!(gen::gen(16, &String::from("abc")).len() == 16);
        }
    }
}

pub mod presets {
    use super::{characters, config};
    use std::io;
    use std::process;
    pub fn default<'a>() -> (u32, String) {
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
    pub fn custom<'a>() -> (u32, String) {
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
}

pub mod config {
    use std::io;
    use std::process;
    pub fn bool(prompt: &str, default: bool) -> bool {
        let default_text: String = if default {
            String::from("Yes")
        } else {
            String::from("No")
        };
        return loop {
            println!("\n{} (Default: `{}`)", prompt, default_text);
            let mut input = String::new();
            io::stdin()
                .read_line(&mut input)
                .expect("Failed to read input!");
            match input.trim().to_lowercase().as_str() {
                "yes" | "y" => break true,
                "no" | "n" => break false,
                "" => {
                    println!("{}", default_text);
                    break default;
                }
                "exit" | "quit" => process::exit(0),
                _ => {
                    println!("\nPlease input one of the following arguments:");
                    println!("- `Yes` or `Y`");
                    println!("- `No` or `N`");
                    println!("(not case-sensitive)");
                    continue;
                }
            };
        };
    }
    pub fn u32(prompt: &str, default: u32) -> u32 {
        return loop {
            println!("\n{} (Default: `{}`)", prompt, default);
            let mut input = String::new();
            io::stdin()
                .read_line(&mut input)
                .expect("Failed to read input!");
            match input.trim().parse() {
                Ok(input) => {
                    if input != 0 {
                        break input;
                    } else {
                        println!("\nValue cannot be zero!");
                        continue;
                    }
                }
                Err(_) => {
                    if input.trim().is_empty() {
                        println!("{}", default);
                        break default;
                    } else if input.trim() == "exit" {
                        process::exit(0)
                    } else {
                        println!("Invalid input!");
                        continue;
                    }
                }
            };
        };
    }
}

mod characters {
    pub fn all<'a>() -> String {
        String::from("abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ1234567890!\"#$%&'()*+,-./:;<=>?@[\\]^_`{|}~")
    }
    pub fn low<'a>() -> String {
        String::from("abcdefghijklmnopqrstuvwxyz")
    }
    pub fn upp<'a>() -> String {
        String::from("ABCDEFGHIJKLMNOPQRSTUVWXYZ")
    }
    pub fn num<'a>() -> String {
        String::from("1234567890")
    }
    pub fn spe<'a>() -> String {
        String::from("!\"#$%&'()*+,-./:;<=>?@[\\]^_`{|}~")
    }
}

pub mod gen {
    use rand::Rng;
    pub fn gen(length: u32, characters: &String) -> String {
        let mut password = String::new();
        for _ in 0..length {
            let i: u32 = rand::thread_rng().gen_range(0..characters.len() as u32);
            password.push(characters.chars().nth(i as usize).unwrap());
        }
        password
    }
}
