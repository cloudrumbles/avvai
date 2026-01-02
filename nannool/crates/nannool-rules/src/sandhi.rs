//! Sandhi rule definitions.
//!
//! This module defines the formalized sandhi rules derived from Nannool
//! verses 151-257.

use serde::{Deserialize, Serialize};
use tamil_unicode::{
    grapheme::{get_graphemes, get_initial_grapheme, graphemes_to_string},
    letters::*,
    transform::*,
};

use crate::patterns::{LeftContext, RightContext};

/// Category of sandhi transformation
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SandhiCategory {
    /// இயல்பு புணர்ச்சி - natural joining, no change
    Iyalbu,
    /// தோன்றல் - letter appears
    Thondral,
    /// திரிதல் - letter changes
    Thirithal,
    /// கெடுதல் - letter disappears
    Keduthal,
}

impl SandhiCategory {
    /// Get the Tamil name
    pub fn tamil_name(&self) -> &'static str {
        match self {
            SandhiCategory::Iyalbu => "இயல்பு",
            SandhiCategory::Thondral => "தோன்றல்",
            SandhiCategory::Thirithal => "திரிதல்",
            SandhiCategory::Keduthal => "கெடுதல்",
        }
    }

    /// Get English description
    pub fn description(&self) -> &'static str {
        match self {
            SandhiCategory::Iyalbu => "Natural joining without change",
            SandhiCategory::Thondral => "A letter appears",
            SandhiCategory::Thirithal => "A letter changes",
            SandhiCategory::Keduthal => "A letter disappears",
        }
    }
}

/// Transformation to apply when sandhi rule matches
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum Transformation {
    /// No change needed (இயல்பு)
    NoChange,
    /// Double the initial consonant of second word (வல்லினம் மிகுதல்)
    DoubleInitial,
    /// Insert letter(s) between words
    Insert { text: String },
    /// Delete letter(s) from end of first word
    DeleteFromFirst { count: usize },
    /// Delete letter(s) from start of second word
    DeleteFromSecond { count: usize },
    /// Replace ending of first word
    ReplaceEnding { old: String, new: String },
    /// Replace beginning of second word
    ReplaceBeginning { old: String, new: String },
    /// Convert final consonant to corresponding mellinam
    FinalToMellinam,
}

impl Transformation {
    /// Apply this transformation to a word pair
    pub fn apply(&self, word1: &str, word2: &str) -> (String, String) {
        match self {
            Transformation::NoChange => (word1.to_string(), word2.to_string()),

            Transformation::DoubleInitial => {
                if let Some(doubled) = double_initial_consonant(word2) {
                    (word1.to_string(), doubled)
                } else {
                    (word1.to_string(), word2.to_string())
                }
            }

            Transformation::Insert { text } => {
                // Insert between the words - append to first word
                (format!("{}{}", word1, text), word2.to_string())
            }

            Transformation::DeleteFromFirst { count } => {
                let graphemes = get_graphemes(word1);
                if graphemes.len() >= *count {
                    let remaining = &graphemes[..graphemes.len() - count];
                    (graphemes_to_string(remaining), word2.to_string())
                } else {
                    (word1.to_string(), word2.to_string())
                }
            }

            Transformation::DeleteFromSecond { count } => {
                let graphemes = get_graphemes(word2);
                if graphemes.len() >= *count {
                    let remaining = &graphemes[*count..];
                    (word1.to_string(), graphemes_to_string(remaining))
                } else {
                    (word1.to_string(), word2.to_string())
                }
            }

            Transformation::ReplaceEnding { old, new } => {
                if word1.ends_with(old) {
                    let base = &word1[..word1.len() - old.len()];
                    (format!("{}{}", base, new), word2.to_string())
                } else {
                    (word1.to_string(), word2.to_string())
                }
            }

            Transformation::ReplaceBeginning { old, new } => {
                if word2.starts_with(old) {
                    let rest = &word2[old.len()..];
                    (word1.to_string(), format!("{}{}", new, rest))
                } else {
                    (word1.to_string(), word2.to_string())
                }
            }

            Transformation::FinalToMellinam => {
                // Convert final vallinam consonant to corresponding mellinam
                let graphemes = get_graphemes(word1);
                if graphemes.is_empty() {
                    return (word1.to_string(), word2.to_string());
                }

                let last = graphemes.last().unwrap();
                if let Some(base) = last.consonant_base() {
                    if let Some(mellinam) = vallinam_to_mellinam(base) {
                        let mut result = graphemes_to_string(&graphemes[..graphemes.len() - 1]);
                        result.push(mellinam);
                        result.push('்');
                        return (result, word2.to_string());
                    }
                }
                (word1.to_string(), word2.to_string())
            }
        }
    }

