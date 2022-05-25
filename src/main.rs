use std::io::*;
use std::collections::HashMap;

#[derive(Debug)]
struct Filter{
    confirmed_chars: Vec<char>,
    tofind_chars: HashMap<char, usize>,
    nonpresent_chars: Vec<char>
}

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

fn compare_words(reference: &String, attempt: &String, filter: &mut Filter){
    let mut chars_ref: Vec<Option<char>> = reference.chars().map(|c| Some(c)).collect();
    let chars_atm: Vec<char> = attempt.chars().collect();
    let mut chars_out: Vec<Option<char>> = Vec::new();

    for idx in 0..attempt.len(){
        if chars_ref[idx] == Some(chars_atm[idx]){
            chars_out.push(Some('+'));
            if filter.tofind_chars.contains_key(&chars_atm[idx]){
                if filter.tofind_chars.get(&chars_atm[idx]) > Some(&1){
                    filter.tofind_chars.entry(chars_atm[idx]).and_modify(|e| {*e -= 1});
                } else {
                    filter.tofind_chars.remove(&chars_atm[idx]);
                }
            }
            filter.confirmed_chars[idx] = chars_atm[idx];
            chars_ref[idx] = None;
        } else {
            chars_out.push(None);
            if !chars_ref.contains(&Some(chars_atm[idx])){
                filter.nonpresent_chars.push(chars_atm[idx]);
            }
        }
    }

    for i1 in 0..attempt.len(){
        if chars_out[i1] != Some('+'){
            let mut check: bool = true;
            for i2 in 0..attempt.len(){
                if chars_ref[i2] == Some(chars_atm[i1]){
                    chars_out[i1] = Some('|');
                    *filter.tofind_chars.entry(chars_atm[i1]).or_insert(0) += 1;
                    chars_ref[i2] = None;
                    check = false;
                    break;
                }
            }
            if check{
                chars_out[i1] = Some('/');
            }
        }
    }
    println!("{:?}", filter);
    println!("{}", chars_out.iter().map(|o| o.unwrap()).collect::<String>());
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

    let mut filter = Filter{
        confirmed_chars: vec!['-'; ref_word.len()],
        tofind_chars: HashMap::new(),
        nonpresent_chars: Vec::new()
    };

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
                    compare_words(&ref_word, &input, &mut filter);
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
