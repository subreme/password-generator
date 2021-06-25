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