    /// Get the expected combined form
    pub fn expected_form(&self, word1: &str, word2: &str) -> String {
        let (w1, w2) = self.apply(word1, word2);
        format!("{}{}", w1, w2)
    }
}

/// A formalized sandhi rule derived from Nannool
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SandhiRule {
    /// Unique identifier
    pub id: String,

    /// Source verse number(s)
    pub nannool_verses: Vec<u32>,

    /// Tamil name of the rule
    pub tamil_name: String,

    /// English name of the rule
    pub english_name: String,

    /// What the first word must end with
    #[serde(flatten)]
    pub left_context: LeftContext,

    /// What the second word must start with
    #[serde(flatten)]
    pub right_context: RightContext,

    /// What transformation should occur
    pub transformation: Transformation,

    /// Rule category
    pub category: SandhiCategory,

    /// Priority (higher = checked first)
    pub priority: u8,

    /// Is this an exception to another rule?
    #[serde(default)]
    pub is_exception: bool,

    /// References to rules this is an exception to
    #[serde(default)]
    pub exception_of: Vec<String>,

    /// Brief description
    pub description: String,
}

impl SandhiRule {
    /// Create a new sandhi rule
    pub fn new(
        id: impl Into<String>,
        tamil_name: impl Into<String>,
        english_name: impl Into<String>,
        nannool_verses: Vec<u32>,
        left_context: LeftContext,
        right_context: RightContext,
        transformation: Transformation,
        category: SandhiCategory,
    ) -> Self {
        Self {
            id: id.into(),
            nannool_verses,
            tamil_name: tamil_name.into(),
            english_name: english_name.into(),
            left_context,
            right_context,
            transformation,
            category,
            priority: 50,
            is_exception: false,
            exception_of: vec![],
            description: String::new(),
        }
    }

    /// Set the priority
    pub fn with_priority(mut self, priority: u8) -> Self {
        self.priority = priority;
        self
    }

    /// Set as exception to another rule
    pub fn as_exception_of(mut self, rule_id: impl Into<String>) -> Self {
        self.is_exception = true;
        self.exception_of.push(rule_id.into());
        self
    }

    /// Set description
    pub fn with_description(mut self, desc: impl Into<String>) -> Self {
        self.description = desc.into();
        self
    }

    /// Check if this rule applies to a word pair
    pub fn matches(&self, word1: &str, word2: &str) -> bool {
        self.left_context.matches(word1) && self.right_context.matches(word2)
    }

    /// Check if the current form satisfies the rule
    /// Returns true if sandhi is correctly applied, false if there's a violation
    pub fn is_satisfied(&self, word1: &str, word2: &str) -> bool {
        match &self.transformation {
            Transformation::NoChange => {
                // No change needed, always satisfied
                true
            }
            Transformation::DoubleInitial => {
                // Check if the second word already has doubled initial
                // e.g., "பாட்டுப் பாடினான்" - the ப் should be doubled
                let graphemes = get_graphemes(word2);
                if graphemes.len() >= 2 {
                    // Check if first grapheme is mei and second has same base
                    let first = &graphemes[0];
                    let second = &graphemes[1];
                    if first.ends_with_consonant() {
                        if let (Some(base1), Some(base2)) =
                            (first.consonant_base(), second.consonant_base())
                        {
                            return base1 == base2;
                        }
                    }
                }
                false
            }
            Transformation::Insert { text } => {
                // Check if first word ends with the inserted text
                word1.ends_with(text)
            }
            _ => {
                // For other transformations, we'd need more complex checking
                true
            }
        }
    }

    /// Get the suggested correction if rule is violated
    pub fn suggest_fix(&self, word1: &str, word2: &str) -> Option<String> {
        if self.is_satisfied(word1, word2) {
            None
        } else {
            Some(self.transformation.expected_form(word1, word2))
        }
    }

