//! Syntax checker for Tamil sentences.
//!
//! Validates grammatical agreement (Subject-Verb, etc.) based on
//! Tamil grammar rules (இயைபிலக்கணம்).
//!
//! ## Tamil Agreement System
//!
//! Tamil verbs agree with their subjects in Person (இடம்), Number (எண்),
//! and Gender (பால்), collectively known as PNG agreement.
//!
//! ### Person (இடம்)
//! - தன்மை (First person): நான், நாங்கள், நாம்
//! - முன்னிலை (Second person): நீ, நீங்கள், நீர்
//! - படர்க்கை (Third person): அவன், அவள், அவர், அது, அவை
//!
//! ### Number (எண்)
//! - ஒருமை (Singular)
//! - பன்மை (Plural)
//!
//! ### Gender (பால்) - Only for 3rd person
//! - ஆண்பால் (Masculine): அவன் வந்தான்
//! - பெண்பால் (Feminine): அவள் வந்தாள்
//! - பலர்பால் (Rational plural): அவர்கள் வந்தார்கள்
//! - ஒன்றன்பால் (Inanimate singular): அது வந்தது
//! - பலவின்பால் (Inanimate plural): அவை வந்தன

use crate::diagnostic::{Diagnostic, DiagnosticLevel, Suggestion};
use crate::tokenizer::Token;
use nannool_morph::{MorphAnalysis, prelude::*};
use std::collections::HashMap;

/// Agreement features extracted from a word
#[derive(Debug, Clone)]
pub struct AgreementFeatures {
    pub person: Option<Person>,
    pub number: Option<Number>,
    pub gender: Option<Gender>,
    pub honorific: Option<Honorific>,
}

impl AgreementFeatures {
    /// Extract agreement features from a morphological analysis
    pub fn from_analysis(analysis: &MorphAnalysis) -> Self {
        Self {
            person: analysis.get_person(),
            number: analysis.get_number(),
            gender: analysis.get_gender(),
            honorific: analysis.get_honorific(),
        }
    }

    /// Check if all features are unspecified
    pub fn is_empty(&self) -> bool {
        self.person.is_none() && self.number.is_none() && self.gender.is_none()
    }

    /// Format features for display
    pub fn format_tamil(&self) -> String {
        let mut parts = Vec::new();

        if let Some(p) = self.person {
            parts.push(match p {
                Person::First => "தன்மை",
                Person::Second => "முன்னிலை",
                Person::Third => "படர்க்கை",
            });
        }

        if let Some(n) = self.number {
            parts.push(match n {
                Number::Singular => "ஒருமை",
                Number::Plural => "பன்மை",
            });
        }

        if let Some(g) = self.gender {
            parts.push(match g {
                Gender::Masculine => "ஆண்பால்",
                Gender::Feminine => "பெண்பால்",
                Gender::RationalPlural => "பலர்பால்",
                Gender::InanimateSingular => "ஒன்றன்பால்",
                Gender::InanimatePlural => "பலவின்பால்",
            });
        }

        if parts.is_empty() {
            "அறியாத".to_string()
        } else {
            parts.join(", ")
        }
    }

    /// Format features for display (English)
    pub fn format_english(&self) -> String {
        let mut parts = Vec::new();

        if let Some(p) = self.person {
            parts.push(match p {
                Person::First => "1st",
                Person::Second => "2nd",
                Person::Third => "3rd",
            });
        }

        if let Some(n) = self.number {
            parts.push(match n {
                Number::Singular => "sg",
                Number::Plural => "pl",
            });
        }

        if let Some(g) = self.gender {
            parts.push(match g {
                Gender::Masculine => "masc",
                Gender::Feminine => "fem",
                Gender::RationalPlural => "rat.pl",
                Gender::InanimateSingular => "neut.sg",
                Gender::InanimatePlural => "neut.pl",
            });
        }

        if parts.is_empty() {
            "unknown".to_string()
        } else {
            parts.join(".")
        }
    }
}

/// Result of agreement check
#[derive(Debug, Clone)]
pub struct AgreementMismatch {
    pub person_mismatch: bool,
    pub number_mismatch: bool,
    pub gender_mismatch: bool,
    pub thiṇai_mismatch: bool, // உயர்திணை vs அஃறிணை
}

impl AgreementMismatch {
    /// Check if there's any mismatch
    pub fn has_mismatch(&self) -> bool {
        self.person_mismatch || self.number_mismatch || self.gender_mismatch || self.thiṇai_mismatch
    }

