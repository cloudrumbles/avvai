//! Verb morphology.
//!
//! Tamil verb features based on Nannool's வினையியல்.

use serde::{Deserialize, Serialize};
use crate::analyzer::{Tense, Person, Gender, Number, Mood};

/// Verb features
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VerbFeatures {
    /// Tense (காலம்)
    pub tense: Option<Tense>,
    /// Person (இடம்)
    pub person: Option<Person>,
    /// Gender (பால்)
    pub gender: Option<Gender>,
    /// Number (எண்)
    pub number: Option<Number>,
    /// Mood
    pub mood: Option<Mood>,
    /// Is this a finite verb?
    pub finite: bool,
}

impl VerbFeatures {
    pub fn new() -> Self {
        Self {
            tense: None,
            person: None,
            gender: None,
            number: None,
            mood: None,
            finite: false,
        }
    }

    pub fn finite(tense: Tense, person: Person, gender: Gender, number: Number) -> Self {
        Self {
            tense: Some(tense),
            person: Some(person),
            gender: Some(gender),
            number: Some(number),
            mood: Some(Mood::Indicative),
            finite: true,
        }
    }
}

impl Default for VerbFeatures {
    fn default() -> Self {
        Self::new()
    }
}

/// Verb class based on conjugation pattern
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum VerbClass {
    /// Strong verbs (வலிமை) - past tense with த்த
    Strong,
    /// Weak verbs (மெலிமை) - past tense with ந்த
    Weak,
    /// Class 3 - past tense with ன்ற
    Class3,
    /// Class 4 - past tense with இன்
    Class4,
    /// Class 5 - past tense with ட்ட
    Class5,
    /// Class 6 - past tense with ற்ற
    Class6,
    /// Irregular
    Irregular,
}

impl VerbClass {
    /// Get the past tense marker for this verb class
    pub fn past_marker(&self) -> &'static str {
        match self {
            VerbClass::Strong => "த்த",
            VerbClass::Weak => "ந்த",
            VerbClass::Class3 => "ன்ற",
            VerbClass::Class4 => "இன",
            VerbClass::Class5 => "ட்ட",
            VerbClass::Class6 => "ற்ற",
            VerbClass::Irregular => "",
        }
    }

    /// Get the present tense marker
    pub fn present_marker(&self) -> &'static str {
        match self {
            VerbClass::Strong | VerbClass::Weak | VerbClass::Class3 => "கிற",
            VerbClass::Class4 | VerbClass::Class5 | VerbClass::Class6 => "கின்ற",
            VerbClass::Irregular => "",
        }
    }

    /// Get the future tense marker
    pub fn future_marker(&self) -> &'static str {
        "வ" // Common for most verbs
    }
}

/// Personal endings for finite verbs
#[derive(Debug)]
pub struct PersonalEndings {
    /// Third person masculine singular
    pub third_masc_sg: &'static str,
    /// Third person feminine singular
    pub third_fem_sg: &'static str,
    /// Third person honorific/plural
    pub third_hon: &'static str,
    /// Third person neuter singular
    pub third_neut_sg: &'static str,
    /// Third person neuter plural
    pub third_neut_pl: &'static str,
    /// First person singular
    pub first_sg: &'static str,
    /// First person plural (inclusive)
    pub first_pl_incl: &'static str,
    /// First person plural (exclusive)
    pub first_pl_excl: &'static str,
    /// Second person singular
    pub second_sg: &'static str,
    /// Second person plural/honorific
    pub second_pl: &'static str,
}

impl Default for PersonalEndings {
    fn default() -> Self {
        Self {
            third_masc_sg: "ஆன்",
            third_fem_sg: "ஆள்",
            third_hon: "ஆர்",
            third_neut_sg: "அது",
            third_neut_pl: "அன",
            first_sg: "ஏன்",
            first_pl_incl: "ஓம்",
            first_pl_excl: "ஓம்",
            second_sg: "ஆய்",
            second_pl: "ீர்கள்",
        }
    }
}

/// Common irregular verbs
pub fn get_irregular_verbs() -> Vec<(&'static str, &'static str, &'static str)> {
    // (infinitive, past stem, present stem)
    vec![
        ("வா", "வந்த", "வருகிற"),
        ("போ", "போன", "போகிற"),
        ("சொல்", "சொன்ன", "சொல்கிற"),
        ("கொள்", "கொண்ட", "கொள்கிற"),
        ("செய்", "செய்த", "செய்கிற"),
        ("தா", "தந்த", "தருகிற"),
        ("நில்", "நின்ற", "நிற்கிற"),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_verb_class_markers() {
        assert_eq!(VerbClass::Strong.past_marker(), "த்த");
        assert_eq!(VerbClass::Weak.past_marker(), "ந்த");
    }

    #[test]
    fn test_irregular_verbs() {
        let irregulars = get_irregular_verbs();
        assert!(!irregulars.is_empty());

        // வா should be in the list
        assert!(irregulars.iter().any(|(inf, _, _)| *inf == "வா"));
    }
}
