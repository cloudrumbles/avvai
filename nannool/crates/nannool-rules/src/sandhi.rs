//! Sandhi rule definitions.
//!
//! This module defines the formalized sandhi rules derived from Nannool
//! verses 151-257.

use serde::{Deserialize, Serialize};
use tamil_unicode::{
    grapheme::{get_graphemes, graphemes_to_string},
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
    /// Insert the corresponding mellinam (nasal) of the second word's initial
    InsertMellinam,
    /// Convert final consonant to corresponding mellinam
    FinalToMellinam,
    /// Replace ending of first word and beginning of second word
    ReplaceBoth {
        left_old: String,
        left_new: String,
        right_old: String,
        right_new: String,
    },
    /// Apply multiple transformations in sequence
    Compound { steps: Vec<Transformation> },
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

            Transformation::InsertMellinam => {
                // Insert the corresponding mellinam of word2's initial
                // e.g., பூ + கொடி → பூங் + கொடி
                let graphemes2 = get_graphemes(word2);
                if let Some(initial_base) = graphemes2.first().and_then(|g| g.consonant_base()) {
                    if let Some(target_mellinam) = vallinam_to_mellinam(initial_base) {
                        return (format!("{}{}{}", word1, target_mellinam, '்'), word2.to_string());
                    }
                }
                (word1.to_string(), word2.to_string())
            }

            Transformation::FinalToMellinam => {
                // Convert final consonant to the mellinam corresponding to word2's initial
                // e.g., மரம் + காய் → மரங் + காய் (ம் → ங் because க's pair is ங்)
                let graphemes1 = get_graphemes(word1);
                let graphemes2 = get_graphemes(word2);

                if graphemes1.is_empty() || graphemes2.is_empty() {
                    return (word1.to_string(), word2.to_string());
                }

                // Get word2's initial consonant to determine target mellinam
                if let Some(initial_base) = graphemes2.first().and_then(|g| g.consonant_base()) {
                    if let Some(target_mellinam) = vallinam_to_mellinam(initial_base) {
                        let mut result = graphemes_to_string(&graphemes1[..graphemes1.len() - 1]);
                        result.push(target_mellinam);
                        result.push('்');
                        return (result, word2.to_string());
                    }
                }
                (word1.to_string(), word2.to_string())
            }

            Transformation::ReplaceBoth { left_old, left_new, right_old, right_new } => {
                let mut current_word1 = word1.to_string();
                let mut current_word2 = word2.to_string();

                if current_word1.ends_with(left_old) {
                    let base = &current_word1[..current_word1.len() - left_old.len()];
                    current_word1 = format!("{}{}", base, left_new);
                }

                if current_word2.starts_with(right_old) {
                    let rest = &current_word2[right_old.len()..];
                    current_word2 = format!("{}{}", right_new, rest);
                }

                (current_word1, current_word2)
            }

            Transformation::Compound { steps } => {
                let mut current_word1 = word1.to_string();
                let mut current_word2 = word2.to_string();
                for t in steps {
                    let (w1, w2) = t.apply(&current_word1, &current_word2);
                    current_word1 = w1;
                    current_word2 = w2;
                }
                (current_word1, current_word2)
            }
        }
    }

    /// Get the expected combined form
    pub fn expected_form(&self, word1: &str, word2: &str) -> String {
        let (w1, w2) = self.apply(word1, word2);
        combine_words(&w1, &w2)
    }

    /// Check if the transformation is already satisfied by the word pair
    pub fn check_satisfied(&self, word1: &str, word2: &str) -> bool {
        match self {
            Transformation::NoChange => true,

            Transformation::DoubleInitial => {
                // Check if the second word already has doubled initial
                // e.g., "பாட்டுப் பாடினான்" - the ப் should be doubled
                let graphemes = get_graphemes(word2);
                if graphemes.len() >= 2 {
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

            Transformation::Insert { text } => word1.ends_with(text),

            Transformation::DeleteFromFirst { .. } => {
                // If the rule still matches, it's not satisfied (it should have been deleted)
                false
            }

            Transformation::DeleteFromSecond { .. } => false,

            Transformation::ReplaceEnding { new, .. } => word1.ends_with(new),

            Transformation::ReplaceBeginning { new, .. } => word2.starts_with(new),

            Transformation::InsertMellinam => {
                // Check if word1 ends with the corresponding mellinam for word2
                let graphemes2 = get_graphemes(word2);
                if let Some(first) = graphemes2.first() {
                    if let Some(initial_base) = first.consonant_base() {
                        if let Some(expected_mellinam) = vallinam_to_mellinam(initial_base) {
                            let expected_suffix = format!("{}்", expected_mellinam);
                            return word1.ends_with(&expected_suffix);
                        }
                    }
                }
                false
            }

            Transformation::FinalToMellinam => {
                // Check if final consonant is the CORRESPONDING mellinam for word2's initial
                let graphemes1 = get_graphemes(word1);
                let graphemes2 = get_graphemes(word2);

                if let (Some(last), Some(first)) = (graphemes1.last(), graphemes2.first()) {
                    if let Some(initial_base) = first.consonant_base() {
                        if let Some(expected_mellinam) = vallinam_to_mellinam(initial_base) {
                            if let Some(final_base) = last.consonant_base() {
                                return final_base == expected_mellinam;
                            }
                        }
                    }
                }
                false
            }

            Transformation::ReplaceBoth { left_new, right_new, .. } => {
                word1.ends_with(left_new) && word2.starts_with(right_new)
            }

            Transformation::Compound { steps } => {
                // For compound transformations, they are satisfied if all steps are satisfied
                // This is a simplification but works for most cases
                steps.iter().all(|t| t.check_satisfied(word1, word2))
            }
        }
    }
}

/// Word classes for rule predicates
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum WordClass {
    /// பெயர்ச்சொல் - Noun
    Noun,
    /// வினைச்சொல் - Verb
    Verb,
    /// பெயரெச்சம் - Relative participle
    RelativeParticiple,
    /// வினையெச்சம் - Verbal participle
    VerbalParticiple,
    /// இடைச்சொல் - Particle
    Particle,
    /// உரிச்சொல் - Adjective/Adverb base
    Urichol,
    /// பிரதிப்பெயர் - Pronoun
    Pronoun,
    /// Any class
    Any,
}

/// Linguistic relationship between two words
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(rename_all = "snake_case")]
pub enum Relationship {
    /// அல்வழி - Non-case relationship
    #[default]
    Alvazhi,
    /// வேற்றுமை - Case relationship
    Vetrumai,
    /// Any relationship
    Any,
}

