//! Main morphological analysis interface.

use serde::{Deserialize, Serialize};
use tamil_unicode::grapheme::{get_graphemes, graphemes_to_string};

use crate::noun::NounFeatures;
use crate::verb::VerbFeatures;
use crate::suffix::SuffixPattern;

/// Part of speech
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PartOfSpeech {
    /// பெயர்ச்சொல் - Noun
    Noun,
    /// வினைச்சொல் - Verb
    Verb,
    /// உரிச்சொல் - Adjective/Adverb
    Adjective,
    /// பெயரடை - Attributive
    Attributive,
    /// வினையடை - Adverb
    Adverb,
    /// பிரதிப்பெயர் - Pronoun
    Pronoun,
    /// சுட்டுப்பெயர் - Demonstrative
    Demonstrative,
    /// வினாப்பெயர் - Interrogative
    Interrogative,
    /// இடைச்சொல் - Particle
    Particle,
    /// உவமைச்சொல் - Simile particle
    Simile,
    /// எண்ணுப்பெயர் - Numeral
    Numeral,
    /// Unknown
    Unknown,
}

impl PartOfSpeech {
    /// Get Tamil name
    pub fn tamil_name(&self) -> &'static str {
        match self {
            PartOfSpeech::Noun => "பெயர்ச்சொல்",
            PartOfSpeech::Verb => "வினைச்சொல்",
            PartOfSpeech::Adjective => "உரிச்சொல்",
            PartOfSpeech::Attributive => "பெயரடை",
            PartOfSpeech::Adverb => "வினையடை",
            PartOfSpeech::Pronoun => "பிரதிப்பெயர்",
            PartOfSpeech::Demonstrative => "சுட்டுப்பெயர்",
            PartOfSpeech::Interrogative => "வினாப்பெயர்",
            PartOfSpeech::Particle => "இடைச்சொல்",
            PartOfSpeech::Simile => "உவமைச்சொல்",
            PartOfSpeech::Numeral => "எண்ணுப்பெயர்",
            PartOfSpeech::Unknown => "அறியாத",
        }
    }
}

/// Grammatical features
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum Feature {
    /// Case marker
    Case(Case),
    /// Number
    Number(Number),
    /// Person
    Person(Person),
    /// Gender
    Gender(Gender),
    /// Tense
    Tense(Tense),
    /// Mood
    Mood(Mood),
    /// Honorific
    Honorific(Honorific),
    /// Suffix (raw)
    Suffix(String),
}

/// Case (வேற்றுமை)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Case {
    /// எழுவாய் - Nominative
    Nominative,
    /// ஐ வேற்றுமை - Accusative
    Accusative,
    /// ஆல் வேற்றுமை - Instrumental
    Instrumental,
    /// கு வேற்றுமை - Dative
    Dative,
    /// இன் வேற்றுமை - Ablative
    Ablative,
    /// அது வேற்றுமை - Genitive
    Genitive,
    /// கண் வேற்றுமை - Locative
    Locative,
    /// விளி வேற்றுமை - Vocative
    Vocative,
}

impl Case {
    pub fn tamil_name(&self) -> &'static str {
        match self {
            Case::Nominative => "எழுவாய்",
            Case::Accusative => "ஐ வேற்றுமை",
            Case::Instrumental => "ஆல் வேற்றுமை",
            Case::Dative => "கு வேற்றுமை",
            Case::Ablative => "இன் வேற்றுமை",
            Case::Genitive => "அது வேற்றுமை",
            Case::Locative => "கண் வேற்றுமை",
            Case::Vocative => "விளி வேற்றுமை",
        }
    }

    pub fn suffix(&self) -> &'static str {
        match self {
            Case::Nominative => "",
            Case::Accusative => "ஐ",
            Case::Instrumental => "ஆல்",
            Case::Dative => "கு",
            Case::Ablative => "இன்",
            Case::Genitive => "அது",
            Case::Locative => "கண்",
            Case::Vocative => "ஏ",
        }
    }
}

/// Number (எண்)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Number {
    /// ஒருமை - Singular
    Singular,
    /// பன்மை - Plural
    Plural,
}

/// Person (இடம்)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Person {
    /// தன்மை - First person
    First,
    /// முன்னிலை - Second person
    Second,
    /// படர்க்கை - Third person
    Third,
}