    /// Get mismatch descriptions in Tamil
    pub fn descriptions_tamil(&self) -> Vec<&'static str> {
        let mut desc = Vec::new();
        if self.person_mismatch {
            desc.push("இடம் (Person)");
        }
        if self.number_mismatch {
            desc.push("எண் (Number)");
        }
        if self.gender_mismatch {
            desc.push("பால் (Gender)");
        }
        if self.thiṇai_mismatch {
            desc.push("திணை (Rational/Inanimate)");
        }
        desc
    }
}

/// Pronoun features lookup table
/// Maps pronoun text -> (Person, Number, Gender)
fn get_pronoun_features(word: &str) -> Option<(Person, Number, Option<Gender>)> {
    match word {
        // First person
        "நான்" => Some((Person::First, Number::Singular, None)),
        "நாங்கள்" | "நாம்" => Some((Person::First, Number::Plural, None)),
        // Second person
        "நீ" => Some((Person::Second, Number::Singular, None)),
        "நீங்கள்" | "நீர்" => Some((Person::Second, Number::Plural, None)),
        // Third person masculine
        "அவன்" => Some((Person::Third, Number::Singular, Some(Gender::Masculine))),
        // Third person feminine
        "அவள்" => Some((Person::Third, Number::Singular, Some(Gender::Feminine))),
        // Third person honorific/plural
        "அவர்" => Some((Person::Third, Number::Singular, Some(Gender::RationalPlural))),
        "அவர்கள்" => Some((Person::Third, Number::Plural, Some(Gender::RationalPlural))),
        // Inanimate
        "அது" | "இது" => Some((Person::Third, Number::Singular, Some(Gender::InanimateSingular))),
        "அவை" | "இவை" => Some((Person::Third, Number::Plural, Some(Gender::InanimatePlural))),
        _ => None,
    }
}

/// Extract gender from Tamil verb ending
fn get_verb_gender(word: &str) -> Option<Gender> {
    // Check verb endings for gender markers
    if word.ends_with("ான்") || word.ends_with("ான்") {
        Some(Gender::Masculine)  // -aan (he did)
    } else if word.ends_with("ாள்") || word.ends_with("ாள்") {
        Some(Gender::Feminine)   // -aaL (she did)
    } else if word.ends_with("ார்") || word.ends_with("ார்கள்") {
        Some(Gender::RationalPlural) // -aar/-aargaL (they did, honorific)
    } else if word.ends_with("து") || word.ends_with("றது") {
        Some(Gender::InanimateSingular) // -thu (it did)
    } else if word.ends_with("ன") || word.ends_with("றன") {
        Some(Gender::InanimatePlural) // -na (they[inanimate] did)
    } else if word.ends_with("ேன்") {
        None // -en (I did) - first person, no gender
    } else if word.ends_with("ோம்") {
        None // -om (we did) - first person plural
    } else if word.ends_with("ாய்") {
        None // -aay (you did) - second person singular
    } else if word.ends_with("ீர்கள்") || word.ends_with("ீர்") {
        None // -irgaL (you did) - second person plural
    } else {
        None
    }
}

/// Checker for sentence-level syntax errors.
pub struct SyntaxChecker<'a> {
    /// Cache of morphological analyses
    analyses: &'a HashMap<String, Option<MorphAnalysis>>,
}

impl<'a> SyntaxChecker<'a> {
    /// Create a new syntax checker
    pub fn new(analyses: &'a HashMap<String, Option<MorphAnalysis>>) -> Self {
        Self { analyses }
    }

    /// Get enhanced agreement features with pronoun lookup
    fn get_enhanced_features(&self, token: &Token, analysis: &MorphAnalysis) -> AgreementFeatures {
        let mut features = AgreementFeatures::from_analysis(analysis);

        // If this is a pronoun, use lookup table to fill in missing features
        if analysis.pos == nannool_morph::PartOfSpeech::Pronoun {
            if let Some((person, number, gender)) = get_pronoun_features(&token.text) {
                if features.person.is_none() { features.person = Some(person); }
                if features.number.is_none() { features.number = Some(number); }
                if features.gender.is_none() { features.gender = gender; }
            }
        }

        // If this is a verb, try to detect gender from ending
        if analysis.pos == nannool_morph::PartOfSpeech::Verb {
            if features.gender.is_none() {
                features.gender = get_verb_gender(&token.text);
            }
        }

        features
    }

