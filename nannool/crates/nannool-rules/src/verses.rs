//! Nannool verse definitions and parsing.
//!
//! This module contains the data structures for representing
//! Nannool verses (நூற்பாக்கள்).

use serde::{Deserialize, Serialize};

/// A Nannool verse (நூற்பா)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NannoolVerse {
    /// Verse number (1-462)
    pub number: u32,

    /// Chapter (அதிகாரம்)
    pub chapter: Chapter,

    /// Section (இயல்)
    pub section: Section,

    /// Original Tamil text
    pub tamil_text: String,

    /// Transliteration (optional)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transliteration: Option<String>,

    /// Brief meaning/translation
    pub meaning: String,

    /// Detailed commentary (optional)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub commentary: Option<String>,
}

/// Main chapters of Nannool
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Chapter {
    /// பொதுப்பாயிரம் (verses 1-55) - General preface
    Payiram,
    /// எழுத்ததிகாரம் (verses 56-257) - Letters/Phonology
    Ezhuthu,
    /// சொல்லதிகாரம் (verses 258-462) - Words/Morphology
    Sol,
}

impl Chapter {
    /// Get the Tamil name of the chapter
    pub fn tamil_name(&self) -> &'static str {
        match self {
            Chapter::Payiram => "பொதுப்பாயிரம்",
            Chapter::Ezhuthu => "எழுத்ததிகாரம்",
            Chapter::Sol => "சொல்லதிகாரம்",
        }
    }

    /// Get the verse range for this chapter
    pub fn verse_range(&self) -> (u32, u32) {
        match self {
            Chapter::Payiram => (1, 55),
            Chapter::Ezhuthu => (56, 257),
            Chapter::Sol => (258, 462),
        }
    }
}

/// Sections within each chapter
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Section {
    // பாயிரம் sections
    /// பாயிரம் - Preface
    PayiramGeneral,

    // எழுத்ததிகாரம் sections
    /// எழுத்து இயல் (56-127) - Nature of letters
    EzhuthuIyal,
    /// பதவியல் (128-150) - Word formation
    Pathavial,
    /// உயிரீற்றுப் புணரியல் (151-203) - Vowel-ending sandhi
    UyirPunarchi,
    /// மெய்யீற்றுப் புணரியல் (204-239) - Consonant-ending sandhi
    MeyPunarchi,
    /// உருபு புணரியல் (240-257) - Case marker sandhi
    UrupuPunarchi,

    // சொல்லதிகாரம் sections
    /// பெயரியல் - Nouns
    Peyariyal,
    /// வினையியல் - Verbs
    Vinaiyiyal,
    /// இடையியல் - Particles
    Idaiyiyal,
    /// உரியியல் - Unique words
    Uriyiyal,
}

impl Section {
    /// Get the Tamil name of the section
    pub fn tamil_name(&self) -> &'static str {
        match self {
            Section::PayiramGeneral => "பாயிரம்",
            Section::EzhuthuIyal => "எழுத்து இயல்",
            Section::Pathavial => "பதவியல்",
            Section::UyirPunarchi => "உயிரீற்றுப் புணரியல்",
            Section::MeyPunarchi => "மெய்யீற்றுப் புணரியல்",
            Section::UrupuPunarchi => "உருபு புணரியல்",
            Section::Peyariyal => "பெயரியல்",
            Section::Vinaiyiyal => "வினையியல்",
            Section::Idaiyiyal => "இடையியல்",
            Section::Uriyiyal => "உரியியல்",
        }
    }

    /// Get the verse range for this section
    pub fn verse_range(&self) -> (u32, u32) {
        match self {
            Section::PayiramGeneral => (1, 55),
            Section::EzhuthuIyal => (56, 127),
            Section::Pathavial => (128, 150),
            Section::UyirPunarchi => (151, 203),
            Section::MeyPunarchi => (204, 239),
            Section::UrupuPunarchi => (240, 257),
            Section::Peyariyal => (258, 320),
            Section::Vinaiyiyal => (321, 400),
            Section::Idaiyiyal => (401, 440),
            Section::Uriyiyal => (441, 462),
        }
    }

    /// Check if this section contains sandhi rules
    pub fn is_sandhi_section(&self) -> bool {
        matches!(
            self,
            Section::UyirPunarchi | Section::MeyPunarchi | Section::UrupuPunarchi
        )
    }

    /// Parse section from Tamil name
    pub fn from_tamil_name(name: &str) -> Option<Self> {
        match name {
            "பாயிரம்" => Some(Section::PayiramGeneral),
            "எழுத்து இயல்" => Some(Section::EzhuthuIyal),
            "பதவியல்" => Some(Section::Pathavial),
            "உயிரீற்றுப் புணரியல்" => Some(Section::UyirPunarchi),
            "மெய்யீற்றுப் புணரியல்" => Some(Section::MeyPunarchi),
            "உருபு புணரியல்" => Some(Section::UrupuPunarchi),
            "பெயரியல்" => Some(Section::Peyariyal),
            "வினையியல்" => Some(Section::Vinaiyiyal),
            "இடையியல்" => Some(Section::Idaiyiyal),
            "உரியியல்" => Some(Section::Uriyiyal),
            _ => None,
        }
    }
}

impl NannoolVerse {
    /// Create a new verse
    pub fn new(
        number: u32,
        chapter: Chapter,
        section: Section,
        tamil_text: String,
        meaning: String,
    ) -> Self {
        Self {
            number,
            chapter,
            section,
            tamil_text,
            transliteration: None,
            meaning,
            commentary: None,
        }
    }

    /// Check if this verse contains sandhi rules
    pub fn is_sandhi_verse(&self) -> bool {
        self.section.is_sandhi_section()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_chapter_verse_ranges() {
        assert_eq!(Chapter::Payiram.verse_range(), (1, 55));
        assert_eq!(Chapter::Ezhuthu.verse_range(), (56, 257));
        assert_eq!(Chapter::Sol.verse_range(), (258, 462));
    }

    #[test]
    fn test_section_is_sandhi() {
        assert!(Section::UyirPunarchi.is_sandhi_section());
        assert!(Section::MeyPunarchi.is_sandhi_section());
        assert!(!Section::EzhuthuIyal.is_sandhi_section());
    }
}