impl WordClass {
    /// Convert from nannool_morph::PartOfSpeech to WordClass
    pub fn from_pos(pos: nannool_morph::PartOfSpeech) -> Self {
        use nannool_morph::PartOfSpeech;
        match pos {
            PartOfSpeech::Noun => WordClass::Noun,
            PartOfSpeech::Verb => WordClass::Verb,
            PartOfSpeech::VerbalParticiple => WordClass::VerbalParticiple,
            PartOfSpeech::RelativeParticiple => WordClass::RelativeParticiple,
            PartOfSpeech::Adjective | PartOfSpeech::Adverb => WordClass::Urichol,
            PartOfSpeech::Particle | PartOfSpeech::Simile => WordClass::Particle,
            PartOfSpeech::Pronoun => WordClass::Pronoun,
            PartOfSpeech::Demonstrative
            | PartOfSpeech::Interrogative
            | PartOfSpeech::Numeral => WordClass::Noun,
            PartOfSpeech::Attributive => WordClass::RelativeParticiple,
            PartOfSpeech::Unknown => WordClass::Any,
        }
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

    /// Relationship type this rule applies to
    #[serde(default)]
    pub relationship: Relationship,

    /// What the first word must end with
    pub left_context: LeftContext,

    /// What the second word must start with
    pub right_context: RightContext,

    /// Word class constraints for the first word
    #[serde(default)]
    pub left_class: Vec<WordClass>,

    /// Word class constraints for the second word
    #[serde(default)]
    pub right_class: Vec<WordClass>,

    /// Morphological conditions (e.g., "causative", "plural")
    #[serde(default)]
    pub conditions: Vec<String>,

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
            relationship: Relationship::default(),
            left_context,
            right_context,
            left_class: vec![],
            right_class: vec![],
            conditions: vec![],
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

    /// Check if this rule applies with additional grammatical context
    pub fn matches_with_context(
        &self,
        word1: &str,
        word2: &str,
        class1: Option<WordClass>,
        class2: Option<WordClass>,
        relationship: Relationship,
        morph_conditions: &[String],
    ) -> bool {
        if !self.matches(word1, word2) {
            return false;
        }

        // Check relationship constraint
        // If caller passes Any, match all rules
        // If rule is Any, match all relationships
        // Otherwise, must match exactly
        if relationship != Relationship::Any
            && self.relationship != Relationship::Any
            && self.relationship != relationship
        {
            return false;
        }

        // Check word class constraints
        if !self.left_class.is_empty() {
            if let Some(c) = class1 {
                if !self.left_class.contains(&c) && !self.left_class.contains(&WordClass::Any) {
                    return false;
                }
            } else {
                return false;
            }
        }

        if !self.right_class.is_empty() {
            if let Some(c) = class2 {
                if !self.right_class.contains(&c) && !self.right_class.contains(&WordClass::Any) {
                    return false;
                }
            } else {
                return false;
            }
        }

        // Check morphological conditions
        for condition in &self.conditions {
            if !morph_conditions.contains(condition) {
                return false;
            }
        }

        true
    }

    /// Check if the current form satisfies the rule
    /// Returns true if sandhi is correctly applied, false if there's a violation
    pub fn is_satisfied(&self, word1: &str, word2: &str) -> bool {
        self.transformation.check_satisfied(word1, word2)
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
        String::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_transformation_insert_mellinam() {
        let trans = Transformation::InsertMellinam;
        let (w1, w2) = trans.apply("பூ", "கொடி");
        assert_eq!(w1, "பூங்");
        assert_eq!(w2, "கொடி");

        let (w1, w2) = trans.apply("பூ", "சோலை");
        assert_eq!(w1, "பூஞ்");
        assert_eq!(w2, "சோலை");
    }

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
    fn test_transformation_replace_both() {
        let trans = Transformation::ReplaceBoth {
            left_old: "ல்".to_string(),
            left_new: "ற்".to_string(),
            right_old: "த".to_string(),
            right_new: "ற".to_string(),
        };
        let (w1, w2) = trans.apply("கல்", "தீது");
        assert_eq!(w1, "கற்");
        assert_eq!(w2, "றீது");
    }

    #[test]
    fn test_transformation_compound() {
        let trans = Transformation::Compound {
            steps: vec![
                Transformation::ReplaceEnding { old: "ல்".to_string(), new: "ற்".to_string() },
                Transformation::ReplaceBeginning { old: "த".to_string(), new: "ற".to_string() },
            ],
        };
        let (w1, w2) = trans.apply("கல்", "தீது");
        assert_eq!(w1, "கற்");
        assert_eq!(w2, "றீது");
    }

    #[test]
    fn test_transformation_check_satisfied() {
        // DoubleInitial
        let trans = Transformation::DoubleInitial;
        assert!(!trans.check_satisfied("பாட்டு", "பாடினான்"));
        assert!(trans.check_satisfied("பாட்டு", "ப்பாடினான்"));

        // Insert
        let trans = Transformation::Insert { text: "ய்".to_string() };
        assert!(!trans.check_satisfied("படி", "அழகு"));
        assert!(trans.check_satisfied("படிய்", "அழகு"));

        // ReplaceEnding
        let trans = Transformation::ReplaceEnding { old: "ம்".to_string(), new: "ங்".to_string() };
        assert!(!trans.check_satisfied("மரம்", "காய்"));
        assert!(trans.check_satisfied("மரங்", "காய்"));

        // ReplaceBeginning
        let trans = Transformation::ReplaceBeginning { old: "த".to_string(), new: "ற".to_string() };
        assert!(!trans.check_satisfied("என்", "தந்தை"));
        assert!(trans.check_satisfied("என்", "றந்தை"));

        // ReplaceBoth
        let trans = Transformation::ReplaceBoth {
            left_old: "ல்".to_string(),
            left_new: "ற்".to_string(),
            right_old: "த".to_string(),
            right_new: "ற".to_string(),
        };
        assert!(!trans.check_satisfied("கல்", "தீது"));
        assert!(trans.check_satisfied("கற்", "றீது"));

        // FinalToMellinam
        let trans = Transformation::FinalToMellinam;
        assert!(!trans.check_satisfied("மரம்", "காய்"));
        assert!(trans.check_satisfied("மரங்", "காய்"));
        assert!(!trans.check_satisfied("மரம்", "தலை"));
        assert!(trans.check_satisfied("மரந்", "தலை"));

        // Compound
        let trans = Transformation::Compound {
            steps: vec![
                Transformation::ReplaceEnding { old: "ல்".to_string(), new: "ற்".to_string() },
                Transformation::ReplaceBeginning { old: "த".to_string(), new: "ற".to_string() },
            ],
        };
        assert!(!trans.check_satisfied("கல்", "தீது"));
        assert!(trans.check_satisfied("கற்", "றீது"));
    }

    #[test]
    fn test_vallinam_rule_matches() {
        let engine = crate::engine::RuleEngine::new();
        let rules = engine.rules();
        let vallinam_rule = rules.iter()
            .find(|r| r.id == "vallinam-miguthal-165")
            .unwrap();

        assert!(vallinam_rule.matches("பாட்டு", "பாடினான்"));
        assert!(!vallinam_rule.matches("தமிழ்", "பாடினான்"));
        assert!(!vallinam_rule.matches("பாட்டு", "மாடு"));
    }

    #[test]
    fn test_vallinam_rule_satisfaction() {
        let engine = crate::engine::RuleEngine::new();
        let rules = engine.rules();
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
        let engine = crate::engine::RuleEngine::new();
        let rules = engine.rules();
        let vallinam_rule = rules.iter()
            .find(|r| r.id == "vallinam-miguthal-165")
            .unwrap();

        let fix = vallinam_rule.suggest_fix("பாட்டு", "பாடினான்");
        assert_eq!(fix, Some("பாட்டுப்பாடினான்".to_string()));
    }

    #[test]
    fn test_udampadumei_ya() {
        let engine = crate::engine::RuleEngine::new();
        let rules = engine.rules();
        let ya_rule = rules.iter()
            .find(|r| r.id == "udampadumei-ya-162")
            .unwrap();

        assert!(ya_rule.matches("படி", "அழகு"));
        assert!(!ya_rule.matches("படு", "அழகு")); // உ is not in the specific vowels
    }

    #[test]
    fn test_rules_count() {
        let engine = crate::engine::RuleEngine::new();
        let rules = engine.rules();
        assert!(!rules.is_empty());
        // Should have many rules from TOML
        assert!(rules.len() >= 50, "Expected at least 50 rules, got {}", rules.len());
    }

    #[test]
    fn test_rule_ids_unique() {
        let engine = crate::engine::RuleEngine::new();
        let rules = engine.rules();
        let mut ids: Vec<&str> = rules.iter().map(|r| r.id.as_str()).collect();
        ids.sort();
        let unique_count = ids.len();
        ids.dedup();
        assert_eq!(ids.len(), unique_count, "Rules should have unique IDs");
    }
}
