//! FST wrapper for ThamizhiMorph using libfoma FFI.
//!
//! This module provides a wrapper around the Foma FST toolkit
//! using libfoma via FFI (no subprocess).

use std::path::{Path, PathBuf};

use crate::analyzer::{
    Case, Feature, Gender, Honorific, Mood, MorphAnalysis, Number, PartOfSpeech, Person, Tense,
};
use crate::error::FstError;
use crate::backend::FomaBackend;
use crate::bundled;
use crate::foma_ffi::FomaFfiAnalyzer;
use crate::lexicon::SemanticLexicon;

/// Wrapper around Foma for morphological analysis.
///
/// This struct manages one or more FST files and provides methods to
/// analyze Tamil words using libfoma via FFI.
pub struct FomaAnalyzer {
    /// Backend used for lookups (FFI only)
    backend: FomaBackend,
    /// Semantic lexicon for tagging words
    lexicon: SemanticLexicon,
}

impl FomaAnalyzer {
    /// Create a new FomaAnalyzer using bundled models.
    pub fn bundled() -> Result<Self, FstError> {
        let mut fst_paths = Vec::new();
        for (name, _) in bundled::BUNDLED_FSTS {
            fst_paths.push(bundled::get_bundled_fst_path(name));
        }
        Self::with_multiple_fsts(&fst_paths)
    }

    /// Create a new FomaAnalyzer with a single FST file.
    pub fn new<P: AsRef<Path>>(fst_path: P) -> Result<Self, FstError> {
        Self::with_multiple_fsts(&[fst_path])
    }

    /// Create a new FomaAnalyzer with multiple FST files.
    pub fn with_multiple_fsts<P: AsRef<Path>>(fst_paths: &[P]) -> Result<Self, FstError> {
        let paths: Vec<PathBuf> = fst_paths.iter().map(|p| p.as_ref().to_path_buf()).collect();

        for path in &paths {
            if !path.exists() {
                return Err(FstError::FstFileNotFound(path.clone()));
            }
        }

        // Load all FSTs via FFI
        let mut ffi_analyzers = Vec::new();
        for path in &paths {
            let analyzer = FomaFfiAnalyzer::new(path)?;
            ffi_analyzers.push(analyzer);
        }

        let mut analyzer = Self {
            backend: FomaBackend::new(ffi_analyzers),
            lexicon: SemanticLexicon::new(),
        };

        // Load default lexicons
        let _ = analyzer.load_default_lexicons();

        Ok(analyzer)
    }

    /// Load default lexicons from the data directory
    pub fn load_default_lexicons(&mut self) -> std::io::Result<()> {
        let bundled_lex_dir = bundled::get_bundled_lexicons_dir();
        let possible_paths = [
            bundled_lex_dir.to_str().unwrap_or(""),
            "/home/shah/workspace/avvai/nannool/data/thamizhi-morph/Lexicons",
            "data/thamizhi-morph/Lexicons",
            "nannool/data/thamizhi-morph/Lexicons",
            "../data/thamizhi-morph/Lexicons",
            "../../data/thamizhi-morph/Lexicons",
        ];

        for base in possible_paths {
            let path = Path::new(base);
            if path.exists() {
                let _ = self.lexicon.load_category("tree", path.join("Nouns-trees"));
                let _ = self.lexicon.load_category("animal", path.join("Nouns-animals"));
                let _ = self.lexicon.load_category("bird", path.join("Nouns-birds"));
                let _ = self.lexicon.load_category("flower", path.join("Nouns-flowers"));
                let _ = self.lexicon.load_category("direction", path.join("Nouns-directions"));
                let _ = self.lexicon.load_category("noun", path.join("Nouns-AUKBC"));
                let _ = self.lexicon.load_category("noun", path.join("Nouns-Open-Tamil-by Muthu Annamalai"));
                let _ = self.lexicon.load_category("proper_noun", path.join("Nouns-Propernouns"));
                break;
            }
        }

        Ok(())
    }

    /// Analyze a single word using the FST.
    pub fn analyze(&self, word: &str) -> Result<Vec<MorphAnalysis>, FstError> {
        let results = self.backend.analyze(word)?;
        let mut analyses = Vec::new();

        for res in results {
            if let Some(analysis) = self.parse_analysis_string(word, &res) {
                analyses.push(analysis);
            }
        }

        // If no FST results, try to create a fallback analysis using semantic lexicon
        if analyses.is_empty() {
            let tags = self.lexicon.get_tags(word);
            if !tags.is_empty() {
                let mut morph = MorphAnalysis::new(word.to_string(), word.to_string(), PartOfSpeech::Noun);
                morph.confidence = 0.5; // Lower confidence for non-FST results
                for tag in tags {
                    morph = morph.with_semantic_tag(tag);
                }
                analyses.push(morph);
            }
        }

        Ok(analyses)
    }

