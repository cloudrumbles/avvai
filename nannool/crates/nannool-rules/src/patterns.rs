//! Pattern matching for sandhi rule contexts.
//!
//! Defines the left (ending) and right (beginning) contexts
//! for sandhi rule matching.

use serde::{Deserialize, Serialize};
use tamil_unicode::{
    grapheme::{TamilGrapheme, get_final_grapheme, get_initial_grapheme},
    letters::*,
    transform::*,
};

/// Left context - what the first word must end with
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum LeftContext {
    /// Any vowel ending
    AnyUyir,
    /// Specific vowel(s)
    SpecificUyir { letters: Vec<char> },
    /// Short vowel (குறில்)
    Kuril,
    /// Long vowel (நெடில்)
    Nedil,
    /// Any consonant ending
    AnyMei,
    /// Specific consonant(s)
    SpecificMei { letters: Vec<String> },
    /// Vallinam consonants
    Vallinam,
    /// Mellinam consonants
    Mellinam,
    /// Idaiyinam consonants
    Idaiyinam,
    /// Specific word(s)
    SpecificWord { words: Vec<String> },
    /// Pattern: short vowel + consonant (for ஒற்று இரட்டல்)
    KurilPlusMei,
    /// Any ending
    Any,
}

/// Right context - what the second word must start with
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum RightContext {
    /// Any vowel beginning
    AnyUyir,
    /// Specific vowel(s)
    SpecificUyir { letters: Vec<char> },
    /// Any consonant beginning
    AnyMei,
    /// Vallinam consonants (க, ச, ட, த, ப, ற)
    Vallinam,
    /// Mellinam consonants
    Mellinam,
    /// Idaiyinam consonants
    Idaiyinam,
    /// Specific consonant(s)
    SpecificMei { letters: Vec<String> },
    /// Any beginning
    Any,
}

impl LeftContext {
    /// Check if a word ending matches this context
    pub fn matches(&self, word: &str) -> bool {
        let final_grapheme = match get_final_grapheme(word) {
            Some(g) => g,
            None => return false,
        };

        match self {
            LeftContext::AnyUyir => final_grapheme.ends_with_vowel(),

            LeftContext::SpecificUyir { letters } => {
                final_grapheme.final_vowel().is_some_and(|v| letters.contains(&v))
            }

            LeftContext::Kuril => {
                final_grapheme.final_vowel().is_some_and(is_kuril)
            }

            LeftContext::Nedil => {
                final_grapheme.final_vowel().is_some_and(is_nedil)
            }

            LeftContext::AnyMei => final_grapheme.ends_with_consonant(),

            LeftContext::SpecificMei { letters } => {
                if let Some(base) = final_grapheme.consonant_base() {
                    let mei = format!("{}்", base);
                    letters.contains(&mei)
                } else {
                    false
                }
            }

            LeftContext::Vallinam => {
                final_grapheme.ends_with_consonant() && final_grapheme.is_vallinam()
            }

            LeftContext::Mellinam => {
                final_grapheme.ends_with_consonant() && final_grapheme.is_mellinam()
            }

            LeftContext::Idaiyinam => {
                final_grapheme.ends_with_consonant() && final_grapheme.is_idaiyinam()
            }

            LeftContext::SpecificWord { words } => words.iter().any(|w| word == w),

            LeftContext::KurilPlusMei => {
                // Check if word ends with: short vowel + consonant
                // e.g., "கல்" (க + ல்) where the previous letter has kuril
                let graphemes = tamil_unicode::grapheme::get_graphemes(word);
                if graphemes.len() >= 2 {
                    let last = &graphemes[graphemes.len() - 1];
                    let second_last = &graphemes[graphemes.len() - 2];

                    // Last must be mei (consonant)
                    // Second last must end with kuril
                    last.ends_with_consonant()
                        && second_last.final_vowel().is_some_and(is_kuril)
                } else {
                    false
                }
            }

            LeftContext::Any => true,
        }
    }

    /// Create a LeftContext for specific vowels
    pub fn specific_uyir(vowels: &[char]) -> Self {
        LeftContext::SpecificUyir {
            letters: vowels.to_vec(),
        }
    }

    /// Create a LeftContext for specific consonants
    pub fn specific_mei(consonants: &[&str]) -> Self {
        LeftContext::SpecificMei {
            letters: consonants.iter().map(|s| s.to_string()).collect(),
        }
    }
}

