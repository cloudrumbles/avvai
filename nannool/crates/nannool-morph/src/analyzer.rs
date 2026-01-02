//! Morphological analysis types and interfaces.

use serde::{Deserialize, Serialize};

/// Part of speech
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PartOfSpeech {
    /// பெயர்ச்சொல் - Noun
    Noun,
    /// வினைச்சொல் - Verb
    Verb,
    /// பெயரெச்சம் - Relative participle
    RelativeParticiple,
    /// வினையெச்சம் - Verbal participle
    VerbalParticiple,
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
            PartOfSpeech::RelativeParticiple => "பெயரெச்சம்",
            PartOfSpeech::VerbalParticiple => "வினையெச்சம்",
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
    /// Semantic tags (e.g., "tree", "direction", "rational")
    pub semantic_tags: Vec<String>,
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
            semantic_tags: vec![],
            confidence: 1.0,
        }
    }

    /// Add a feature
    pub fn with_feature(mut self, feature: Feature) -> Self {
        self.features.push(feature);
        self
    }

    /// Add a semantic tag
    pub fn with_semantic_tag(mut self, tag: impl Into<String>) -> Self {
        self.semantic_tags.push(tag.into());
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

    /// Get the person (இடம்) feature
    pub fn get_person(&self) -> Option<Person> {
        self.features.iter().find_map(|f| {
            if let Feature::Person(p) = f {
                Some(*p)
            } else {
                None
            }
        })
    }

    /// Get the number (எண்) feature
    pub fn get_number(&self) -> Option<Number> {
        self.features.iter().find_map(|f| {
            if let Feature::Number(n) = f {
                Some(*n)
            } else {
                None
            }
        })
    }

    /// Get the gender (பால்) feature
    pub fn get_gender(&self) -> Option<Gender> {
        self.features.iter().find_map(|f| {
            if let Feature::Gender(g) = f {
                Some(*g)
            } else {
                None
            }
        })
    }

    /// Get the honorific feature
    pub fn get_honorific(&self) -> Option<Honorific> {
        self.features.iter().find_map(|f| {
            if let Feature::Honorific(h) = f {
                Some(*h)
            } else {
                None
            }
        })
    }

    /// Get the mood feature
    pub fn get_mood(&self) -> Option<Mood> {
        self.features.iter().find_map(|f| {
            if let Feature::Mood(m) = f {
                Some(*m)
            } else {
                None
            }
        })
    }

    /// Check if this is a potential subject (noun/pronoun in nominative case)
    pub fn is_potential_subject(&self) -> bool {
        matches!(self.pos, PartOfSpeech::Noun | PartOfSpeech::Pronoun)
            && self.get_case().map_or(true, |c| c == Case::Nominative)
    }

    /// Check if this is a finite verb (has PNG markers)
    pub fn is_finite_verb(&self) -> bool {
        self.pos == PartOfSpeech::Verb
            && self.features.iter().any(|f| {
                matches!(
                    f,
                    Feature::Person(_) | Feature::Number(_) | Feature::Gender(_)
                )
            })
    }

    /// Check if this is a rational (உயர்திணை) gender
    pub fn is_rational(&self) -> bool {
        self.get_gender().map_or(false, |g| {
            matches!(
                g,
                Gender::Masculine | Gender::Feminine | Gender::RationalPlural
            )
        })
    }

    /// Check if this is an inanimate (அஃறிணை) gender
    pub fn is_inanimate(&self) -> bool {
        self.get_gender().map_or(false, |g| {
            matches!(g, Gender::InanimateSingular | Gender::InanimatePlural)
        })
    }
}
