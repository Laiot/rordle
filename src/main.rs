use std::io::*;

fn main() {
    println!("Welcome to a Rust-y version of Wordle!\nEnter `h` to ask for help.");
    loop {
        let _=stdout().flush();
        let mut raw_input = String::new();
        stdin().read_line(&mut raw_input).expect("Something's wrong with your input!");
        if let Some('\n')=raw_input.chars().next_back() {
            raw_input.pop();
        }
        println!("You typed: {}",raw_input);
    }
}
