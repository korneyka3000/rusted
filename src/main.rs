use std::io;
const VOWEL_LETTER: &str = "aeiou";
fn main() {

    let mut sentence = String::new();

    println!("Enter words, split by space:");
    io::stdin()
        .read_line(& mut sentence)
        .expect("Failed to read");

    println!("{sentence}");

    let res = basic_str(&sentence);
    println!("{:#?}", res);
    let res1 = with_format_str(sentence);
    println!("{:#?}", res1);
}


fn basic_str(sentence_in: &String) -> String {
    let mut result = String:: new();

    for word in sentence_in.split_whitespace() {
        if VOWEL_LETTER.contains(&word[0..1]) {
            result += &*(word.to_string() + &"-hay ".to_string());
        } else {
            result += &*(word[1..].to_string() + &"-" + &*word[0..1].to_string() + &"ay ");
        }
    }
    result.trim().to_string()
}

fn with_format_str(sentence_in: String) -> String {
    let mut result = "".to_string();
    let letter = "h";
    for word in sentence_in.split_whitespace() {
        if VOWEL_LETTER.contains(&word[0..1]){
            result += &format!("{word}-{letter}ay ");
        } else {
            let first_symb = &word[0..1];
            let start_word = word[1..].to_string();
            result += &format!("{start_word}-{first_symb}ay ")
        }
    }
    result.trim().to_string()
}