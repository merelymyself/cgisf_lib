use once_cell::sync::Lazy;
use rand::Rng;

struct Words {
    adverbs: Vec<&'static str>,
    age_adjectives: Vec<&'static str>,
    material_adjectives: Vec<&'static str>,
    opinion_adjectives: Vec<&'static str>,
    size_adjectives: Vec<&'static str>,
    color_adjectives: Vec<&'static str>,
    nouns: Vec<&'static str>,
    plural_nouns: Vec<&'static str>,
    singular_nouns: Vec<&'static str>,
    verbs: Vec<&'static str>,
    transitive_verbs: Vec<&'static str>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum WordType {
    GeneralNoun,
    SingularNoun,
    PluralNoun,
    OpinionAdjective,
    ColorAdjective,
    SizeAdjective,
    AgeAdjective,
    MaterialAdjective,
    TransitiveVerb,
    NoSingularVerb,
    Verb,
    Adverb,
    The,
}

impl WordType {
    pub fn any() -> Self {
        use WordType::*;
        let mut rng = rand::thread_rng();
        let possible = [
            GeneralNoun,
            SingularNoun,
            PluralNoun,
            OpinionAdjective,
            ColorAdjective,
            SizeAdjective,
            AgeAdjective,
            MaterialAdjective,
            TransitiveVerb,
            NoSingularVerb,
            Verb,
            Adverb,
        ];
        possible[rng.gen_range(0..possible.len())]
    }
    pub fn any_adjective() -> Self {
        use WordType::*;
        let mut rng = rand::thread_rng();
        let possible = [
            OpinionAdjective,
            ColorAdjective,
            SizeAdjective,
            AgeAdjective,
            MaterialAdjective,
        ];
        possible[rng.gen_range(0..possible.len())]
    }
    pub fn adjective_mul(n: u16) -> Vec<Self> {
        use WordType::*;
        let mut rng = rand::thread_rng();
        let mut adjectives: Vec<WordType> = Vec::with_capacity(n as usize);
        let order = [
            OpinionAdjective,
            ColorAdjective,
            SizeAdjective,
            AgeAdjective,
            MaterialAdjective,
        ];
        let len = order.len();
        for _ in 0..n {
            adjectives.push(order[rng.gen_range(0..len)]);
        }
        adjectives.sort_by_key(|x| order.iter().position(|c| c == x));
        adjectives
    }
}

static WORDS: Lazy<Words> = Lazy::new(|| Words {
    adverbs: include_str!("../wordlists/adverbs.txt").lines().collect(),
    age_adjectives: include_str!("../wordlists/age_adjectives.txt")
        .lines()
        .collect(),
    material_adjectives: include_str!("../wordlists/material_adjectives.txt")
        .lines()
        .collect(),
    opinion_adjectives: include_str!("../wordlists/opinion_adjectives.txt")
        .lines()
        .collect(),
    size_adjectives: include_str!("../wordlists/size_adjectives.txt")
        .lines()
        .collect(),
    color_adjectives: include_str!("../wordlists/color_adjectives.txt")
        .lines()
        .collect(),
    nouns: include_str!("../wordlists/nouns.txt").lines().collect(),
    plural_nouns: include_str!("../wordlists/plural_nouns.txt")
        .lines()
        .collect(),
    singular_nouns: include_str!("../wordlists/singular_nouns.txt")
        .lines()
        .collect(),
    verbs: include_str!("../wordlists/verbs.txt").lines().collect(),
    transitive_verbs: include_str!("../wordlists/transitive_verbs.txt")
        .lines()
        .collect(),
});

/*
The full linkage of characters and what they represent:
n - nouns (general), made plural by adding an 's' at the end.
s - nouns (singular), only exist in singular form
p - nouns (plural), only exist in plural form
O - opinion adjective
S - size adjective
A - age adjective
C - colour adjective
M - material adjective
t - transitive verbs, aka noun verb noun.
v - verbs, aka noun verb.
a - adverbs
0 - 'the', couldn't think of any other way to implement it.
V - verb, except for the singular form - aka 'coders code' vs 'coder codes': this is the latter
 */

#[inline]
pub fn gen_word(wordtype: WordType) -> String {
    use WordType::*;
    let mut rng = rand::thread_rng();
    let wordlist = match wordtype {
        NoSingularVerb => {
            let word = unsafe {
                WORDS
                    .verbs
                    .get_unchecked(rng.gen_range(0..WORDS.verbs.len()))
                    .to_string()
            };
            return pluralize(word);
        }
        PluralNoun => {
            let (plural_len, singular_len) = (WORDS.plural_nouns.len(), WORDS.singular_nouns.len());
            let pos = rng.gen_range(0..plural_len + singular_len - 1);
            if pos >= plural_len {
                let word = unsafe {
                    WORDS
                        .singular_nouns
                        .get_unchecked(pos % plural_len)
                        .to_string()
                };
                return pluralize(word);
            }
            &WORDS.plural_nouns
        }
        SingularNoun => {
            let (singular_len, general_len) = (WORDS.plural_nouns.len(), WORDS.nouns.len());
            let pos = rng.gen_range(0..singular_len + general_len - 1);
            if pos >= singular_len {
                &WORDS.singular_nouns
            } else {
                &WORDS.nouns
            }
        }
        Verb => &WORDS.verbs,
        Adverb => &WORDS.adverbs,
        GeneralNoun => &WORDS.nouns,
        OpinionAdjective => &WORDS.opinion_adjectives,
        SizeAdjective => &WORDS.size_adjectives,
        AgeAdjective => &WORDS.age_adjectives,
        ColorAdjective => &WORDS.color_adjectives,
        MaterialAdjective => &WORDS.material_adjectives,
        TransitiveVerb => &WORDS.transitive_verbs,
        The => return "the".to_string(),
    };
    unsafe {
        wordlist
            .get_unchecked(rng.gen_range(0..wordlist.len())) // using unchecked as we cannot
            // ever try to get an index out of bounds
            .to_string()
    }
}

fn pluralize(mut word: String) -> String {
    let len = word.len();
    if let Some(suffix) = word.get(len - 2..len) {
        if suffix == "sh" || ['x', 's', 'o'].into_iter().any(|c| suffix.ends_with(c)) {
            // rush -> rushes, box -> boxes, go -> goes, bless -> blesses
            word.push('e');
        } else if suffix.ends_with('y') {
            word.pop();
            word.push_str("ie")
        }
    }
    word.push('s');
    word
}
