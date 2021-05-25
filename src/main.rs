use password_generator::{config, gen, presets};

fn main() {
    println!("Password Generator");
    println!("- Press `enter` to use the default value");
    println!("- Type `exit` to quit\n");

    let (length, characters) = presets::default();

    println!("\nYour password is: {}", gen(length, &characters));

    loop {
        if config::bool("Run again?", true) {
            println!("\nYour password is: {}", gen(length, &characters));
        } else {
            break;
        }
    }
}