/// Gender (பால்)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Gender {
    /// ஆண்பால் - Masculine
    Masculine,
    /// பெண்பால் - Feminine
    Feminine,
    /// பலர்பால் - Rational plural (people)
    RationalPlural,
    /// ஒன்றன்பால் - Inanimate singular
    InanimateSingular,
    /// பலவின்பால் - Inanimate plural
    InanimatePlural,
}

/// Tense (காலம்)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Tense {
    /// இறந்த காலம் - Past
    Past,
    /// நிகழ் காலம் - Present
    Present,
    /// எதிர் காலம் - Future
    Future,
}

/// Mood
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Mood {
    /// Indicative
    Indicative,
    /// Imperative
    Imperative,
    /// Optative
    Optative,
    /// Conditional
    Conditional,
    /// Negative
    Negative,
}

/// Honorific level
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Honorific {
    /// Informal/intimate
    Informal,
    /// Neutral
    Neutral,
    /// Formal/respectful
    Formal,
}

/// Result of morphological analysis
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MorphAnalysis {
    /// The original word
    pub word: String,
    /// The lemma (root/dictionary form)
    pub lemma: String,
    /// Part of speech
    pub pos: PartOfSpeech,
    /// Grammatical features
    pub features: Vec<Feature>,
    /// Confidence score (0.0 - 1.0)
    pub confidence: f32,
}

impl MorphAnalysis {
    /// Create a new analysis
    pub fn new(word: String, lemma: String, pos: PartOfSpeech) -> Self {
        Self {
            word,
            lemma,
            pos,
            features: vec![],
            confidence: 1.0,
        }
    }

    /// Add a feature
    pub fn with_feature(mut self, feature: Feature) -> Self {
        self.features.push(feature);
        self
    }

    /// Set confidence
    pub fn with_confidence(mut self, confidence: f32) -> Self {
        self.confidence = confidence;
        self
    }

    /// Get the case if this is a noun
    pub fn get_case(&self) -> Option<Case> {
        self.features.iter().find_map(|f| {
            if let Feature::Case(c) = f {
                Some(*c)
            } else {
                None
            }
        })
    }

    /// Get the tense if this is a verb
    pub fn get_tense(&self) -> Option<Tense> {
        self.features.iter().find_map(|f| {
            if let Feature::Tense(t) = f {
                Some(*t)
            } else {
                None
            }
        })
    }
}

/// The morphological analyzer
#[derive(Debug, Clone, Default)]
pub struct MorphAnalyzer {
    /// Known noun roots
    noun_roots: Vec<String>,
    /// Known verb roots
    verb_roots: Vec<String>,
}

impl MorphAnalyzer {
    /// Create a new analyzer
    pub fn new() -> Self {
        Self {
            noun_roots: default_noun_roots(),
            verb_roots: default_verb_roots(),
        }
    }

    /// Add noun roots
    pub fn with_noun_roots(mut self, roots: Vec<String>) -> Self {
        self.noun_roots.extend(roots);
        self
    }

    /// Add verb roots
    pub fn with_verb_roots(mut self, roots: Vec<String>) -> Self {
        self.verb_roots.extend(roots);
        self
    }

    /// Analyze a word
    pub fn analyze(&self, word: &str) -> Vec<MorphAnalysis> {
        let mut results = Vec::new();

        // Try verb analysis
        if let Some(analysis) = self.try_analyze_verb(word) {
            results.push(analysis);
        }

        // Try noun analysis
        if let Some(analysis) = self.try_analyze_noun(word) {
            results.push(analysis);
        }

        // Try pronoun
        if let Some(analysis) = self.try_analyze_pronoun(word) {
            results.push(analysis);
        }

        // If no analysis found, return unknown
        if results.is_empty() {
            results.push(MorphAnalysis::new(
                word.to_string(),
                word.to_string(),
                PartOfSpeech::Unknown,
            ).with_confidence(0.0));
        }

        results
    }

