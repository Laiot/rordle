use std::io::*;
use std::collections::HashMap;

/**
 * Struct used to filter out the words in vocabulary that don't comply with the following constraints: 
 * + Given character not present in word ✅ 
 * + Given character is present in known position in word ✅ 
 * + Given character is not present in known position in word 
 * + Minimum repetitions of given character in word
 * + Exact repetitions of given character in word ✅ 
 * 
 * @confirmed_chars: vector of same length as word that contains the correct found characters 
 *  ->  in word in the correct position and the character '-' in the positions where the correct character has not yet been found
 * 
 * @tofind_chars: hashmap that contains the characters found in the wrong position in word and how many times they need to be found in word yet
 * 
 * @nonpresent_chars: vector containing any character that is not present in word in any position
 */
#[derive(Debug)]
struct Filter{
    confirmed_chars: Vec<char>,
    tofind_chars: HashMap<char, usize>,
    nonpresent_chars: Vec<char>
}

/**
 * Function used to get input from standard input 
 * Returns a String stripped from possible special characters like '/n' and '/r'
 */
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

/**
 * Function used to create a first container for possible words that can be found in the game
 * @words_len: length that every word in the vocabulary must have
 * @returns a vector of strings
 */
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

/**
 * Function that compares two words and prints a logical output of the comparison
 */
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

/**
 * Function that prints the possible words according to the restraints
 */
fn print_filtered(vocabulary: &Vec<String>, filter: &Filter){
    'outer: for word in vocabulary{
        for npr in &filter.nonpresent_chars{
            if word.contains(*npr){
                for tfc in &filter.tofind_chars{
                    if word.chars().filter(|&n| n >= *tfc.0).count() == *tfc.1{
                        for (pos, cfc) in filter.confirmed_chars.iter().enumerate(){
                            if word.chars().nth(pos).unwrap() != *cfc{
                                break 'outer;
                            }
                        }
                        println!("{}", word);
                    }
                }
            }
        }
    }
}

/**
 * Function that lets the player add words to the vocabulary
 */
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

/**
 * Function that starts a new game
 */
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
            "Print" => print_filtered(vocabulary, &filter),
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
