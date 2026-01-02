//! Tamil letter transformations and classifications.
//!
//! This module provides functions for:
//! - Splitting uyirmei into mei and uyir components
//! - Combining mei and uyir into uyirmei
//! - Letter class determination

use crate::grapheme::{TamilGrapheme, get_graphemes, graphemes_to_string};
use crate::letters::*;
use crate::unicode_ranges::PULLI;

/// Vowel length classification
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum VowelLength {
    /// Short vowel (குறில்) - 1 mātra
    Short,
    /// Long vowel (நெடில்) - 2 mātra
    Long,
}

/// Consonant class classification
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ConsonantClass {
    /// Hard consonants (வல்லினம்): க், ச், ட், த், ப், ற்
    Vallinam,
    /// Soft/nasal consonants (மெல்லினம்): ங், ஞ், ண், ந், ம், ன்
    Mellinam,
    /// Medium consonants (இடையினம்): ய், ர், ல், வ், ழ், ள்
    Idaiyinam,
    /// Grantha consonants: ஜ், ஷ், ஸ், ஹ்
    Grantha,
}

/// Complete letter classification
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LetterClass {
    /// Pure vowel
    Uyir(VowelLength),
    /// Pure consonant
    Mei(ConsonantClass),
    /// Combined consonant-vowel
    UyirMei {
        consonant: ConsonantClass,
        vowel: VowelLength,
    },
    /// Aaytham
    Aaytham,
    /// Other (non-Tamil)
    Other,
}

/// Get the vowel length for a vowel character
pub fn get_vowel_length(vowel: char) -> Option<VowelLength> {
    if is_kuril(vowel) {
        Some(VowelLength::Short)
    } else if is_nedil(vowel) {
        Some(VowelLength::Long)
    } else {
        None
    }
}

/// Get the consonant class for a consonant base character
pub fn get_consonant_class(consonant_base: char) -> Option<ConsonantClass> {
    if VALLINAM_BASE.contains(&consonant_base) {
        Some(ConsonantClass::Vallinam)
    } else if MELLINAM_BASE.contains(&consonant_base) {
        Some(ConsonantClass::Mellinam)
    } else if IDAIYINAM_BASE.contains(&consonant_base) {
        Some(ConsonantClass::Idaiyinam)
    } else if GRANTHA_BASE.contains(&consonant_base) {
        Some(ConsonantClass::Grantha)
    } else {
        None
    }
}

/// Classify a Tamil grapheme
pub fn classify_grapheme(grapheme: &TamilGrapheme) -> LetterClass {
    match grapheme {
        TamilGrapheme::Uyir(v) => {
            let length = get_vowel_length(*v).unwrap_or(VowelLength::Short);
            LetterClass::Uyir(length)
        }
        TamilGrapheme::Mei(s) => {
            let base = s.chars().next().unwrap();
            let class = get_consonant_class(base).unwrap_or(ConsonantClass::Vallinam);
            LetterClass::Mei(class)
        }
        TamilGrapheme::UyirMei { mei_base, uyir } => {
            let consonant = get_consonant_class(*mei_base).unwrap_or(ConsonantClass::Vallinam);
            let vowel = get_vowel_length(*uyir).unwrap_or(VowelLength::Short);
            LetterClass::UyirMei { consonant, vowel }
        }
        TamilGrapheme::Aaytham => LetterClass::Aaytham,
        TamilGrapheme::Grantha { base: _, vowel } => {
            if let Some(v) = vowel {
                let vowel_len = get_vowel_length(*v).unwrap_or(VowelLength::Short);
                LetterClass::UyirMei {
                    consonant: ConsonantClass::Grantha,
                    vowel: vowel_len,
                }
            } else {
                LetterClass::Mei(ConsonantClass::Grantha)
            }
        }
        TamilGrapheme::Digit(_) | TamilGrapheme::Other(_) => LetterClass::Other,
    }
}

/// Split an uyirmei grapheme into its mei and uyir components
///
/// # Example
/// ```
/// use tamil_unicode::transform::split_uyirmei;
///
/// let (mei, uyir) = split_uyirmei("கா").unwrap();
/// assert_eq!(mei, "க்");
/// assert_eq!(uyir, 'ஆ');
/// ```
pub fn split_uyirmei(uyirmei: &str) -> Option<(String, char)> {
    let graphemes = get_graphemes(uyirmei);
    if graphemes.len() != 1 {
        return None;
    }

    match &graphemes[0] {
        TamilGrapheme::UyirMei { mei_base, uyir } => {
            let mei = format!("{}{}", mei_base, PULLI);
            Some((mei, *uyir))
        }
        TamilGrapheme::Grantha { base, vowel: Some(v) } => {
            let mei = format!("{}{}", base, PULLI);
            Some((mei, *v))
        }
        _ => None,
    }
}

