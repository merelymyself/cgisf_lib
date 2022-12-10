use const_format::{str_replace, str_split};
use once_cell::sync::Lazy;
use rand::Rng;

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

struct Words {
    adverbs: [&'static str; str_split!(include_str!("../wordlists/adverbs.txt"), '\n').len()],
    age_adjectives:
        [&'static str; str_split!(include_str!("../wordlists/age_adjectives.txt"), '\n').len()],
    material_adjectives: [&'static str;
        str_split!(include_str!("../wordlists/material_adjectives.txt"), '\n').len()],
    opinion_adjectives:
        [&'static str; str_split!(include_str!("../wordlists/opinion_adjectives.txt"), '\n').len()],
    size_adjectives:
        [&'static str; str_split!(include_str!("../wordlists/size_adjectives.txt"), '\n').len()],
    color_adjectives:
        [&'static str; str_split!(include_str!("../wordlists/color_adjectives.txt"), '\n').len()],
    nouns: [&'static str; str_split!(include_str!("../wordlists/nouns.txt"), '\n').len()],
    plural_nouns:
        [&'static str; str_split!(include_str!("../wordlists/plural_nouns.txt"), '\n').len()],
    singular_nouns:
        [&'static str; str_split!(include_str!("../wordlists/singular_nouns.txt"), '\n').len()],
    verbs: [&'static str; str_split!(include_str!("../wordlists/verbs.txt"), '\n').len()],
    transitive_verbs:
        [&'static str; str_split!(include_str!("../wordlists/transitive_verbs.txt"), '\n').len()],
}

static WORDS: Lazy<Words> = Lazy::new(|| Words {
    adverbs: str_split!(
        str_replace!(include_str!("../wordlists/adverbs.txt"), '\r', ""),
        '\n'
    ),
    age_adjectives: str_split!(
        str_replace!(include_str!("../wordlists/age_adjectives.txt"), '\r', ""),
        '\n'
    ),
    material_adjectives: str_split!(
        str_replace!(
            include_str!("../wordlists/material_adjectives.txt"),
            '\r',
            ""
        ),
        '\n'
    ),
    opinion_adjectives: str_split!(
        str_replace!(
            include_str!("../wordlists/opinion_adjectives.txt"),
            '\r',
            ""
        ),
        '\n'
    ),
    size_adjectives: str_split!(
        str_replace!(include_str!("../wordlists/size_adjectives.txt"), '\r', ""),
        '\n'
    ),
    color_adjectives: str_split!(
        str_replace!(include_str!("../wordlists/color_adjectives.txt"), '\r', ""),
        '\n'
    ),
    nouns: str_split!(
        str_replace!(include_str!("../wordlists/nouns.txt"), '\r', ""),
        '\n'
    ),
    plural_nouns: str_split!(
        str_replace!(include_str!("../wordlists/plural_nouns.txt"), '\r', ""),
        '\n'
    ),
    singular_nouns: str_split!(
        str_replace!(include_str!("../wordlists/singular_nouns.txt"), '\r', ""),
        '\n'
    ),
    verbs: str_split!(
        str_replace!(include_str!("../wordlists/verbs.txt"), '\r', ""),
        '\n'
    ),
    transitive_verbs: str_split!(
        str_replace!(include_str!("../wordlists/transitive_verbs.txt"), '\r', ""),
        '\n'
    ),
});

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
            WORDS.plural_nouns.as_slice()
        }
        SingularNoun => {
            let (singular_len, general_len) = (WORDS.plural_nouns.len(), WORDS.nouns.len());
            let pos = rng.gen_range(0..singular_len + general_len - 1);
            if pos >= singular_len {
                WORDS.singular_nouns.as_slice()
            } else {
                WORDS.nouns.as_slice()
            }
        }
        Verb => WORDS.verbs.as_slice(),
        Adverb => WORDS.adverbs.as_slice(),
        GeneralNoun => WORDS.nouns.as_slice(),
        OpinionAdjective => WORDS.opinion_adjectives.as_slice(),
        SizeAdjective => WORDS.size_adjectives.as_slice(),
        AgeAdjective => WORDS.age_adjectives.as_slice(),
        ColorAdjective => WORDS.color_adjectives.as_slice(),
        MaterialAdjective => WORDS.material_adjectives.as_slice(),
        TransitiveVerb => WORDS.transitive_verbs.as_slice(),
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
