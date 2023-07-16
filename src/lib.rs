//! A random sentence generator, still without commas.
//!
//! Has three functions:
//! - [`gen_sentence()`], to generate a sentence;
//! - [`gen_word()`], to generate a word of a certain kind; and
//! - [`gen_structure()`], to generate the structure of a sentence based on your SentenceConfig.
//!
//! cgisf is a reference to 'Colourless Green Ideas Sleep Furiously' - a 1957 example of a sentence that is grammatical but makes no sense.
//! Similarly, don't expect these sentences to make that much sense either.
//! # Usage
//!
//! First, add this to your Cargo.toml
//!
//! ```toml
//! [dependencies]
//! cgisf = "0.2.1"
//! ```
//!
//! Then,
//! ```
//! use cgisf_lib::{gen_sentence, SentenceConfigBuilder};
//! print!("{}", gen_sentence(SentenceConfigBuilder::random().build()));
//! ```
//! A deeper explanation can be found in the individual functions' pages.

mod words;
use rand::Rng;
pub use words::{gen_word, WordType};

fn string_cleanup(mut s: String) -> String {
    if let Some(c) = s.get_mut(0..1) {
        let me = unsafe { c.as_bytes_mut() };
        me[0].make_ascii_uppercase()
    };
    let len = s.len().max(1);
    if let Some(x) = s.get_mut(len - 1..len) {
        let me = unsafe { x.as_bytes_mut() };
        me[0] = b'.'
    };
    s
}

/// The structure of the sentence is one of these four options.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Structure {
    /// A sentence like: "Colourless Green Ideas Sleep Furiously".
    AdjectivesNounVerbAdverbs,
    /// A sentence like: "Colourless Green Ideas Furiously Sleep".
    AdjectivesNounAdverbsVerb,
    /// A sentence like: "Colourless Green Ideas Hit Furiously Red Sheep".
    AdjectivesNounVerbAdverbsAdjectivesNoun,
    /// A sentence like: "Colourless Green Ideas Furiously Hit Red Sheep".
    AdjectivesNounVerbAdverbsNounAdjectives,
}

/// The actual structure of the config. To build it, use [`SentenceConfigBuilder`].
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct SentenceConfig {
    adjectives: u16,       // The number of adjectives attached to the noun
    adverbs: u16,          // The number of adverbs attached to the verb
    structure: Structure,  // The sentence structure. There are four options.
    plural: bool,          // Whether the noun should be plural
    count_adjectives: u16, // The number of adjectives attached to the object noun(if present).
    on_plural: bool,       // Whether the object noun should be plural
}

/// The builder for [`SentenceConfig`].
///
/// By default, the config creates a sentence with two adjectives, one adverb, and plural nouns
/// following the "AdjectivesNounVerbAdverbs" [`Structure`].
///
/// # Example
/// ```
/// use cgisf_lib::SentenceConfigBuilder;
///
/// let config = SentenceConfigBuilder::new()
///     .adjectives(3)
///     .adverbs(2)
///     .plural(false)
///     .build();
/// ```
pub struct SentenceConfigBuilder {
    adjectives: u16,      // The number of adjectives attached to the noun
    adverbs: u16,         // The number of adverbs attached to the verb
    structure: Structure, // The sentence structure. There are four options.
    plural: bool,         // Whether the noun should be plural
    on_adjectives: u16,   // The number of adjectives attached to the object noun(if present).
    on_plural: bool,      // Whether the object noun should be plural
}

impl SentenceConfigBuilder {
    pub fn new() -> Self {
        SentenceConfigBuilder::default()
    }
    pub fn random() -> Self {
        use Structure::*;
        let mut rng = rand::thread_rng();
        SentenceConfigBuilder {
            adjectives: rng.gen_range(1..=3),
            adverbs: rng.gen_range(0..=2),
            structure: {
                let options = [
                    AdjectivesNounVerbAdverbs,
                    AdjectivesNounAdverbsVerb,
                    AdjectivesNounVerbAdverbsAdjectivesNoun,
                    AdjectivesNounVerbAdverbsNounAdjectives,
                ];
                options[rng.gen_range(0..options.len())]
            },
            plural: rng.gen_range(0..=1) == 1,
            on_adjectives: rng.gen_range(1..=3),
            on_plural: rng.gen_range(0..=1) == 1,
        }
    }
    pub fn adjectives(mut self, n: u16) -> Self {
        self.adjectives = n;
        self
    }
    pub fn adverbs(mut self, n: u16) -> Self {
        self.adverbs = n;
        self
    }
    pub fn structure(mut self, n: Structure) -> Self {
        self.structure = n;
        self
    }
    pub fn plural(mut self, n: bool) -> Self {
        self.plural = n;
        self
    }
    /// The number of adjectives attached to the object noun. This only has an effect if the
    /// [`Structure`] is "AdjectivesNounVerbAdverbsNounAdjectives" or
    /// "AdjectivesNounVerbAdverbsAdjectivesNoun"
    pub fn on_adjectives(mut self, n: u16) -> Self {
        self.on_adjectives = n;
        self
    }
    /// Whether the object noun is plural or not. This only has an effect if the
    /// [`Structure`] is "AdjectivesNounVerbAdverbsNounAdjectives" or
    /// "AdjectivesNounVerbAdverbsAdjectivesNoun"
    pub fn on_plural(mut self, n: bool) -> Self {
        self.on_plural = n;
        self
    }
    pub fn build(self) -> SentenceConfig {
        let Self {
            adjectives,
            adverbs,
            structure,
            plural,
            on_adjectives,
            on_plural,
        } = self;
        SentenceConfig {
            adjectives,
            adverbs,
            structure,
            plural,
            count_adjectives: on_adjectives,
            on_plural,
        }
    }
}