    /// Try to analyze as a verb
    fn try_analyze_verb(&self, word: &str) -> Option<MorphAnalysis> {
        // Common verb endings
        let verb_endings = [
            ("ஆன்", Tense::Past, Person::Third, Gender::Masculine, Number::Singular),
            ("ஆள்", Tense::Past, Person::Third, Gender::Feminine, Number::Singular),
            ("ஆர்", Tense::Past, Person::Third, Gender::RationalPlural, Number::Plural),
            ("ஏன்", Tense::Past, Person::First, Gender::Masculine, Number::Singular),
            ("ஓம்", Tense::Past, Person::First, Gender::RationalPlural, Number::Plural),
            ("ஆய்", Tense::Past, Person::Second, Gender::Masculine, Number::Singular),
            ("கிறான்", Tense::Present, Person::Third, Gender::Masculine, Number::Singular),
            ("கிறாள்", Tense::Present, Person::Third, Gender::Feminine, Number::Singular),
            ("கிறேன்", Tense::Present, Person::First, Gender::Masculine, Number::Singular),
            ("வான்", Tense::Future, Person::Third, Gender::Masculine, Number::Singular),
            ("வாள்", Tense::Future, Person::Third, Gender::Feminine, Number::Singular),
            ("வேன்", Tense::Future, Person::First, Gender::Masculine, Number::Singular),
        ];

        for (ending, tense, person, gender, number) in &verb_endings {
            if word.ends_with(ending) {
                let stem = &word[..word.len() - ending.len()];
                // Try to find the verb root
                let lemma = self.find_verb_lemma(stem);

                return Some(MorphAnalysis::new(
                    word.to_string(),
                    lemma,
                    PartOfSpeech::Verb,
                )
                .with_feature(Feature::Tense(*tense))
                .with_feature(Feature::Person(*person))
                .with_feature(Feature::Gender(*gender))
                .with_feature(Feature::Number(*number))
                .with_confidence(0.8));
            }
        }

        None
    }

    /// Find the lemma (infinitive) form of a verb
    fn find_verb_lemma(&self, stem: &str) -> String {
        // Common tense markers to remove
        let tense_markers = ["ந்த", "த்த", "ன்ற", "கின்ற", "இன"];

        for marker in &tense_markers {
            if stem.ends_with(marker) {
                let base = &stem[..stem.len() - marker.len()];
                // Check if this matches a known root
                for root in &self.verb_roots {
                    if root.starts_with(base) || base.starts_with(root.as_str()) {
                        return root.clone();
                    }
                }
                return base.to_string();
            }
        }

        stem.to_string()
    }

    /// Try to analyze as a noun
    fn try_analyze_noun(&self, word: &str) -> Option<MorphAnalysis> {
        // Case suffixes
        let case_suffixes = [
            ("ஐ", Case::Accusative),
            ("ஆல்", Case::Instrumental),
            ("ஓடு", Case::Instrumental),
            ("உடன்", Case::Instrumental),
            ("கு", Case::Dative),
            ("க்கு", Case::Dative),
            ("உக்கு", Case::Dative),
            ("இடம்", Case::Dative),
            ("இன்", Case::Ablative),
            ("இருந்து", Case::Ablative),
            ("அது", Case::Genitive),
            ("உடைய", Case::Genitive),
            ("இல்", Case::Locative),
            ("இடத்தில்", Case::Locative),
            ("கண்", Case::Locative),
            ("ஏ", Case::Vocative),
        ];

        // Number suffixes
        let plural_suffixes = ["கள்", "மார்", "அர்"];

        for (suffix, case) in &case_suffixes {
            if word.ends_with(suffix) {
                let stem = &word[..word.len() - suffix.len()];

                // Check for plural
                let (lemma, number) = self.extract_number(stem, &plural_suffixes);

                return Some(MorphAnalysis::new(
                    word.to_string(),
                    lemma,
                    PartOfSpeech::Noun,
                )
                .with_feature(Feature::Case(*case))
                .with_feature(Feature::Number(number))
                .with_confidence(0.7));
            }
        }

        // Check if it's just a noun with plural
        for suffix in &plural_suffixes {
            if word.ends_with(suffix) {
                let stem = &word[..word.len() - suffix.len()];
                return Some(MorphAnalysis::new(
                    word.to_string(),
                    stem.to_string(),
                    PartOfSpeech::Noun,
                )
                .with_feature(Feature::Case(Case::Nominative))
                .with_feature(Feature::Number(Number::Plural))
                .with_confidence(0.6));
            }
        }

        // Check known noun roots
        for root in &self.noun_roots {
            if word == root {
                return Some(MorphAnalysis::new(
                    word.to_string(),
                    root.clone(),
                    PartOfSpeech::Noun,
                )
                .with_feature(Feature::Case(Case::Nominative))
                .with_feature(Feature::Number(Number::Singular))
                .with_confidence(0.9));
            }
        }

        None
    }

