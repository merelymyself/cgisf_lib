//! A random sentence generator, still without commas.
//!
//! Has three functions:
//! - [`cgisf()`], to generate a sentence;
//! - [`get_word()`], to generate a word of a certain kind; and
//! - [`get_structure()`], to get the structure of a sentence.
//!
//! cgisf is a reference to 'Colourless Green Ideas Sleep Furiously' - a 1957 example of a sentence that is grammatical but makes no sense.
//! Similarly, don't expect these sentences to make that much sense either.
//! # Usage
//!
//! First, add this to your Cargo.toml
//!
//! ```toml
//! [dependencies]
//! cgisf = "0.1.2"
//! ```
//!
//! Then,
//! ```
//! print!("{}", cgisf(2, 1, 1, true, 2, false));
//! ```
//! A deeper explanation can be found in the individual functions' pages.

mod getstructure;
mod getword;
pub use getstructure::get_structure;
pub use getword::get_word;

fn string_cleanup(str: String) -> String {
    let mut char_vec: Vec<char> = str.chars().collect();
    char_vec[0] = char_vec[0].to_ascii_uppercase();
    // It's absolutely ridiculous how much work it takes to convert the first letter of a String to uppercase in Rust.
    char_vec[(str.len() - 1) as usize] = '.';
    // I never want to touch this cleanup function again.
    char_vec.push(' ');
    char_vec.into_iter().collect()
}

/// Takes in 6 arguments, spits out a random sentence.
///
/// # Arguments
/// The first three arguments are non-negative integers. Respectively:
/// - The first argument is used to denote the number of adjectives attached to the subject noun, i.e. Colourless Green
/// - The second argument is used to denote the number of adverbs attached to the verb, i.e. Furiously
/// - The third argument is used to denote the sentence structure. There are four structures available, laid out below.
/// The fourth argument is a boolean, indicating if the subject noun should be plural (true) or not (false).
/// The fifth argument is a non-negative integer, used to denote the number of adjectives attached to the object noun (if present).
/// The sixth argument is a boolean, indicating if the object noun should be plural (true) or not (false).
///
/// # Structures
/// With respect to the integer argument (argument 3):
/// * 1: Colourless Green Ideas Sleep Furiously. (Adjectives-Noun-Verb-Adverbs)
/// * 2: Colourless Green Ideas Furiously Sleep. (Adjectives-Noun-Adverbs-Verb)
/// * 3: Colourless Green Ideas Hit Furiously Red Sheep. (Adjectives-Noun-Verb-Adverbs-Adjectives-Noun) -> Has object noun.
/// * 4: Colourless Green Ideas Furiously Hit Red Sheep. (Adjectives-Noun-Verb-Adverbs-Noun-Adjectives) -> Has object noun.
///
/// # Usage
/// ```
/// print!("{}", cgisf(2, 1, 1, true, 2, false));
/// ```
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
