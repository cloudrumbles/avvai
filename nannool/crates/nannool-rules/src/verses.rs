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

/// Key sandhi verses with their Tamil text
pub mod key_verses {
    /// Verse 162 - உடம்படுமெய் (Buffer consonants)
    pub const VERSE_162: &str =
        "இ ஈ ஐ வழி யவ்வும் ஏனை\n\
         உயிர்வழி வவ்வும் ஏ முன் இவ் இருமையும்\n\
         உயிர்வரின் உடம்படு மெய் என்று ஆகும்";

    /// Verse 165 - வல்லினம் மிகுதல் (Vallinam doubling)
    pub const VERSE_165: &str =
        "இயல்பினும் விதியினும் நின்ற உயிர்முன்\n\
         க ச த ப மிகும் விதவாதன மன்னே";

    /// Verse 204 - மெய்யும் உயிரும் புணர்தல் (Mei-uyir combination)
    pub const VERSE_204: &str =
        "உடல்மேல் உயிர்வந்து ஒன்றுவது இயல்பே";

    /// Verse 205 - ஒற்று இரட்டல் (Consonant doubling)
    pub const VERSE_205: &str =
        "தனிக்குறில் முன் ஒற்று உயிர்வரின் இரட்டும்";

    /// Verse 206 - குற்றியலுகரப் புணர்ச்சி
    pub const VERSE_206: &str =
        "குற்றியலுகரம் முன் வன்மை மிகும்";

    /// Verse 217 - ம் + வல்லினம் புணர்ச்சி
    pub const VERSE_217: &str =
        "மகர இறுதி வருமொழி முதலிய\n\
         வன்மைக் கேற்ற மெல்லெழுத்து ஆகும்";
}

/// Built-in collection of key verses for sandhi checking
pub fn get_sandhi_verses() -> Vec<NannoolVerse> {
    vec![
        NannoolVerse::new(
            162,
            Chapter::Ezhuthu,
            Section::UyirPunarchi,
            key_verses::VERSE_162.to_string(),
            "After இ/ஈ/ஐ, ய் appears; after other vowels, வ் appears; after ஏ, either can appear - when a vowel follows".to_string(),
        ),
        NannoolVerse::new(
            165,
            Chapter::Ezhuthu,
            Section::UyirPunarchi,
            key_verses::VERSE_165.to_string(),
            "Before vowel endings (by nature or rule), க ச ட த ப ற will double, unless exceptions apply".to_string(),
        ),
        NannoolVerse::new(
            204,
            Chapter::Ezhuthu,
            Section::MeyPunarchi,
            key_verses::VERSE_204.to_string(),
            "When a vowel comes after a consonant, they naturally combine".to_string(),
        ),
        NannoolVerse::new(
            205,
            Chapter::Ezhuthu,
            Section::MeyPunarchi,
            key_verses::VERSE_205.to_string(),
            "A consonant after a single short vowel doubles when a vowel follows".to_string(),
        ),
        NannoolVerse::new(
            206,
            Chapter::Ezhuthu,
            Section::MeyPunarchi,
            key_verses::VERSE_206.to_string(),
            "Kutriyalukaram followed by vallinam - the vallinam doubles".to_string(),
        ),
        NannoolVerse::new(
            217,
            Chapter::Ezhuthu,
            Section::MeyPunarchi,
            key_verses::VERSE_217.to_string(),
            "When ம் ending meets vallinam beginning, ம் becomes the corresponding mellinam".to_string(),
        ),
    ]
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

    #[test]
    fn test_get_sandhi_verses() {
        let verses = get_sandhi_verses();
        assert!(!verses.is_empty());
        assert!(verses.iter().all(|v| v.is_sandhi_verse()));
    }
}
