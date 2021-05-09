use rand::Rng;
use std::io;
use std::process;

fn main() {
    // This text is split into 3 `println!()` calls instead of  single one with
    // newlines (`\n`) as this improves legibility
    println!("Password Generator");
    println!("- Press `enter` to use the default value");
    println!("- Type `exit` to quit\n");

    // `len`, the variable that stores the desired length of the password, is
    // the only config variable which is declared, as the value needs to be
    // passed as a parameter to `gen()` if it's called again

    // The function's arguments are the prompt and the default value (allowing
    // the user to simply hit `enter` to select the reccommended option)
    let len: u32 = config_u32("How many characters?", 32);

    // The other parameter passed to `gen()` is `characters`, a `String` which
    // contains all characters that can be incuded in the generated password
    let mut characters = String::new();

    // This function works very similarly to `config_u32()`, although saving its
    // value to a variable is unnecessary as the respective characters only need
    // to  be appended to `characters` once
    if config_bool("Include lowercase letters?", true) {
        characters.push_str("abcdefghijklmnopqrstuvwxyz");
    };

    // The same function is called again, with different parameters, for each
    // configuration value
    if config_bool("Include uppercase letters?", true) {
        characters.push_str("ABCDEFGHIJKLMNOPQRSTUVWXYZ");
    };

    if config_bool("Include numbers?", true) {
        characters.push_str("1234567890");
    };

    if config_bool("Include special characters?", true) {
        characters.push_str("!\"#$%&'()*+,-./:;<=>?@[\\]^_`{|}~");
    };

    // `len` and `characters` are finally passed to `gen()` and the first
    // password is generated, with `characters` being borrowed so that it
    // doesn't go out of scope and can be used again to invoke `gen()`
    println!("\nYour password is: {}", gen(len, &characters));

    // This is used to ask the user to run the program again, calling `gen()`
    // using the same parameters, or quit the program (as stated in line 10)
    loop {
        println!("\nRun again? (Default: `Yes`)");

        // This variable stores the user's input
        let mut new = String::new();

        io::stdin()
            .read_line(&mut new)
            .expect("Failed to read input!");
        // [I'm not sure why rustfmt doesn't let me leave an empty line here]
        //
        // The value of `new` is processed to allow more freedom with the user's
        // input, eliminating case-sensitivity and discarding trailing
        // whitespace, while converting the `String` variable to a `str` to make
        // the conditional statements within `match{}` cleaner
        match new.trim().to_lowercase().as_str() {
            // To allow for default values, an empty input is also recognized
            "yes" | "y" | "" => {
                // If the user selects the default option, that value is printed
                // to the console, imitating what it would look like if the user
                // had typed it instead
                if new.trim().is_empty() {
                    // I still haven't figured out a way to have this be
                    // indistinguishable from real input in the console as the
                    // empty input leaves a blank line (I've tried using `/b`
                    // but it does't work), but I'll keep trying different
                    // methods until I get it to work
                    println!("Yes");
                }

                // The password is generated again, exactly the same way as it
                // was in line 48
                println!("\nYour password is: {}", gen(len, &characters));
            }

            // "exit` is still recognized as a valid input (as specified
            // earlier) and is treated the same was as "no"

            // Since there isn't any other code in `main()` after this, the
            // program can effectively be terminated by breaking the loop
            "no" | "n" | "exit" => break,

            // Everything else is considered invalid, therefore a list of vaild
            // inputs is provided to assist the user
            _ => {
                println!("\nPlease input one of the following arguments:");
                println!("- `Yes` or `Y`");
                println!("- `No` or `N`");
                println!("(not case-sensitive)");

                // This will simply run the loop again
                continue;
            }
        }
    }
}

// This function simply returns a `bool` based on the user's input, triggering
// the `if` statements and appending the matching `str` to `characters` if
// `true` is returned
fn config_bool(prompt: &str, default: bool) -> bool {
    // `deafult_text` is declared, as the `default` parameter doesn't match the
    // "Yes" default input it corresponds to, and the variable can't be shadowed
    // as this would complicate the function's subsequent logic
    let default_text: String = if default {
        String::from("Yes")
    } else {
        String::from("No")
    };

    // By using `return`, the loop's  output is returned instead of having it
    // run forever (which would happen if "return" and the final semicolon were
    // removed, as I attempted earlier)
    return loop {
        println!("\n{} (Default: `{}`)", prompt, default_text);

        // Overall, the function is extremely similar to the code within the
        // `loop{}` on line 52
        let mut x = String::new();
        // [Again, why doesn't rustfmt let me leave spaces?]
        io::stdin()
            .read_line(&mut x)
            .expect("Failed to read input!");
        // [...]
        match x.trim().to_lowercase().as_str() {
            // The only significant difference with the previous `loop{}` is
            // that the values are returned through `break`
            "yes" | "y" => break true,
            "no" | "n" => break false,

            "" => {
                println!("{}", default_text);
                break default;
            }

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
}

// Although this function is currently only used once, it was created anyway as
// it may be useful as more features are intoduced
fn config_u32(prompt: &str, default: u32) -> u32 {
    // Unlike `config_bool()`, the default value doesn't have to be processed
    // and can be displayed as-is
    return loop {
        println!("\n{} (Default: `{}`)", prompt, default);
        // [...]
        let mut x = String::new();

        io::stdin()
            .read_line(&mut x)
            .expect("Failed to read input!");
        // [...]
        // Since in this case `match{}` doesn't return to a variable's
        // declaration, `parse()`'s target HAD to be explicitly stated by using
        // "::<u32>", however I tested it again and the compiler seems to have
        // stopped complaining...
        match x.trim().parse/*::<u32>*/() {
            Ok(x) => {
                if x != 0 {
                    break x;
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
}

fn gen(length: u32, characters: &String) -> String {
    let mut password = String::new();

    for _ in 0..(length + 1) {
        // By default, `.size()` returns a `usize` value, therefore we must tell
        // the compiler to interpret the value as a `u32` to match "1"
        let i: u32 = rand::thread_rng().gen_range(1..(characters.len() as u32));

        // Here, the opposite is true, so `as` is used again
        password.push(characters.chars().nth(i as usize).unwrap());
    }
    password
}