    /// Get the verse text for this rule
    pub fn verse_text(&self) -> String {
        use crate::verses::key_verses;

        // Return the verse text for known verses
        match self.nannool_verses.first() {
            Some(162) => key_verses::VERSE_162.to_string(),
            Some(165) => key_verses::VERSE_165.to_string(),
            Some(204) => key_verses::VERSE_204.to_string(),
            Some(205) => key_verses::VERSE_205.to_string(),
            Some(206) => key_verses::VERSE_206.to_string(),
            Some(217) => key_verses::VERSE_217.to_string(),
            _ => String::new(),
        }
    }
}

/// Get all built-in sandhi rules
pub fn get_builtin_rules() -> Vec<SandhiRule> {
    vec![
        // Verse 165 - வல்லினம் மிகுதல் (Vallinam doubling)
        SandhiRule::new(
            "vallinam-miguthal-165",
            "வல்லினம் மிகுதல்",
            "Vallinam doubling",
            vec![165],
            LeftContext::AnyUyir,
            RightContext::Vallinam,
            Transformation::DoubleInitial,
            SandhiCategory::Thondral,
        )
        .with_priority(50)
        .with_description("Hard consonants (க ச ட த ப ற) double after vowel endings"),

        // Verse 162 - உடம்படுமெய் யகரம் (Buffer consonant - ya)
        SandhiRule::new(
            "udampadumei-ya-162",
            "உடம்படுமெய் - யகரம்",
            "Buffer consonant - ya",
            vec![162],
            LeftContext::SpecificUyir { letters: vec!['இ', 'ஈ', 'ஐ'] },
            RightContext::AnyUyir,
            Transformation::Insert { text: "ய்".to_string() },
            SandhiCategory::Thondral,
        )
        .with_priority(60)
        .with_description("After இ/ஈ/ஐ, ய் appears before a following vowel"),

        // Verse 162 - உடம்படுமெய் வகரம் (Buffer consonant - va)
        SandhiRule::new(
            "udampadumei-va-162",
            "உடம்படுமெய் - வகரம்",
            "Buffer consonant - va",
            vec![162],
            LeftContext::SpecificUyir { letters: vec!['அ', 'ஆ', 'உ', 'ஊ', 'எ', 'ஒ', 'ஓ', 'ஔ'] },
            RightContext::AnyUyir,
            Transformation::Insert { text: "வ்".to_string() },
            SandhiCategory::Thondral,
        )
        .with_priority(60)
        .with_description("After vowels other than இ/ஈ/ஐ/ஏ, வ் appears before a following vowel"),

        // Verse 204 - மெய்யும் உயிரும் புணர்தல் (Mei-Uyir combination)
        SandhiRule::new(
            "mei-uyir-combination-204",
            "மெய்யும் உயிரும் புணர்தல்",
            "Consonant-vowel combination",
            vec![204],
            LeftContext::AnyMei,
            RightContext::AnyUyir,
            Transformation::NoChange, // They naturally combine
            SandhiCategory::Iyalbu,
        )
        .with_priority(100)
        .with_description("When a vowel follows a consonant, they naturally combine"),

        // Verse 205 - ஒற்று இரட்டல் (Consonant doubling after kuril)
        SandhiRule::new(
            "otru-irattal-205",
            "ஒற்று இரட்டல்",
            "Consonant doubling",
            vec![205],
            LeftContext::KurilPlusMei,
            RightContext::AnyUyir,
            Transformation::DoubleInitial, // The final consonant doubles
            SandhiCategory::Thondral,
        )
        .with_priority(70)
        .with_description("A consonant after a single short vowel doubles when a vowel follows"),

        // Verse 206 - குற்றியலுகரப் புணர்ச்சி
        SandhiRule::new(
            "kutriyalukaram-206",
            "குற்றியலுகரப் புணர்ச்சி",
            "Kutriyalukaram sandhi",
            vec![206],
            LeftContext::SpecificUyir { letters: vec!['உ'] }, // Short உ
            RightContext::Vallinam,
            Transformation::DoubleInitial,
            SandhiCategory::Thondral,
        )
        .with_priority(65)
        .with_description("Kutriyalukaram (short உ) followed by vallinam - the vallinam doubles"),

        // Verse 217 - ம் + வல்லினம் → corresponding mellinam
        SandhiRule::new(
            "mam-vallinam-217",
            "மகர இறுதி வல்லின புணர்ச்சி",
            "Mam + vallinam sandhi",
            vec![217],
            LeftContext::SpecificMei { letters: vec!["ம்".to_string()] },
            RightContext::Vallinam,
            Transformation::FinalToMellinam,
            SandhiCategory::Thirithal,
        )
        .with_priority(55)
        .with_description("When ம் ending meets vallinam beginning, ம் becomes the corresponding mellinam"),

        // Additional common rules

        // Aaytham before vallinam
        SandhiRule::new(
            "aaytham-vallinam",
            "ஆய்த வல்லினம்",
            "Aaytham before vallinam",
            vec![],
            LeftContext::SpecificWord { words: vec!["அஃது".to_string()] },
            RightContext::Vallinam,
            Transformation::NoChange,
            SandhiCategory::Iyalbu,
        )
        .with_priority(80)
        .with_description("Aaytham (ஃ) before vallinam follows special rules"),

        // ன் + த → ன்ற (retroflex assimilation)
        SandhiRule::new(
            "retroflex-assimilation",
            "ன்-த புணர்ச்சி",
            "Retroflex assimilation",
            vec![220],
            LeftContext::SpecificMei { letters: vec!["ன்".to_string()] },
            RightContext::SpecificMei { letters: vec!["த்".to_string()] },
            Transformation::ReplaceBeginning {
                old: "த".to_string(),
                new: "ற".to_string()
            },
            SandhiCategory::Thirithal,
        )
        .with_priority(75)
        .with_description("When ன் meets த, த becomes ற"),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_transformation_double_initial() {
        let trans = Transformation::DoubleInitial;
        let (w1, w2) = trans.apply("பாட்டு", "பாடினான்");
        assert_eq!(w1, "பாட்டு");
        assert_eq!(w2, "ப்பாடினான்");
    }

    #[test]
    fn test_transformation_insert() {
        let trans = Transformation::Insert { text: "ய்".to_string() };
        let (w1, w2) = trans.apply("படி", "அழகு");
        assert_eq!(w1, "படிய்");
        assert_eq!(w2, "அழகு");
    }

    #[test]
    fn test_vallinam_rule_matches() {
        let rules = get_builtin_rules();
        let vallinam_rule = rules.iter()
            .find(|r| r.id == "vallinam-miguthal-165")
            .unwrap();

        assert!(vallinam_rule.matches("பாட்டு", "பாடினான்"));
        assert!(!vallinam_rule.matches("தமிழ்", "பாடினான்"));
        assert!(!vallinam_rule.matches("பாட்டு", "மாடு"));
    }

    #[test]
    fn test_vallinam_rule_satisfaction() {
        let rules = get_builtin_rules();
        let vallinam_rule = rules.iter()
            .find(|r| r.id == "vallinam-miguthal-165")
            .unwrap();

        // Incorrect - should have doubled
        assert!(!vallinam_rule.is_satisfied("பாட்டு", "பாடினான்"));

        // Correct - properly doubled
        assert!(vallinam_rule.is_satisfied("பாட்டு", "ப்பாடினான்"));
    }

    #[test]
    fn test_suggest_fix() {
        let rules = get_builtin_rules();
        let vallinam_rule = rules.iter()
            .find(|r| r.id == "vallinam-miguthal-165")
            .unwrap();

        let fix = vallinam_rule.suggest_fix("பாட்டு", "பாடினான்");
        assert_eq!(fix, Some("பாட்டுப்பாடினான்".to_string()));
    }

    #[test]
    fn test_udampadumei_ya() {
        let rules = get_builtin_rules();
        let ya_rule = rules.iter()
            .find(|r| r.id == "udampadumei-ya-162")
            .unwrap();

        assert!(ya_rule.matches("படி", "அழகு"));
        assert!(!ya_rule.matches("படு", "அழகு")); // உ is not in the specific vowels
    }

    #[test]
    fn test_builtin_rules_count() {
        let rules = get_builtin_rules();
        assert!(!rules.is_empty());
        assert!(rules.len() >= 8); // We have at least 8 rules defined
    }
}