    /// Check for Subject-Verb agreement in a list of tokens (single sentence)
    pub fn check_agreement(&self, tokens: &[&Token], source: &str) -> Vec<Diagnostic> {
        // Split into sentences first
        let sentences = self.split_sentences(tokens);

        let mut diagnostics = Vec::new();
        for sentence in sentences {
            if let Some(diag) = self.check_sentence_agreement(&sentence, source) {
                diagnostics.push(diag);
            }
        }

        diagnostics
    }

    /// Split tokens into sentences based on punctuation
    fn split_sentences<'b>(&self, tokens: &[&'b Token]) -> Vec<Vec<&'b Token>> {
        let mut sentences = Vec::new();
        let mut current = Vec::new();

        for token in tokens {
            current.push(*token);
            // Tamil sentence enders: . (.), ! (!), ? (?), । (danda)
            if token.text == "." || token.text == "!" || token.text == "?" || token.text == "।" {
                if !current.is_empty() {
                    sentences.push(std::mem::take(&mut current));
                }
            }
        }

        // Don't forget the last sentence if no terminator
        if !current.is_empty() {
            sentences.push(current);
        }

        sentences
    }

    /// Check agreement for a single sentence
    fn check_sentence_agreement(&self, tokens: &[&Token], source: &str) -> Option<Diagnostic> {
        // 1. Find subject and verb candidates
        let (subject, verb) = self.find_subject_verb(tokens)?;

        // 2. Get enhanced features (with pronoun lookup and verb ending detection)
        let s_features = self.get_enhanced_features(subject.0, subject.1);
        let v_features = self.get_enhanced_features(verb.0, verb.1);

        // 3. Check agreement using enhanced features
        let mismatch = self.check_features_agreement(&s_features, &v_features);

        if mismatch.has_mismatch() {

            let suggestion = self.suggest_correction(subject.1, verb.1, &mismatch);

            Some(Diagnostic {
                level: DiagnosticLevel::Error,
                message: format!(
                    "இலக்கணப் பிழை: எழுவாய்-பயனிலை இயைபுப் பிழை ({})",
                    mismatch.descriptions_tamil().join(", ")
                ),
                english_message: Some(format!(
                    "Grammar error: Subject-Verb agreement mismatch ({})",
                    mismatch.descriptions_tamil().join(", ")
                )),
                span: verb.0.span, // Flag the verb
                source_text: source
                    .get(verb.0.span.start..verb.0.span.end)
                    .unwrap_or(&verb.0.text)
                    .to_string(),
                rule_id: "subject-verb-agreement".to_string(),
                nannool_verses: vec![], // Could reference specific verses
                suggestion,
                notes: vec![
                    format!(
                        "எழுவாய்: '{}' ({}) [{}]",
                        subject.0.text,
                        s_features.format_tamil(),
                        s_features.format_english()
                    ),
                    format!(
                        "பயனிலை: '{}' ({}) [{}]",
                        verb.0.text,
                        v_features.format_tamil(),
                        v_features.format_english()
                    ),
                ],
            })
        } else {
            None
        }
    }

    /// Find subject and verb in a sentence
    ///
    /// Tamil is typically SOV (Subject-Object-Verb), so:
    /// - Subject is usually the first nominative noun/pronoun
    /// - Verb is usually the last finite verb
    fn find_subject_verb<'b>(
        &self,
        tokens: &[&'b Token],
    ) -> Option<((&'b Token, &'a MorphAnalysis), (&'b Token, &'a MorphAnalysis))> {
        let mut subject: Option<(&Token, &MorphAnalysis)> = None;
        let mut verb: Option<(&Token, &MorphAnalysis)> = None;

        for token in tokens {
            if let Some(Some(analysis)) = self.analyses.get(&token.text) {
                // Check for subject candidate
                if analysis.is_potential_subject() && subject.is_none() {
                    subject = Some((*token, analysis));
                }

                // Check for verb candidate (take the last one in SOV)
                if analysis.is_finite_verb() {
                    verb = Some((*token, analysis));
                }
            }
        }

        match (subject, verb) {
            (Some(s), Some(v)) => Some((s, v)),
            _ => None,
        }
    }

    /// Check Person, Number, and Gender agreement using enhanced features
    fn check_features_agreement(
        &self,
        subject: &AgreementFeatures,
        verb: &AgreementFeatures,
    ) -> AgreementMismatch {
        // Person agreement (must match if both specified)
        let person_mismatch = match (subject.person, verb.person) {
            (Some(sp), Some(vp)) => sp != vp,
            _ => false,
        };

        // Number agreement
        let number_mismatch = match (subject.number, verb.number) {
            (Some(sn), Some(vn)) => sn != vn,
            _ => false,
        };

        // Gender agreement (complex rules for Tamil)
        let (gender_mismatch, thiṇai_mismatch) = self.check_gender_agreement(subject.gender, verb.gender);

        AgreementMismatch {
            person_mismatch,
            number_mismatch,
            gender_mismatch,
            thiṇai_mismatch,
        }
    }

    /// Check Person, Number, and Gender agreement (deprecated, use check_features_agreement)
    #[allow(dead_code)]
    fn check_png_agreement(
        &self,
        subject: &MorphAnalysis,
        verb: &MorphAnalysis,
    ) -> AgreementMismatch {
        let s_person = subject.get_person();
        let s_number = subject.get_number();
        let s_gender = subject.get_gender();

        let v_person = verb.get_person();
        let v_number = verb.get_number();
        let v_gender = verb.get_gender();

        // Person agreement (must match if both specified)
        let person_mismatch = match (s_person, v_person) {
            (Some(sp), Some(vp)) => sp != vp,
            _ => false,
        };

        // Number agreement
        let number_mismatch = match (s_number, v_number) {
            (Some(sn), Some(vn)) => sn != vn,
            _ => false,
        };

        // Gender agreement (complex rules for Tamil)
        let (gender_mismatch, thiṇai_mismatch) = self.check_gender_agreement(s_gender, v_gender);

        AgreementMismatch {
            person_mismatch,
            number_mismatch,
            gender_mismatch,
            thiṇai_mismatch,
        }
    }

    /// Check gender agreement with Tamil-specific rules
    ///
    /// Tamil gender agreement rules:
    /// 1. Rational (உயர்திணை) genders must not mix with inanimate (அஃறிணை)
    /// 2. Masculine singular verb can only go with masculine subject
    /// 3. Feminine singular verb can only go with feminine subject
    /// 4. Rational plural verb can go with any rational plural subject
    /// 5. Inanimate singular verb goes with inanimate singular subject
    /// 6. Inanimate plural verb goes with inanimate plural subject
    fn check_gender_agreement(
        &self,
        subject_gender: Option<Gender>,
        verb_gender: Option<Gender>,
    ) -> (bool, bool) {
        let (sg, vg) = match (subject_gender, verb_gender) {
            (Some(s), Some(v)) => (s, v),
            _ => return (false, false), // Can't check if not specified
        };

        // Check திணை (rational vs inanimate) first
        let s_rational = matches!(sg, Gender::Masculine | Gender::Feminine | Gender::RationalPlural);
        let v_rational = matches!(vg, Gender::Masculine | Gender::Feminine | Gender::RationalPlural);
        let s_inanimate = matches!(sg, Gender::InanimateSingular | Gender::InanimatePlural);
        let v_inanimate = matches!(vg, Gender::InanimateSingular | Gender::InanimatePlural);

        // திணை mismatch: rational subject with inanimate verb or vice versa
        let thiṇai_mismatch = (s_rational && v_inanimate) || (s_inanimate && v_rational);

        if thiṇai_mismatch {
            return (false, true);
        }

        // Within the same திணை, check பால்
        let gender_mismatch = match (sg, vg) {
            // Exact match is always fine
            (a, b) if a == b => false,

            // Rational plural verb can agree with masculine/feminine plural subjects
            // (This is a simplification - the subject would have பலர்பால் if plural)
            (Gender::Masculine | Gender::Feminine, Gender::RationalPlural) => true, // singular subj, plural verb
            (Gender::RationalPlural, Gender::Masculine | Gender::Feminine) => true, // plural subj, singular verb

            // Masculine/Feminine shouldn't swap
            (Gender::Masculine, Gender::Feminine) | (Gender::Feminine, Gender::Masculine) => true,

            // Inanimate singular/plural shouldn't swap
            (Gender::InanimateSingular, Gender::InanimatePlural)
            | (Gender::InanimatePlural, Gender::InanimateSingular) => true,

            // All other cases within same திணை
            _ => false,
        };

        (gender_mismatch, false)
    }

    /// Suggest a correction for the verb based on subject's features
    fn suggest_correction(
        &self,
        subject: &MorphAnalysis,
        verb: &MorphAnalysis,
        _mismatch: &AgreementMismatch,
    ) -> Option<Suggestion> {
        // This is a simplified suggestion system
        // A full implementation would require a morphological generator

        let s_features = AgreementFeatures::from_analysis(subject);

        // Generate description of expected verb form
        let expected = format!(
            "'{}'க்கு ஏற்ற வினைமுற்று ({} வேண்டும்)",
            subject.word,
            s_features.format_tamil()
        );

        // For common cases, we can provide specific suggestions
        let replacement = self.generate_verb_form(verb, &s_features)?;

        Some(Suggestion {
            replacement,
            description: expected,
        })
    }

    /// Generate the correct verb form (simplified)
    ///
    /// This is a rule-based approximation. A proper implementation would use
    /// the morphological generator from ThamizhiMorph.
    fn generate_verb_form(&self, verb: &MorphAnalysis, target: &AgreementFeatures) -> Option<String> {
        // Get the verb stem (lemma)
        let stem = &verb.lemma;

        // Common verb ending patterns
        let ending = match (target.person, target.number, target.gender) {
            // First person
            (Some(Person::First), Some(Number::Singular), _) => "ஏன்", // வந்தேன்
            (Some(Person::First), Some(Number::Plural), _) => "ஓம்",   // வந்தோம்

            // Second person
            (Some(Person::Second), Some(Number::Singular), _) => "ஆய்", // வந்தாய்
            (Some(Person::Second), Some(Number::Plural), _) => "ீர்கள்", // வந்தீர்கள்

            // Third person masculine singular
            (Some(Person::Third), Some(Number::Singular), Some(Gender::Masculine)) => "ஆன்",

            // Third person feminine singular
            (Some(Person::Third), Some(Number::Singular), Some(Gender::Feminine)) => "ஆள்",

            // Third person rational plural
            (Some(Person::Third), _, Some(Gender::RationalPlural)) => "ஆர்கள்",

            // Third person inanimate singular
            (Some(Person::Third), Some(Number::Singular), Some(Gender::InanimateSingular)) => "அது", // Placeholder - needs tense marker

            // Third person inanimate plural
            (Some(Person::Third), _, Some(Gender::InanimatePlural)) => "அன", // வந்தன

            // Default - can't generate
            _ => return None,
        };

        // This is very simplified - real morphology is much more complex
        // (tense markers, sandhi rules, irregular verbs, etc.)
        Some(format!("{}{}", stem, ending))
    }

    /// Check for common pronoun-verb patterns
    ///
    /// This provides quick lookups for common subject pronouns and their
    /// expected verb agreement patterns.
    #[allow(dead_code)]
    fn get_pronoun_agreement(&self, pronoun: &str) -> Option<AgreementFeatures> {
        // Common Tamil pronouns and their features
        let features = match pronoun {
            // First person singular
            "நான்" => AgreementFeatures {
                person: Some(Person::First),
                number: Some(Number::Singular),
                gender: None, // 1st person doesn't mark gender
                honorific: None,
            },
            // First person plural (exclusive)
            "நாங்கள்" => AgreementFeatures {
                person: Some(Person::First),
                number: Some(Number::Plural),
                gender: None,
                honorific: None,
            },
            // First person plural (inclusive)
            "நாம்" => AgreementFeatures {
                person: Some(Person::First),
                number: Some(Number::Plural),
                gender: None,
                honorific: None,
            },
            // Second person singular (informal)
            "நீ" => AgreementFeatures {
                person: Some(Person::Second),
                number: Some(Number::Singular),
                gender: None,
                honorific: Some(Honorific::Informal),
            },
            // Second person plural/formal
            "நீங்கள்" => AgreementFeatures {
                person: Some(Person::Second),
                number: Some(Number::Plural),
                gender: None,
                honorific: Some(Honorific::Formal),
            },
            // Second person singular (respectful)
            "நீர்" => AgreementFeatures {
                person: Some(Person::Second),
                number: Some(Number::Singular),
                gender: None,
                honorific: Some(Honorific::Neutral),
            },
            // Third person masculine singular
            "அவன்" => AgreementFeatures {
                person: Some(Person::Third),
                number: Some(Number::Singular),
                gender: Some(Gender::Masculine),
                honorific: Some(Honorific::Informal),
            },
            // Third person feminine singular
            "அவள்" => AgreementFeatures {
                person: Some(Person::Third),
                number: Some(Number::Singular),
                gender: Some(Gender::Feminine),
                honorific: Some(Honorific::Informal),
            },
            // Third person singular (respectful)
            "அவர்" => AgreementFeatures {
                person: Some(Person::Third),
                number: Some(Number::Singular),
                gender: Some(Gender::RationalPlural), // Takes plural agreement
                honorific: Some(Honorific::Formal),
            },
            // Third person plural (rational)
            "அவர்கள்" => AgreementFeatures {
                person: Some(Person::Third),
                number: Some(Number::Plural),
                gender: Some(Gender::RationalPlural),
                honorific: None,
            },
            // Third person inanimate singular
            "அது" => AgreementFeatures {
                person: Some(Person::Third),
                number: Some(Number::Singular),
                gender: Some(Gender::InanimateSingular),
                honorific: None,
            },
            // Third person inanimate plural
            "அவை" => AgreementFeatures {
                person: Some(Person::Third),
                number: Some(Number::Plural),
                gender: Some(Gender::InanimatePlural),
                honorific: None,
            },
            _ => return None,
        };

        Some(features)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tokenizer::Span;

    fn mock_analysis(
        word: &str,
        pos: PartOfSpeech,
        person: Option<Person>,
        number: Option<Number>,
        gender: Option<Gender>,
    ) -> MorphAnalysis {
        let mut analysis = MorphAnalysis::new(word.to_string(), word.to_string(), pos);
        if let Some(p) = person {
            analysis = analysis.with_feature(Feature::Person(p));
        }
        if let Some(n) = number {
            analysis = analysis.with_feature(Feature::Number(n));
        }
        if let Some(g) = gender {
            analysis = analysis.with_feature(Feature::Gender(g));
        }
        analysis
    }

    fn mock_token(text: &str, start: usize) -> Token {
        Token::new(
            text.to_string(),
            Span::new(start, start + text.len(), 1, (start + 1) as u32),
            crate::tokenizer::TokenKind::TamilWord,
        )
    }

    #[test]
    fn test_agreement_features_from_analysis() {
        let analysis = mock_analysis(
            "வந்தான்",
            PartOfSpeech::Verb,
            Some(Person::Third),
            Some(Number::Singular),
            Some(Gender::Masculine),
        );
        let features = AgreementFeatures::from_analysis(&analysis);

        assert_eq!(features.person, Some(Person::Third));
        assert_eq!(features.number, Some(Number::Singular));
        assert_eq!(features.gender, Some(Gender::Masculine));
    }

    #[test]
    fn test_correct_agreement() {
        // அவன் வந்தான் - 3rd person masculine singular, should agree
        let mut cache = HashMap::new();
        cache.insert(
            "அவன்".to_string(),
            Some(mock_analysis(
                "அவன்",
                PartOfSpeech::Pronoun,
                Some(Person::Third),
                Some(Number::Singular),
                Some(Gender::Masculine),
            )),
        );
        cache.insert(
            "வந்தான்".to_string(),
            Some(mock_analysis(
                "வந்தான்",
                PartOfSpeech::Verb,
                Some(Person::Third),
                Some(Number::Singular),
                Some(Gender::Masculine),
            )),
        );

        let checker = SyntaxChecker::new(&cache);
        let tokens = vec![mock_token("அவன்", 0), mock_token("வந்தான்", 8)];
        let token_refs: Vec<&Token> = tokens.iter().collect();

        let diagnostics = checker.check_agreement(&token_refs, "அவன் வந்தான்");
        assert!(diagnostics.is_empty(), "Should not have errors for correct agreement");
    }

    #[test]
    fn test_person_mismatch() {
        // நான் வந்தான் - 1st person subject with 3rd person verb
        let mut cache = HashMap::new();
        cache.insert(
            "நான்".to_string(),
            Some(mock_analysis(
                "நான்",
                PartOfSpeech::Pronoun,
                Some(Person::First),
                Some(Number::Singular),
                None,
            )),
        );
        cache.insert(
            "வந்தான்".to_string(),
            Some(mock_analysis(
                "வந்தான்",
                PartOfSpeech::Verb,
                Some(Person::Third),
                Some(Number::Singular),
                Some(Gender::Masculine),
            )),
        );

        let checker = SyntaxChecker::new(&cache);
        let tokens = vec![mock_token("நான்", 0), mock_token("வந்தான்", 8)];
        let token_refs: Vec<&Token> = tokens.iter().collect();

        let diagnostics = checker.check_agreement(&token_refs, "நான் வந்தான்");
        assert_eq!(diagnostics.len(), 1);
        assert!(diagnostics[0].message.contains("இடம்"));
    }

    #[test]
    fn test_gender_mismatch() {
        // அவள் வந்தான் - feminine subject with masculine verb
        let mut cache = HashMap::new();
        cache.insert(
            "அவள்".to_string(),
            Some(mock_analysis(
                "அவள்",
                PartOfSpeech::Pronoun,
                Some(Person::Third),
                Some(Number::Singular),
                Some(Gender::Feminine),
            )),
        );
        cache.insert(
            "வந்தான்".to_string(),
            Some(mock_analysis(
                "வந்தான்",
                PartOfSpeech::Verb,
                Some(Person::Third),
                Some(Number::Singular),
                Some(Gender::Masculine),
            )),
        );

        let checker = SyntaxChecker::new(&cache);
        let tokens = vec![mock_token("அவள்", 0), mock_token("வந்தான்", 8)];
        let token_refs: Vec<&Token> = tokens.iter().collect();

        let diagnostics = checker.check_agreement(&token_refs, "அவள் வந்தான்");
        assert_eq!(diagnostics.len(), 1);
        assert!(diagnostics[0].message.contains("பால்"));
    }

    #[test]
    fn test_thinai_mismatch() {
        // அவன் வந்தது - rational subject with inanimate verb
        let mut cache = HashMap::new();
        cache.insert(
            "அவன்".to_string(),
            Some(mock_analysis(
                "அவன்",
                PartOfSpeech::Pronoun,
                Some(Person::Third),
                Some(Number::Singular),
                Some(Gender::Masculine),
            )),
        );
        cache.insert(
            "வந்தது".to_string(),
            Some(mock_analysis(
                "வந்தது",
                PartOfSpeech::Verb,
                Some(Person::Third),
                Some(Number::Singular),
                Some(Gender::InanimateSingular),
            )),
        );

        let checker = SyntaxChecker::new(&cache);
        let tokens = vec![mock_token("அவன்", 0), mock_token("வந்தது", 8)];
        let token_refs: Vec<&Token> = tokens.iter().collect();

        let diagnostics = checker.check_agreement(&token_refs, "அவன் வந்தது");
        assert_eq!(diagnostics.len(), 1);
        assert!(diagnostics[0].message.contains("திணை"));
    }

    #[test]
    fn test_number_mismatch() {
        // நாங்கள் வந்தேன் - plural subject with singular verb
        let mut cache = HashMap::new();
        cache.insert(
            "நாங்கள்".to_string(),
            Some(mock_analysis(
                "நாங்கள்",
                PartOfSpeech::Pronoun,
                Some(Person::First),
                Some(Number::Plural),
                None,
            )),
        );
        cache.insert(
            "வந்தேன்".to_string(),
            Some(mock_analysis(
                "வந்தேன்",
                PartOfSpeech::Verb,
                Some(Person::First),
                Some(Number::Singular),
                None,
            )),
        );

        let checker = SyntaxChecker::new(&cache);
        let tokens = vec![mock_token("நாங்கள்", 0), mock_token("வந்தேன்", 16)];
        let token_refs: Vec<&Token> = tokens.iter().collect();

        let diagnostics = checker.check_agreement(&token_refs, "நாங்கள் வந்தேன்");
        assert_eq!(diagnostics.len(), 1);
        assert!(diagnostics[0].message.contains("எண்"));
    }

    #[test]
    fn test_agreement_features_format() {
        let features = AgreementFeatures {
            person: Some(Person::Third),
            number: Some(Number::Singular),
            gender: Some(Gender::Masculine),
            honorific: None,
        };

        assert!(features.format_tamil().contains("படர்க்கை"));
        assert!(features.format_tamil().contains("ஒருமை"));
        assert!(features.format_tamil().contains("ஆண்பால்"));

        assert!(features.format_english().contains("3rd"));
        assert!(features.format_english().contains("sg"));
        assert!(features.format_english().contains("masc"));
    }
}
