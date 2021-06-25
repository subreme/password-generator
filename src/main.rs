mod characters;
mod config;
mod presets;

use rand::{thread_rng, Rng};

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

pub fn gen(length: u32, characters: &str) -> String {
    let mut password = String::new();
    for _ in 0..length {
        let i: u32 = thread_rng().gen_range(0..characters.len() as u32);
        password.push(characters.chars().nth(i as usize).unwrap());
    }
    password
}
