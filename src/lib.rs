mod getstructure;
mod getword;
use getstructure::get_structure;
use getword::get_word;



fn string_cleanup(str: String) -> String {
    let mut char_vec: Vec<char> = str.chars().collect();
    char_vec[0] = char_vec[0].to_ascii_uppercase();
    // It's absolutely ridiculous how much work it takes to convert the first letter of a String to uppercase in Rust.
    char_vec[(str.len() - 1) as usize] = '.';
    // I never want to touch this cleanup function again.
    char_vec.push(' ');
    char_vec.into_iter().collect()
}

pub fn cgisf(
    adjectives1: i32,
    adverbs: i32,
    structure: i32,
    plural1: bool,
    adjectives2: i32,
    plural2: bool,
) -> String {
    let y = get_structure(
        adjectives1,
        adverbs,
        structure,
        plural1,
        adjectives2,
        plural2,
    );
    let y2: String = y.iter().collect();
    println!("{}", y2);
    let mut final_sentence = String::new();
    for letter in y {
        if letter == 'x' {
            break;
        } else {
            let mut word = get_word(letter);
            word.push(' ');
            final_sentence.push_str(&word);
        }
    }
    string_cleanup(final_sentence)
}
