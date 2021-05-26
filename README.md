# Password Generator
While going through the [Rust Book](https://doc.rust-lang.org/book/), I decided I wanted to write a simple project to implement the new concepts I learned and get used to working with Rust before moving onto a serious project.
I ended up choosing to write a Password Generator, as I was in the process of choosing new ones for several of my accounts, and was finding out the maximum number of characters allowed on different sites.

## Usage
The tool is meant to be simple to use, while gradually gaining more functionality as I become more proficient in the language.

### Selecting A Preset
This password generator supports multiple presets, allowing for a setup-free generation of the most secure passwords for pre-defined platforms as well as a `Custom` mode, allowing the user to manually set all toggles at will.

The program's first prompt asks the user to select the desired preset, listing the available options as follows:

```
Select a default preset:
#1 Google
#2 Discord
#3 Custom
```

In order to allow for intuitive and error-free usage, the program allows for several different valid inputs to choose an option.

For example Option #1, "Google", could be selected using any of the following inputs:
- "Google", typed using any capitalization
- "1"
- "#1"

Before processing the user's input, the inputted strings are "trimmed", removing any initial or final whitespace, therefore any excess spaces are ignored and will not be recognized as invalid.

Some sites can even be selected using aliases. "Google", for example, can also be selected by typing "Gmail" and "YouTube".

### Custom Settings
The final option in the preset list is `Custom`, which allows you to adjust all parameters.

#### Default Options
Some settings have a default option, indicated using the following format: ``Question? (Default: `Value`)``. In order to quickly select the default option, the user can simply press `Enter`, as long as no non-whitespace character is included.

#### Invalid Input
If the input is invalid or an error occurs, the program will simply report the problem and once again prompt the user with the setting, specifying the recognized inputs if necessary, using messages like the one below:

```
Please input one of the following arguments:
- `Yes` or `Y`
- `No` or `N`
(not case-sensitive)
```

#### Password Length
The first setting the user is prompted with is `How many characters?`, which defaults to `32`. The question accepts a minimum value of `1` and a maximum of `4294967295`, the highest value that can be store in an Unsigned 32 Bit Integer, however due to the program's recursive password-generation process *(and absolutely no website accepting such a long password)` using shorter passwords is highly recommended.

Inputting `0` will show the error message `Value cannot be zero!`, while entering a number that cannot be contained by a `u32` or a non-numerical character will simply return the error `Invalid input!`

#### Include...
The next settings all function similarly, acting as boolean toggles specifying whether characters of a certain type should be included or not, as shown in the following table:

| Question                    | Default | Characters                          |
|-----------------------------|---------|-------------------------------------|
| Include lowercase letters?  | Yes     | abcdefghijklmnopqrstuvwxyz          |
| Include uppercase letters?  | Yes     | ABCDEFGHIJKLMNOPQRSTUVWXYZ          |
| Include numbers?            | Yes     | 1234567890                          |
| Include special characters? | Yes     | !\"#$%&'()*+,-./:;<=>?@[\\]^_`{\|}~ |

As mentioned in the example above, the prompt recognizes two inputs:

- "Yes" or "Y"
- "No" or "N"

As usual, the input isn't case-sensitive.

#### Run Again?
After a password is generated using the selected parameters, the user is asked if the program should generate a new one using the same settings.

The prompt works exactly like the `Include...` questions, as it operates using the same function ([`config::bool()`](src/lib.rs#L66-L95)), and therefore recognizes the same inputs.

### Quitting
The program can be terminated at any time by inputting either "quit" or "exit".

Alternatively, once a password has been generated at least once, selecting "No" when asked if the generator should run again will end the program.

## Generation
Due to my lack of experience, the password generating function acts very simply, as shown below:

```rust
use rand::{thread_rng, Rng};
pub fn gen(length: u32, characters: &String) -> String {
    let mut password = String::new();
    for _ in 0..length {
        let i: u32 = thread_rng().gen_range(0..characters.len() as u32);
        password.push(characters.chars().nth(i as usize).unwrap());
    }
    password
}
```

A loop is repeated the same number of times as the desired password length, specified by the `length` parameter, and a random character from the provided sample string is selected and appended to `password`.

I am not aware of whether or not this approach is secure or not, and am inclined to believe that recursion is not the most efficient approach for this function to work. I will therefore most likely rewrite it to operate differently in the future, once I've learned more and can figure out a more appropriate method.

## Running
I wrote this program for practice, and although I'll occasionally use it myself, I see little practical use for it.

Regardless, if you want to use it yourself, you should first [install Rust](https://www.rust-lang.org/learn/get-started).

Once you Rust's installed, you can clone the repository using [GitHub CLI](https://cli.github.com/), using the command `gh repo clone subreme/password-generator`.

Alternatively, you can simply download the [ZIP](https://github.com/subreme/password-generator/archive/refs/heads/main.zip) and uncompress the file.

The project can then be run using [Cargo](https://doc.rust-lang.org/cargo/), installed with the Rust Language, by accessing the directory using the command `cargo run`.

The Password Generator can also be compiled using the command `cargo build --release`, and the binaries will be generated in the path `password-generator\target\release\password_generator.exe`.

Once the project is closer to being complete, I'll compile it for different platforms so that the binaries can be downloaded directly, eliminating the need for the however I have not done so yet as I am still in the process of adding more functionalities.