/// Combine a mei and uyir into an uyirmei
///
/// # Example
/// ```
/// use tamil_unicode::transform::join_mei_uyir;
///
/// let combined = join_mei_uyir("க்", 'ஆ').unwrap();
/// assert_eq!(combined, "கா");
/// ```
pub fn join_mei_uyir(mei: &str, uyir: char) -> Option<String> {
    // Validate mei (should be consonant + pulli)
    let chars: Vec<char> = mei.chars().collect();
    if chars.len() != 2 || chars[1] != PULLI {
        return None;
    }

    let base = chars[0];
    if !is_mei_base(base) && !is_grantha_base(base) {
        return None;
    }

    // Validate uyir
    if !is_uyir(uyir) {
        return None;
    }

    // Combine
    let mut result = base.to_string();
    if let Some(sign) = vowel_to_sign(uyir) {
        result.push(sign);
    }
    // If uyir is 'அ', no sign needed (inherent vowel)

    Some(result)
}

/// Get the mei (pure consonant) form of a consonant
pub fn to_mei(consonant_base: char) -> Option<String> {
    if is_mei_base(consonant_base) || is_grantha_base(consonant_base) {
        Some(format!("{}{}", consonant_base, PULLI))
    } else {
        None
    }
}

/// Get the base consonant from a mei string
pub fn mei_to_base(mei: &str) -> Option<char> {
    let chars: Vec<char> = mei.chars().collect();
    if chars.len() == 2 && chars[1] == PULLI {
        Some(chars[0])
    } else {
        None
    }
}

/// Check if a word ends with a short vowel (kuril)
pub fn ends_with_kuril(word: &str) -> bool {
    let graphemes = get_graphemes(word);
    graphemes.last().is_some_and(|g| match g {
        TamilGrapheme::Uyir(v) => is_kuril(*v),
        TamilGrapheme::UyirMei { uyir, .. } => is_kuril(*uyir),
        TamilGrapheme::Grantha { vowel: Some(v), .. } => is_kuril(*v),
        _ => false,
    })
}

/// Check if a word ends with a long vowel (nedil)
pub fn ends_with_nedil(word: &str) -> bool {
    let graphemes = get_graphemes(word);
    graphemes.last().is_some_and(|g| match g {
        TamilGrapheme::Uyir(v) => is_nedil(*v),
        TamilGrapheme::UyirMei { uyir, .. } => is_nedil(*uyir),
        TamilGrapheme::Grantha { vowel: Some(v), .. } => is_nedil(*v),
        _ => false,
    })
}

/// Check if a word ends with a vowel
pub fn ends_with_vowel(word: &str) -> bool {
    let graphemes = get_graphemes(word);
    graphemes.last().is_some_and(|g| g.ends_with_vowel())
}

/// Check if a word ends with a consonant
pub fn ends_with_consonant(word: &str) -> bool {
    let graphemes = get_graphemes(word);
    graphemes.last().is_some_and(|g| g.ends_with_consonant())
}

/// Check if a word starts with a vallinam consonant
pub fn starts_with_vallinam(word: &str) -> bool {
    let graphemes = get_graphemes(word);
    graphemes.first().is_some_and(|g| g.is_vallinam())
}

/// Get the final vowel of a word
pub fn get_final_vowel(word: &str) -> Option<char> {
    let graphemes = get_graphemes(word);
    graphemes.last()?.final_vowel()
}

/// Get the final consonant base of a word (if it ends with a consonant)
pub fn get_final_consonant(word: &str) -> Option<char> {
    let graphemes = get_graphemes(word);
    let last = graphemes.last()?;
    if last.ends_with_consonant() {
        last.consonant_base()
    } else {
        None
    }
}

/// Get the initial consonant base of a word (if it starts with a consonant)
pub fn get_initial_consonant(word: &str) -> Option<char> {
    let graphemes = get_graphemes(word);
    graphemes.first()?.consonant_base()
}

/// Double the initial consonant of a word (for vallinam miguthal)
///
/// # Example
/// ```
/// use tamil_unicode::transform::double_initial_consonant;
///
/// let result = double_initial_consonant("பாடு").unwrap();
/// assert_eq!(result, "ப்பாடு");
/// ```
pub fn double_initial_consonant(word: &str) -> Option<String> {
    let graphemes = get_graphemes(word);
    if graphemes.is_empty() {
        return None;
    }

    let first = &graphemes[0];
    let base = first.consonant_base()?;

    // Create the doubled form: mei + original
    let mei = to_mei(base)?;
    let mut result = mei;
    for g in &graphemes {
        result.push_str(&g.as_str());
    }

    Some(result)
}

/// Remove the final letter from a word
pub fn remove_final_letter(word: &str) -> Option<String> {
    let mut graphemes = get_graphemes(word);
    if graphemes.is_empty() {
        return None;
    }
    graphemes.pop();
    Some(graphemes.iter().map(|g| g.as_str()).collect())
}

/// Append a letter/grapheme to a word
pub fn append_letter(word: &str, letter: &str) -> String {
    format!("{}{}", word, letter)
}