impl Default for SentenceConfigBuilder {
    fn default() -> Self {
        SentenceConfigBuilder {
            adjectives: 2,
            adverbs: 1,
            structure: Structure::AdjectivesNounVerbAdverbs,
            plural: true,
            on_adjectives: 2,
            on_plural: false,
        }
    }
}

pub fn gen_sentence(config: SentenceConfig) -> String {
    let tokens = gen_structure(config);
    let mut sentence = String::with_capacity((tokens.len() + 1) * 5);
    for token in tokens {
        sentence.push_str(&gen_word(token));
        sentence.push(' ');
    }
    sentence = string_cleanup(sentence);
    sentence
}

pub fn gen_structure(config: SentenceConfig) -> Vec<WordType> {
    use Structure::*;
    let words = (config.adverbs + config.adjectives + config.count_adjectives) as usize + 3;
    let mut tokens: Vec<WordType> = Vec::with_capacity(words);
    if !config.plural || rand::thread_rng().gen_range(0..=1) == 1 {
        tokens.push(WordType::The)
    }
    WordType::adjective_mul(&mut tokens, config.adjectives);
    // Adding adjectives before the noun
    tokens.push(if config.plural {
        WordType::PluralNoun
    } else {
        WordType::SingularNoun
    });
    // The noun itself
    match config.structure {
        AdjectivesNounVerbAdverbs => {
            tokens.push(if config.plural {
                WordType::Verb
            } else {
                WordType::NoSingularVerb
            }); // Adding the verb
            for _ in 0..config.adverbs {
                tokens.push(WordType::Adverb)
            } // Adding the adverbs
        }
        AdjectivesNounAdverbsVerb => {
            for _ in 0..config.adverbs {
                tokens.push(WordType::Adverb)
            } // Adding the adverbs
            tokens.push(if config.plural {
                WordType::Verb
            } else {
                WordType::NoSingularVerb
            }); // Adding the verb
        }
        AdjectivesNounVerbAdverbsAdjectivesNoun => {
            tokens.push(if config.plural {
                WordType::Verb
            } else {
                WordType::NoSingularVerb
            }); // Adding the verb
            for _ in 0..config.adverbs {
                tokens.push(WordType::Adverb)
            } // Adding the adverbs
            if !config.on_plural || rand::thread_rng().gen_range(0..=1) == 1 {
                tokens.push(WordType::The)
            }
            WordType::adjective_mul(&mut tokens, config.adjectives);
            tokens.push(if config.plural {
                WordType::PluralNoun
            } else {
                WordType::SingularNoun
            });
        }
        AdjectivesNounVerbAdverbsNounAdjectives => {
            for _ in 0..config.adverbs {
                tokens.push(WordType::Adverb)
            } // Adding the adverbs
            tokens.push(if config.plural {
                WordType::Verb
            } else {
                WordType::NoSingularVerb
            }); // Adding the verb
            if !config.on_plural || rand::thread_rng().gen_range(0..=1) == 1 {
                tokens.push(WordType::The)
            }
            WordType::adjective_mul(&mut tokens, config.count_adjectives);
            tokens.push(if config.plural {
                WordType::PluralNoun
            } else {
                WordType::SingularNoun
            });
        }
    };
    tokens
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn show_result() {
        let sc = SentenceConfigBuilder::default().build();
        let sentence = gen_sentence(sc);
        assert!(sentence.chars().next().unwrap().is_uppercase());
        assert_eq!(sentence.chars().last().unwrap(), '.');
        println!("{}", sentence);
    }
}
