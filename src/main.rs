use rand::Rng;
use std::io;
use std::process;

fn main() {
    println!("Password Generator");
    println!("- Press `enter` to use the default value");
    println!("- Type `exit` to quit\n");

    let len: u32 = config_u32("How many characters?", 32);
    let low: bool = config_bool("Include lowercase letters?", true);
    let upp: bool = config_bool("Include uppercase letters?", true);
    let num: bool = config_bool("Include numbers?", true);
    let spe: bool = config_bool("Include special characters?", true);

    let mut characters = String::new();

    if low {
        characters.push_str("abcdefghijklmnopqrstuvwxyz");
    };

    if upp {
        characters.push_str("ABCDEFGHIJKLMNOPQRSTUVWXYZ");
    };

    if num {
        characters.push_str("1234567890");
    };

    if spe {
        characters.push_str("!\"#$%&'()*+,-./:;<=>?@[\\]^_`{|}~");
    };

    println!("\nYour password is: {}", gen(len, &characters));

    loop {
        println!("\nRun again? (Default: `Yes`)");

        let mut new = String::new();

        io::stdin()
            .read_line(&mut new)
            .expect("Failed to read input!");
        match new.trim().to_lowercase().as_str() {
            "yes" | "y" | "" => {
                if new.trim().is_empty() {
                    println!("Yes");
                }
                println!("\nYour password is: {}", gen(len, &characters));
            }

            "no" | "n" | "exit" => {
                break;
            }

            _ => {
                println!("\nPlease input one of the following arguments:");
                println!("- `Yes` or `Y`");
                println!("- `No` or `N`");
                println!("(not case-sensitive)");
                continue;
            }
        }
    }
}

fn config_bool(prompt: &str, default: bool) -> bool {
    let default_text: String = if default {
        String::from("Yes")
    } else {
        String::from("No")
    };

    let result: bool = loop {
        println!("\n{} (Default: `{}`)", prompt, default_text);
        let mut x = String::new();
        io::stdin()
            .read_line(&mut x)
            .expect("Failed to read input!");
        match x.trim().to_lowercase().as_str() {
            "yes" | "y" | "" => {
                if x.trim().is_empty() {
                    println!("{}", default_text);
                }

                break default;
            }

            "no" | "n" => break false,

            "exit" => {
                process::exit(1);
            }

            _ => {
                println!("\nPlease input one of the following arguments:");
                println!("- `Yes` or `Y`");
                println!("- `No` or `N`");
                println!("(not case-sensitive)");
                continue;
            }
        };
    };

    result
}

fn config_u32(prompt: &str, default: u32) -> u32 {
    let result: u32 = loop {
        println!("\n{} (Default: `{}`)", prompt, default);
        let mut x = String::new();

        io::stdin()
            .read_line(&mut x)
            .expect("Failed to read input!");
        match x.trim().parse::<u32>() {
            Ok(x) => {
                if x != 0 {
                    x
                } else {
                    println!("\nValue cannot be zero!");
                    continue;
                }
            }

            Err(_) => {
                if x.trim().is_empty() {
                    println!("{}", default);
                    break default;
                } else if x.trim() == "exit" {
                    process::exit(1)
                } else {
                    println!("Invalid input!");
                    continue;
                }
            }
        };
    };

    result
}

fn gen(length: u32, characters: &String) -> String {
    let mut password = String::new();

    for _ in 0..(length + 1) {
        let i: u32 = rand::thread_rng().gen_range(1..(characters.len() as u32));

        password.push(characters.chars().nth(i as usize).unwrap());
    }
    password
}
