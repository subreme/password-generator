use rand::Rng;
use std::io;
use std::process;

fn main() {
    // This was previously printed with just one `println!`
    // macro but I divided it to improve legibility
    println!("Password Generator");
    println!("- Press `enter` to use the default value");

    // The `restart` command is currently disabled...
    /*println!("- Type `restart` to start over");*/
    println!("- Type `exit` to quit\n");

    // This loop has a `'label`, as this allows me to restart the
    // configuration at any time from within a nested loop
    /*'main:*/
    loop {
        println!("\nHow many characters? (Default: `32`)");

        // `len` is initially used to store the user's input
        let mut len = String::new();

        io::stdin()
            .read_line(&mut len)
            .expect("Failed to read input!");
        // Pattern matching is used to interpret `len` and shadow
        // it with an integer value (the desired password length)
        let len: u32 = match len.trim().parse() {
            // If the trimmed String can be parsed to an integer,
            // this `if` statement makes sure `len`'s value is
            // positive, for obvious reasons...
            Ok(len) => {
                // As it turns out, the only invalid integer is zero,
                // since `len`'s `u32` type results in an error if
                // the parsing of a negative number is attempted
                if len /*>*/ != 0 {
                    len
                } else {
                    // This error message was edited for the same reason
                    /*println!("\nPassword length must be a positive integer!");*/
                    println!("\nPassword length cannot be zero!");

                    // This will repeat the current loop, prompting
                    // a length value once again
                    continue;
                }
            }

            // Since several input values could result in an error,
            // several `if` statements are used to react accordingly
            Err(_) => {
                // To allow for the autocompletion of the default value,
                // this checks if the input doesn't contain any characters
                if len.trim().is_empty() {
                    // In order to make the program more intuitive, the
                    // default value is printed when this is the case
                    println!("32");
                    32
                // As mentioned in line 10, "restart" is treated as a
                // command allowing the user to reset the config values
                /*} else if len.trim() == "restart" {
                // By using the outer-most loop's label, the whole
                // config process can be restarted
                break 'main;*/

                // "Exit" is also a recognized command, as mentioned in
                // line 11, allowing users to easily quit the program if
                // they are not familiat with `^C`
                } else if len.trim() == "exit" {
                    // The process is terminated with a default error message
                    process::exit(1);

                // All other inputs will be considered invalid
                } else {
                    println!("Invalid input!");
                    continue;
                }
            }
        };
        let low: bool = config_bool("Include lowercase letters?", true);
        let upp: bool = config_bool("Include uppercase letters?", true);
        let num: bool = config_bool("Include numbers?", true);
        let spe: bool = config_bool("Include special characters?", true);

        // Once the configuration is done, a String with all
        // accepted characters is created as the source for
        // the password's contents
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

        // The password is generated and displayed
        println!("\nYour password is: {}", gen(len, &characters));

        // After the first passoword generation,
        // the user is allowed to generate more
        // using the same settings
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

                // This input is approached slightly
                // differently, as ending the loop is
                // enough to terminate the program
                "no" | "n" | "exit" => {
                    break;
                }

                // As usual, though, the `restart`
                // command can be used to reset
                // the password settings
                /*"restart" => {
                    break 'main;
                }*/
                _ => {
                    println!("\nPlease input one of the following arguments:");
                    println!("- `Yes` or `Y`");
                    println!("- `No` or `N`");
                    println!("(not case-sensitive)");
                    continue;
                }
            }
        }

        break;
    }
}

// This function is the first step in streamlining the
// configuration process, greatly simplifying the code
fn config_bool(prompt: &str, default: bool) -> bool {
    let default: String = if default {
        String::from("Yes")
    } else {
        String::from("No")
    };

    let result: bool = loop {
        println!("\n{} (Default: `{}`)", prompt, default);
        let mut x = String::new();
        io::stdin()
            .read_line(&mut x)
            .expect("Failed to read input!");
        match x.trim().to_lowercase().as_str() {
            "yes" | "y" | "" => {
                if x.trim().is_empty() {
                    println!("Yes");
                }

                break true;
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

fn gen(length: u32, characters: &String) -> String {
    let mut password = String::new();

    for _ in 0..(length + 1) {
        let i: u32 = rand::thread_rng().gen_range(1..(characters.len() as u32));

        password.push(characters.chars().nth(i as usize).unwrap());
    }
    password
}
