use std::io::*;

fn get_input() -> String{
    let mut raw_input = String::new();
    stdin().read_line(&mut raw_input).expect("Something's wrong with your input!");
    if let Some('\n') = raw_input.chars().next_back() {
        raw_input.pop();
    }
    if let Some('\r') = raw_input.chars().next_back() {
        raw_input.pop();
    }
    return raw_input;
}

fn create_vocabulary(words_len: usize) -> Vec<String>{
    let mut vocabulary: Vec<String> = Vec::new();

    loop{
        let new_word = get_input();
        if new_word == "new_game"{
            return vocabulary;
        }
        if new_word.len() == words_len && !vocabulary.contains(&new_word){
            vocabulary.push(new_word);
        }
    }
}

fn new_game(vocabulary: Vec<String>){
    let mut ref_word = get_input();

    while !vocabulary.contains(&ref_word){
        ref_word = get_input();
    }

    let attempts: usize = get_input().parse().unwrap();

    for _ in 0..attempts{
        if get_input() == ref_word{
            println!("You won!");
            break;
        }
    }
}

fn main() {
    println!("Welcome to a Rust-y version of Wordle!\nEnter `h` to ask for help.");
    
    let words_len: usize = get_input().parse().unwrap();

    let vocabulary: Vec<String> = create_vocabulary(words_len);

    println!("{:?}", vocabulary);
    
    new_game(vocabulary);
}
