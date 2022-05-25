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
        if new_word == "NewGame"{
            return vocabulary;
        }
        if new_word.len() == words_len && !vocabulary.contains(&new_word){
            vocabulary.push(new_word);
        }
    }
}

fn compare_words(reference: &String, attempt: &String){
    let mut chars_ref: Vec<char> = reference.chars().collect();
    let chars_atm: Vec<char> = attempt.chars().collect();
    let mut chars_out: Vec<char> = Vec::new();

    for idx in 0..attempt.len(){
        if chars_ref[idx] == chars_atm[idx]{
            chars_out.push('+');
        } else {
            let mut cb: bool = true;
            for check in 0..reference.len(){
                if chars_ref[check] == chars_atm[idx]{
                    chars_out.push('|');
                    chars_ref[check] = '-';
                    cb = false;
                    break;
                }
            }
            if cb{
                chars_out.push('/');
            }
        }
    }
    println!("{}", String::from_iter(chars_out));
}

fn print_filtered(){}

fn start_adding(vocabulary: &mut Vec<String>){
    let words_len = (*vocabulary.get(0).unwrap()).len();
    loop{
        let new_word = get_input();
        if new_word == "StopAdding"{
            return;
        }
        if new_word.len() == words_len && !vocabulary.contains(&new_word){
            vocabulary.push(new_word);
        }
    }
}

fn new_game(vocabulary: &mut Vec<String>){
    let mut ref_word = get_input();

    while !vocabulary.contains(&ref_word){
        ref_word = get_input();
    }

    let mut attempts: usize = get_input().parse().unwrap();

    while attempts > 0 {
        let input = get_input();

        match input.as_str(){
            "Print" => print_filtered(),
            "StartAdding" => start_adding(vocabulary),
            _ => {
                if &input == &ref_word{
                    println!("You won!");
                    return;
                } else {
                    compare_words(&ref_word, &input);
                    attempts -= 1;
                }
            }
        }
    }
}

fn main() {
    println!("Welcome to a Rust-y version of Wordle!\nHow many characters will your words contain?");
    
    let words_len: usize = get_input().parse().unwrap();

    println!("Your words will be {} characters long! Start adding words to our vocabulary.", words_len);
    let mut vocabulary: Vec<String> = create_vocabulary(words_len);

    println!("{:?}", vocabulary);
    
    new_game(&mut vocabulary);

    println!("{:?}", vocabulary);
}
