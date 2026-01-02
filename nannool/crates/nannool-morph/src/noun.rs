//! Noun morphology.
//!
//! Tamil noun features based on Nannool's பெயரியல்.

use serde::{Deserialize, Serialize};
use crate::analyzer::{Case, Number, Gender};

/// Noun features
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NounFeatures {
    /// Case (வேற்றுமை)
    pub case: Case,
    /// Number (எண்)
    pub number: Number,
    /// Gender (பால்) - optional for inanimate
    pub gender: Option<Gender>,
}

impl NounFeatures {
    pub fn new(case: Case, number: Number) -> Self {
        Self {
            case,
            number,
            gender: None,
        }
    }

    pub fn with_gender(mut self, gender: Gender) -> Self {
        self.gender = Some(gender);
        self
    }
}

/// Noun class based on ending
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NounClass {
    /// Ends with அம் (e.g., மரம், புத்தகம்)
    AmEnding,
    /// Ends with உ (e.g., வீடு, நாடு)
    UEnding,
    /// Ends with இ (e.g., பள்ளி, தமிழி)
    IEnding,
    /// Ends with ஐ (e.g., கவிதை, சிலை)
    AiEnding,
    /// Ends with consonant (e.g., தமிழ், நூல்)
    MeiEnding,
    /// Other
    Other,
}

impl NounClass {
    /// Determine noun class from a word
    pub fn from_word(word: &str) -> Self {
        if word.ends_with("ம்") {
            // Check for -அம் ending
            NounClass::AmEnding
        } else if word.ends_with("டு") || word.ends_with("று") || word.ends_with("ணு") {
            NounClass::UEnding
        } else if word.ends_with("ள்") || word.ends_with("ன்") || word.ends_with("ல்") || word.ends_with("ழ்") {
            NounClass::MeiEnding
        } else if word.ends_with("தை") || word.ends_with("லை") {
            NounClass::AiEnding
        } else if word.ends_with("ளி") || word.ends_with("டி") {
            NounClass::IEnding
        } else {
            NounClass::Other
        }
    }

    /// Get the oblique stem transformation
    pub fn oblique_suffix(&self) -> &'static str {
        match self {
            NounClass::AmEnding => "த்த",  // மரம் → மரத்த-
            NounClass::UEnding => "",      // வீடு → வீட்ட- (with doubling)
            NounClass::IEnding => "ய்",    // பள்ளி → பள்ளிய்-
            NounClass::AiEnding => "",     // கவிதை → கவிதையின்
            NounClass::MeiEnding => "",    // தமிழ் → தமிழ்-
            NounClass::Other => "",
        }
    }
}

/// Case suffix allomorphs
#[derive(Debug)]
pub struct CaseSuffixes {
    pub accusative: Vec<&'static str>,
    pub dative: Vec<&'static str>,
    pub genitive: Vec<&'static str>,
    pub locative: Vec<&'static str>,
    pub ablative: Vec<&'static str>,
    pub instrumental: Vec<&'static str>,
}

impl Default for CaseSuffixes {
    fn default() -> Self {
        Self {
            accusative: vec!["ஐ", "யை"],
            dative: vec!["கு", "க்கு", "உக்கு"],
            genitive: vec!["இன்", "உடைய", "அது"],
            locative: vec!["இல்", "இடத்தில்", "கண்"],
            ablative: vec!["இலிருந்து", "இன்", "இருந்து"],
            instrumental: vec!["ஆல்", "ஓடு", "உடன்"],
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_noun_class_detection() {
        assert_eq!(NounClass::from_word("மரம்"), NounClass::AmEnding);
        assert_eq!(NounClass::from_word("வீடு"), NounClass::UEnding);
        assert_eq!(NounClass::from_word("தமிழ்"), NounClass::MeiEnding);
    }
}