impl RightContext {
    /// Check if a word beginning matches this context
    pub fn matches(&self, word: &str) -> bool {
        let initial_grapheme = match get_initial_grapheme(word) {
            Some(g) => g,
            None => return false,
        };

        match self {
            RightContext::AnyUyir => {
                matches!(initial_grapheme, TamilGrapheme::Uyir(_))
            }

            RightContext::SpecificUyir { letters } => {
                if let TamilGrapheme::Uyir(v) = initial_grapheme {
                    letters.contains(&v)
                } else {
                    false
                }
            }

            RightContext::AnyMei => initial_grapheme.consonant_base().is_some(),

            RightContext::Vallinam => initial_grapheme.is_vallinam(),

            RightContext::Mellinam => initial_grapheme.is_mellinam(),

            RightContext::Idaiyinam => initial_grapheme.is_idaiyinam(),

            RightContext::SpecificMei { letters } => {
                if let Some(base) = initial_grapheme.consonant_base() {
                    let mei = format!("{}்", base);
                    letters.contains(&mei)
                } else {
                    false
                }
            }

            RightContext::Any => true,
        }
    }

    /// Create a RightContext for specific vowels
    pub fn specific_uyir(vowels: &[char]) -> Self {
        RightContext::SpecificUyir {
            letters: vowels.to_vec(),
        }
    }

    /// Create a RightContext for specific consonants
    pub fn specific_mei(consonants: &[&str]) -> Self {
        RightContext::SpecificMei {
            letters: consonants.iter().map(|s| s.to_string()).collect(),
        }
    }
}

/// Combined context for a sandhi rule
#[derive(Debug, Clone)]
pub struct SandhiContext {
    pub left: LeftContext,
    pub right: RightContext,
}

impl SandhiContext {
    pub fn new(left: LeftContext, right: RightContext) -> Self {
        Self { left, right }
    }

    /// Check if a word pair matches this context
    pub fn matches(&self, word1: &str, word2: &str) -> bool {
        self.left.matches(word1) && self.right.matches(word2)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_left_context_any_uyir() {
        let ctx = LeftContext::AnyUyir;
        assert!(ctx.matches("பாடு")); // ends with உ
        assert!(ctx.matches("வா"));   // ends with ஆ
        assert!(!ctx.matches("தமிழ்")); // ends with ழ்
    }

    #[test]
    fn test_left_context_specific_uyir() {
        let ctx = LeftContext::specific_uyir(&['இ', 'ஈ', 'ஐ']);
        assert!(ctx.matches("படி")); // ends with இ
        assert!(!ctx.matches("படு")); // ends with உ
    }

    #[test]
    fn test_left_context_kuril() {
        let ctx = LeftContext::Kuril;
        assert!(ctx.matches("படு")); // ends with உ (kuril)
        assert!(!ctx.matches("படூ")); // ends with ஊ (nedil)
    }

    #[test]
    fn test_left_context_any_mei() {
        let ctx = LeftContext::AnyMei;
        assert!(ctx.matches("தமிழ்")); // ends with ழ்
        assert!(!ctx.matches("பாடு")); // ends with உ
    }

    #[test]
    fn test_left_context_kuril_plus_mei() {
        let ctx = LeftContext::KurilPlusMei;
        assert!(ctx.matches("கல்")); // க (with அ kuril) + ல்
        assert!(ctx.matches("மரம்")); // ர (with அ kuril) + ம்
        assert!(!ctx.matches("காற்")); // கா (nedil) + ற்
    }

    #[test]
    fn test_right_context_vallinam() {
        let ctx = RightContext::Vallinam;
        assert!(ctx.matches("கண்")); // starts with க
        assert!(ctx.matches("பாடு")); // starts with ப
        assert!(!ctx.matches("மாடு")); // starts with ம (mellinam)
        assert!(!ctx.matches("அழகு")); // starts with அ (vowel)
    }

    #[test]
    fn test_right_context_any_uyir() {
        let ctx = RightContext::AnyUyir;
        assert!(ctx.matches("அழகு")); // starts with vowel
        assert!(ctx.matches("இது")); // starts with vowel
        assert!(!ctx.matches("பாடு")); // starts with consonant
    }

    #[test]
    fn test_sandhi_context() {
        let ctx = SandhiContext::new(LeftContext::AnyUyir, RightContext::Vallinam);
        assert!(ctx.matches("பாடு", "பாடினான்")); // vowel ending + vallinam beginning
        assert!(!ctx.matches("தமிழ்", "பாடினான்")); // consonant ending
        assert!(!ctx.matches("பாடு", "மாடு")); // mellinam beginning
    }
}