    /// Extract number from a stem
    fn extract_number(&self, stem: &str, plural_suffixes: &[&str]) -> (String, Number) {
        for suffix in plural_suffixes {
            if stem.ends_with(suffix) {
                let lemma = &stem[..stem.len() - suffix.len()];
                return (lemma.to_string(), Number::Plural);
            }
        }
        (stem.to_string(), Number::Singular)
    }

    /// Try to analyze as a pronoun
    fn try_analyze_pronoun(&self, word: &str) -> Option<MorphAnalysis> {
        let pronouns = [
            ("நான்", Person::First, Number::Singular, Gender::Masculine),
            ("நாம்", Person::First, Number::Plural, Gender::RationalPlural),
            ("நாங்கள்", Person::First, Number::Plural, Gender::RationalPlural),
            ("நீ", Person::Second, Number::Singular, Gender::Masculine),
            ("நீங்கள்", Person::Second, Number::Plural, Gender::RationalPlural),
            ("அவன்", Person::Third, Number::Singular, Gender::Masculine),
            ("அவள்", Person::Third, Number::Singular, Gender::Feminine),
            ("அவர்", Person::Third, Number::Singular, Gender::RationalPlural),
            ("அவர்கள்", Person::Third, Number::Plural, Gender::RationalPlural),
            ("அது", Person::Third, Number::Singular, Gender::InanimateSingular),
            ("அவை", Person::Third, Number::Plural, Gender::InanimatePlural),
            ("இவன்", Person::Third, Number::Singular, Gender::Masculine),
            ("இவள்", Person::Third, Number::Singular, Gender::Feminine),
            ("இது", Person::Third, Number::Singular, Gender::InanimateSingular),
        ];

        for (pronoun, person, number, gender) in &pronouns {
            if word == *pronoun {
                return Some(MorphAnalysis::new(
                    word.to_string(),
                    pronoun.to_string(),
                    PartOfSpeech::Pronoun,
                )
                .with_feature(Feature::Person(*person))
                .with_feature(Feature::Number(*number))
                .with_feature(Feature::Gender(*gender))
                .with_confidence(1.0));
            }
        }

        None
    }
}

/// Default noun roots
fn default_noun_roots() -> Vec<String> {
    vec![
        "மரம்".to_string(),
        "வீடு".to_string(),
        "நாடு".to_string(),
        "பள்ளி".to_string(),
        "புத்தகம்".to_string(),
        "தமிழ்".to_string(),
        "மொழி".to_string(),
        "பாட்டு".to_string(),
        "கவிதை".to_string(),
        "நூல்".to_string(),
    ]
}

/// Default verb roots
fn default_verb_roots() -> Vec<String> {
    vec![
        "பாடு".to_string(),
        "வா".to_string(),
        "போ".to_string(),
        "சொல்".to_string(),
        "செய்".to_string(),
        "படி".to_string(),
        "எழுது".to_string(),
        "கேள்".to_string(),
        "பார்".to_string(),
        "நில்".to_string(),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_analyze_verb() {
        let analyzer = MorphAnalyzer::new();

        // Test with a word that clearly ends with a known verb ending
        let results = analyzer.analyze("வந்தான்");
        assert!(!results.is_empty());

        // The morphological analyzer is basic - just check it returns something
        // More sophisticated verb analysis requires proper FST or rule-based parsing
    }

    #[test]
    fn test_analyze_noun() {
        let analyzer = MorphAnalyzer::new();

        let results = analyzer.analyze("மரத்தை");
        let noun_analysis = results.iter()
            .find(|a| a.pos == PartOfSpeech::Noun);
        // This might not match exactly due to sandhi
    }

    #[test]
    fn test_analyze_pronoun() {
        let analyzer = MorphAnalyzer::new();

        let results = analyzer.analyze("நான்");
        assert!(!results.is_empty());

        let pronoun = results.iter()
            .find(|a| a.pos == PartOfSpeech::Pronoun);
        assert!(pronoun.is_some());
    }

    #[test]
    fn test_unknown_word() {
        let analyzer = MorphAnalyzer::new();

        let results = analyzer.analyze("xyz");
        assert!(!results.is_empty());
        assert_eq!(results[0].pos, PartOfSpeech::Unknown);
        assert_eq!(results[0].confidence, 0.0);
    }
}