/// Combine two words, joining final consonant and initial vowel if applicable
pub fn combine_words(word1: &str, word2: &str) -> String {
    let graphemes1 = get_graphemes(word1);
    let graphemes2 = get_graphemes(word2);

    if graphemes1.is_empty() {
        return word2.to_string();
    }
    if graphemes2.is_empty() {
        return word1.to_string();
    }

    let last = &graphemes1[graphemes1.len() - 1];
    let first = &graphemes2[0];

    if last.ends_with_consonant() {
        if let TamilGrapheme::Uyir(v) = first {
            if let Some(base) = last.consonant_base() {
                let mei = format!("{}்", base);
                if let Some(combined) = join_mei_uyir(&mei, *v) {
                    let mut result = graphemes_to_string(&graphemes1[..graphemes1.len() - 1]);
                    result.push_str(&combined);
                    result.push_str(&graphemes_to_string(&graphemes2[1..]));
                    return result;
                }
            }
        }
    }

    format!("{}{}", word1, word2)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_vowel_length() {
        assert_eq!(get_vowel_length('அ'), Some(VowelLength::Short));
        assert_eq!(get_vowel_length('ஆ'), Some(VowelLength::Long));
        assert_eq!(get_vowel_length('இ'), Some(VowelLength::Short));
        assert_eq!(get_vowel_length('ஐ'), Some(VowelLength::Long));
    }

    #[test]
    fn test_get_consonant_class() {
        assert_eq!(get_consonant_class('க'), Some(ConsonantClass::Vallinam));
        assert_eq!(get_consonant_class('ங'), Some(ConsonantClass::Mellinam));
        assert_eq!(get_consonant_class('ய'), Some(ConsonantClass::Idaiyinam));
        assert_eq!(get_consonant_class('ஜ'), Some(ConsonantClass::Grantha));
    }

    #[test]
    fn test_classify_grapheme() {
        let graphemes = get_graphemes("அ");
        assert!(matches!(
            classify_grapheme(&graphemes[0]),
            LetterClass::Uyir(VowelLength::Short)
        ));

        let graphemes = get_graphemes("க்");
        assert!(matches!(
            classify_grapheme(&graphemes[0]),
            LetterClass::Mei(ConsonantClass::Vallinam)
        ));

        let graphemes = get_graphemes("கா");
        assert!(matches!(
            classify_grapheme(&graphemes[0]),
            LetterClass::UyirMei {
                consonant: ConsonantClass::Vallinam,
                vowel: VowelLength::Long
            }
        ));
    }

    #[test]
    fn test_split_uyirmei() {
        let result = split_uyirmei("கா");
        assert_eq!(result, Some(("க்".to_string(), 'ஆ')));

        let result = split_uyirmei("தி");
        assert_eq!(result, Some(("த்".to_string(), 'இ')));

        // Inherent vowel
        let result = split_uyirmei("க");
        assert_eq!(result, Some(("க்".to_string(), 'அ')));

        // Pure mei - can't split
        let result = split_uyirmei("க்");
        assert_eq!(result, None);
    }

    #[test]
    fn test_join_mei_uyir() {
        let result = join_mei_uyir("க்", 'ஆ');
        assert_eq!(result, Some("கா".to_string()));

        let result = join_mei_uyir("த்", 'இ');
        assert_eq!(result, Some("தி".to_string()));

        let result = join_mei_uyir("க்", 'அ');
        assert_eq!(result, Some("க".to_string()));
    }

    #[test]
    fn test_ends_with_kuril_nedil() {
        assert!(ends_with_kuril("பாடு"));  // ends with உ (kuril)
        assert!(!ends_with_kuril("பாடூ")); // ends with ஊ (nedil)

        assert!(ends_with_nedil("பாடூ"));
        assert!(!ends_with_nedil("பாடு"));
    }

    #[test]
    fn test_ends_with_vowel_consonant() {
        assert!(ends_with_vowel("பாடு"));
        assert!(!ends_with_vowel("தமிழ்"));

        assert!(ends_with_consonant("தமிழ்"));
        assert!(!ends_with_consonant("பாடு"));
    }

    #[test]
    fn test_starts_with_vallinam() {
        assert!(starts_with_vallinam("கண்"));
        assert!(starts_with_vallinam("பாடு"));
        assert!(!starts_with_vallinam("மாடு"));
        assert!(!starts_with_vallinam("அழகு"));
    }

    #[test]
    fn test_double_initial_consonant() {
        let result = double_initial_consonant("பாடு");
        assert_eq!(result, Some("ப்பாடு".to_string()));

        let result = double_initial_consonant("கண்");
        assert_eq!(result, Some("க்கண்".to_string()));
    }

    #[test]
    fn test_remove_final_letter() {
        let result = remove_final_letter("தமிழ்");
        assert_eq!(result, Some("தமி".to_string()));

        let result = remove_final_letter("க");
        assert_eq!(result, Some("".to_string()));
    }

    #[test]
    fn test_get_final_vowel() {
        assert_eq!(get_final_vowel("பாடு"), Some('உ'));
        assert_eq!(get_final_vowel("பாடா"), Some('ஆ'));
        assert_eq!(get_final_vowel("தமிழ்"), None);
    }

    #[test]
    fn test_get_initial_consonant() {
        assert_eq!(get_initial_consonant("கண்"), Some('க'));
        assert_eq!(get_initial_consonant("அழகு"), None);
    }
}
