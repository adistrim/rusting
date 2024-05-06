fn main() {
    let sentence = String::from("Rust is a lot");
    let is_string: bool = true;

    if is_string {
        println!("{}", get_first_word(sentence))
    } else {
        println!("Not a string");
    }
}

fn get_first_word(sentence: String) -> String {
    let mut word = String::from("");
    for c in sentence.chars() {
        if c == ' ' {
            break;
        }
        word.push(c);
    }
    return word;
}