    /// Analyze multiple words in batch.
    pub fn analyze_batch(&self, words: &[&str]) -> Result<Vec<(String, Vec<MorphAnalysis>)>, FstError> {
        let mut batch_results = Vec::new();
        for &word in words {
            let analyses = self.analyze(word)?;
            batch_results.push((word.to_string(), analyses));
        }
        Ok(batch_results)
    }

    /// Generate inflected forms from a lemma and features.
    pub fn generate(
        &self,
        lemma: &str,
        pos: PartOfSpeech,
        features: &[Feature],
    ) -> Result<Vec<String>, FstError> {
        let query = self.build_generation_query(lemma, pos, features);
        let results = self.backend.generate(&query)?;
        
        let mut forms = Vec::new();
        for res in results {
            if res.contains('\t') {
                let parts: Vec<&str> = res.split('\t').collect();
                if parts.len() >= 2 {
                    forms.push(parts[1].trim().to_string());
                }
            } else {
                forms.push(res.trim().to_string());
            }
        }

        forms.sort();
        forms.dedup();
        Ok(forms)
    }

    /// Generate a full paradigm (all case forms) for a noun.
    pub fn generate_noun_paradigm(&self, lemma: &str) -> Result<Vec<(Case, Vec<String>)>, FstError> {
        let cases = [
            Case::Nominative,
            Case::Accusative,
            Case::Instrumental,
            Case::Dative,
            Case::Ablative,
            Case::Genitive,
            Case::Locative,
            Case::Vocative,
        ];

        let mut paradigm = Vec::new();
        for case in cases {
            let features = vec![Feature::Case(case)];
            let forms = self.generate(lemma, PartOfSpeech::Noun, &features)?;
            let forms = if forms.is_empty() && case == Case::Nominative {
                vec![lemma.to_string()]
            } else {
                forms
            };
            paradigm.push((case, forms));
        }
        Ok(paradigm)
    }

    /// Generate verb conjugation paradigm.
    pub fn generate_verb_paradigm(
        &self,
        lemma: &str,
        tense: Tense,
    ) -> Result<Vec<(Person, Number, Gender, Vec<String>)>, FstError> {
        let persons = [Person::First, Person::Second, Person::Third];
        let numbers = [Number::Singular, Number::Plural];
        let genders = [
            Gender::Masculine,
            Gender::Feminine,
            Gender::RationalPlural,
            Gender::InanimateSingular,
            Gender::InanimatePlural,
        ];

        let mut paradigm = Vec::new();
        for person in persons {
            for number in numbers {
                let gender_list: &[Gender] = if person == Person::Third {
                    &genders
                } else {
                    &[Gender::Masculine]
                };

                for &gender in gender_list {
                    let features = vec![
                        Feature::Tense(tense),
                        Feature::Person(person),
                        Feature::Number(number),
                        Feature::Gender(gender),
                    ];

                    let forms = self.generate(lemma, PartOfSpeech::Verb, &features)?;
                    if !forms.is_empty() {
                        paradigm.push((person, number, gender, forms));
                    }
                }
            }
        }
        Ok(paradigm)
    }

    // ... (rest of the parsing methods from previous version)
    
    /// Build a morphological specification string for generation.
    fn build_generation_query(&self, lemma: &str, pos: PartOfSpeech, features: &[Feature]) -> String {
        let mut parts = vec![lemma.to_string()];
        parts.push(self.pos_to_tag(pos));
        for feature in features {
            if let Some(tag) = self.feature_to_tag(feature) {
                parts.push(tag);
            }
        }
        parts.join("+")
    }

    fn pos_to_tag(&self, pos: PartOfSpeech) -> String {
        match pos {
            PartOfSpeech::Noun => "noun".to_string(),
            PartOfSpeech::Verb => "verb".to_string(),
            PartOfSpeech::Adjective => "adj".to_string(),
            PartOfSpeech::Adverb => "adv".to_string(),
            PartOfSpeech::Pronoun => "pron".to_string(),
            PartOfSpeech::Demonstrative => "dem".to_string(),
            PartOfSpeech::Interrogative => "interr".to_string(),
            PartOfSpeech::Particle => "part".to_string(),
            PartOfSpeech::Numeral => "num".to_string(),
            PartOfSpeech::RelativeParticiple => "rel".to_string(),
            PartOfSpeech::VerbalParticiple => "vbp".to_string(),
            PartOfSpeech::Attributive => "attr".to_string(),
            PartOfSpeech::Simile => "sim".to_string(),
            PartOfSpeech::Unknown => "unk".to_string(),
        }
    }

    fn feature_to_tag(&self, feature: &Feature) -> Option<String> {
        match feature {
            Feature::Case(case) => Some(match case {
                Case::Nominative => "nom".to_string(),
                Case::Accusative => "acc".to_string(),
                Case::Instrumental => "ins".to_string(),
                Case::Dative => "dat".to_string(),
                Case::Ablative => "abl".to_string(),
                Case::Genitive => "gen".to_string(),
                Case::Locative => "loc".to_string(),
                Case::Vocative => "voc".to_string(),
            }),
            Feature::Number(num) => Some(match num {
                Number::Singular => "sg".to_string(),
                Number::Plural => "pl".to_string(),
            }),
            Feature::Person(person) => Some(match person {
                Person::First => "1".to_string(),
                Person::Second => "2".to_string(),
                Person::Third => "3".to_string(),
            }),
            Feature::Gender(gender) => Some(match gender {
                Gender::Masculine => "m".to_string(),
                Gender::Feminine => "f".to_string(),
                Gender::RationalPlural => "hon".to_string(),
                Gender::InanimateSingular => "n".to_string(),
                Gender::InanimatePlural => "np".to_string(),
            }),
            Feature::Tense(tense) => Some(match tense {
                Tense::Past => "past".to_string(),
                Tense::Present => "pres".to_string(),
                Tense::Future => "fut".to_string(),
            }),
            Feature::Mood(mood) => Some(match mood {
                Mood::Indicative => "ind".to_string(),
                Mood::Imperative => "imp".to_string(),
                Mood::Optative => "opt".to_string(),
                Mood::Conditional => "cond".to_string(),
                Mood::Negative => "neg".to_string(),
            }),
            Feature::Honorific(hon) => Some(match hon {
                Honorific::Informal => "inf".to_string(),
                Honorific::Neutral => "neut".to_string(),
                Honorific::Formal => "form".to_string(),
            }),
            Feature::Suffix(s) => Some(s.clone()),
        }
    }

    fn parse_analysis_string(&self, word: &str, analysis: &str) -> Option<MorphAnalysis> {
        let parts: Vec<&str> = analysis.split('+').collect();
        if parts.is_empty() {
            return None;
        }

        let lemma = parts[0].to_string();
        let pos = if parts.len() > 1 {
            self.parse_pos(parts[1])
        } else {
            PartOfSpeech::Unknown
        };

        let mut morph = MorphAnalysis::new(word.to_string(), lemma, pos);
        morph.confidence = 1.0;

        for tag in parts.iter().skip(2) {
            let tag_name = if let Some(idx) = tag.find('=') {
                &tag[..idx]
            } else {
                tag
            };
            let tag_lower = tag_name.to_lowercase();

            match tag_lower.as_str() {
                "inf" | "vbp" | "vpart" | "nonfin" => {
                    if morph.pos == PartOfSpeech::Verb || morph.pos == PartOfSpeech::Unknown {
                        morph.pos = PartOfSpeech::VerbalParticiple;
                    }
                }
                "rel" | "rp" | "adjpart" => {
                    morph.pos = PartOfSpeech::RelativeParticiple;
                }
                _ => {}
            }

            if let Some(feature) = self.parse_tag(tag) {
                morph = morph.with_feature(feature);
            }
        }

        // Apply semantic tags from lexicon
        let tags = self.lexicon.get_tags(&morph.lemma);
        for tag in tags {
            morph = morph.with_semantic_tag(tag);
        }

        Some(morph)
    }

    fn parse_pos(&self, tag: &str) -> PartOfSpeech {
        match tag.to_lowercase().as_str() {
            "verb" | "v" | "vb" => PartOfSpeech::Verb,
            "noun" | "n" | "nn" => PartOfSpeech::Noun,
            "adj" | "adjective" | "jj" => PartOfSpeech::Adjective,
            "adv" | "adverb" | "rb" => PartOfSpeech::Adverb,
            "pron" | "pronoun" | "prp" => PartOfSpeech::Pronoun,
            "dem" | "demonstrative" => PartOfSpeech::Demonstrative,
            "interr" | "interrogative" | "wh" => PartOfSpeech::Interrogative,
            "part" | "particle" | "prt" => PartOfSpeech::Particle,
            "num" | "numeral" | "cd" => PartOfSpeech::Numeral,
            _ => PartOfSpeech::Unknown,
        }
    }

    fn parse_tag(&self, tag: &str) -> Option<Feature> {
        let tag_name = if let Some(idx) = tag.find('=') {
            &tag[..idx]
        } else {
            tag
        };
        let tag_lower = tag_name.to_lowercase();

        match tag_lower.as_str() {
            "past" | "pst" => return Some(Feature::Tense(Tense::Past)),
            "present" | "pres" | "prs" => return Some(Feature::Tense(Tense::Present)),
            "future" | "fut" => return Some(Feature::Tense(Tense::Future)),
            _ => {}
        }

        if let Some(features) = self.parse_agreement_tag(&tag_lower) {
            return features.into_iter().next();
        }

        match tag_lower.as_str() {
            "1" | "1st" | "first" => return Some(Feature::Person(Person::First)),
            "2" | "2nd" | "second" => return Some(Feature::Person(Person::Second)),
            "3" | "3rd" | "third" => return Some(Feature::Person(Person::Third)),
            "sg" | "singular" | "sing" => return Some(Feature::Number(Number::Singular)),
            "pl" | "plural" | "plur" => return Some(Feature::Number(Number::Plural)),
            "m" | "masc" | "masculine" => return Some(Feature::Gender(Gender::Masculine)),
            "f" | "fem" | "feminine" => return Some(Feature::Gender(Gender::Feminine)),
            "n" | "neut" | "neuter" => return Some(Feature::Gender(Gender::InanimateSingular)),
            "hon" | "honorific" | "polite" => return Some(Feature::Gender(Gender::RationalPlural)),
            "nom" | "nominative" => return Some(Feature::Case(Case::Nominative)),
            "acc" | "accusative" => return Some(Feature::Case(Case::Accusative)),
            "dat" | "dative" => return Some(Feature::Case(Case::Dative)),
            "gen" | "genitive" => return Some(Feature::Case(Case::Genitive)),
            "loc" | "locative" => return Some(Feature::Case(Case::Locative)),
            "abl" | "ablative" => return Some(Feature::Case(Case::Ablative)),
            "ins" | "inst" | "instrumental" => return Some(Feature::Case(Case::Instrumental)),
            "voc" | "vocative" => return Some(Feature::Case(Case::Vocative)),
            "ind" | "indicative" => return Some(Feature::Mood(Mood::Indicative)),
            "imp" | "imperative" => return Some(Feature::Mood(Mood::Imperative)),
            "opt" | "optative" => return Some(Feature::Mood(Mood::Optative)),
            "cond" | "conditional" => return Some(Feature::Mood(Mood::Conditional)),
            "neg" | "negative" => return Some(Feature::Mood(Mood::Negative)),
            "fin" | "inf" | "sim" | "strong" | "weak" | "caus" | "pass" | "refl"
            | "infinc" | "inc" | "comp" | "rel" | "adj" | "adv" => return None,
            _ => {}
        }

        Some(Feature::Suffix(tag.to_string()))
    }

    fn parse_agreement_tag(&self, tag: &str) -> Option<Vec<Feature>> {
        let mut features = Vec::new();
        let (person, rest) = if tag.starts_with('1') {
            (Some(Person::First), &tag[1..])
        } else if tag.starts_with('2') {
            (Some(Person::Second), &tag[1..])
        } else if tag.starts_with('3') {
            (Some(Person::Third), &tag[1..])
        } else {
            (None, tag)
        };

        let Some(p) = person else {
            return None;
        };
        features.push(Feature::Person(p));

        let rest = if rest.starts_with("sg") {
            features.push(Feature::Number(Number::Singular));
            &rest[2..]
        } else if rest.starts_with("pl") {
            features.push(Feature::Number(Number::Plural));
            &rest[2..]
        } else {
            rest
        };

        if rest.starts_with('m') || rest == "masc" {
            features.push(Feature::Gender(Gender::Masculine));
        } else if rest.starts_with('f') || rest == "fem" {
            features.push(Feature::Gender(Gender::Feminine));
        } else if rest.starts_with('n') || rest == "neut" {
            features.push(Feature::Gender(Gender::InanimateSingular));
        } else if rest == "hon" || rest == "h" {
            features.push(Feature::Gender(Gender::RationalPlural));
        }

        if features.len() > 1 {
            Some(features)
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn dummy_analyzer() -> FomaAnalyzer {
        FomaAnalyzer {
            backend: FomaBackend::new(vec![]),
            lexicon: SemanticLexicon::new(),
        }
    }

    #[test]
    fn test_parse_pos() {
        let analyzer = dummy_analyzer();
        assert_eq!(analyzer.parse_pos("verb"), PartOfSpeech::Verb);
        assert_eq!(analyzer.parse_pos("noun"), PartOfSpeech::Noun);
    }

    #[test]
    fn test_parse_tag() {
        let analyzer = dummy_analyzer();
        assert!(matches!(
            analyzer.parse_tag("past"),
            Some(Feature::Tense(Tense::Past))
        ));
    }
}
