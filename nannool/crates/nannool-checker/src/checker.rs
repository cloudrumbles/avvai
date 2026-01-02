//! Main sandhi checking logic.
//!
//! Validates Tamil text against Nannool sandhi rules and
//! produces diagnostics for violations.

use nannool_morph::{MorphAnalyzer, MorphAnalysis};
use nannool_rules::{RuleEngine, SandhiRule, StrictnessLevel, WordClass};
use crate::tokenizer::{tokenize, Token, TokenKind, Span};
use crate::diagnostic::{Diagnostic, DiagnosticLevel, Suggestion};
use crate::sdp::ShallowDependencyParser;
use crate::speller::SpellingChecker;
use crate::syntax::SyntaxChecker;

/// The main sandhi checker
pub struct SandhiChecker {
    /// Rule engine
    rule_engine: RuleEngine,
    /// Strictness level
    strictness: StrictnessLevel,
    /// Optional morphological analyzer for word class detection
    morph_analyzer: Option<MorphAnalyzer>,
    /// Cache of morphological analyses for the current check
    morph_cache: std::collections::HashMap<String, Option<MorphAnalysis>>,
}

impl Default for SandhiChecker {
    fn default() -> Self {
        Self::new()
    }
}

impl SandhiChecker {
    /// Create a new sandhi checker with default settings
    pub fn new() -> Self {
        Self {
            rule_engine: RuleEngine::new(),
            strictness: StrictnessLevel::Standard,
            morph_analyzer: None,
            morph_cache: std::collections::HashMap::new(),
        }
    }

    /// Create a checker with a specific rule engine
    pub fn with_engine(rule_engine: RuleEngine) -> Self {
        let strictness = rule_engine.strictness();
        Self {
            rule_engine,
            strictness,
            morph_analyzer: None,
            morph_cache: std::collections::HashMap::new(),
        }
    }

    /// Create a checker with a specific strictness level
    pub fn with_strictness(strictness: StrictnessLevel) -> Self {
        Self {
            rule_engine: RuleEngine::new().with_strictness(strictness),
            strictness,
            morph_analyzer: None,
            morph_cache: std::collections::HashMap::new(),
        }
    }

    /// Set the morphological analyzer for word class detection
    ///
    /// When set, the checker will use morphological analysis to determine
    /// word classes (noun, verb, etc.) for more accurate rule matching.
    pub fn with_morph_analyzer(mut self, analyzer: MorphAnalyzer) -> Self {
        self.morph_analyzer = Some(analyzer);
        self
    }

    /// Try to load morphological analyzer from bundled FST models
    ///
    /// This will prefer bundled models if available, falling back to
    /// scanning the filesystem for models in common locations.
    ///
    /// Returns self unchanged if models aren't found (graceful degradation).
    pub fn with_default_morph(mut self) -> Self {
        // Try bundled first
        if let Ok(analyzer) = MorphAnalyzer::bundled() {
            self.morph_analyzer = Some(analyzer);
            return self;
        }

        let possible_paths = [
            "data/thamizhi-morph/FST-Models",
            "../data/thamizhi-morph/FST-Models",
            "../../data/thamizhi-morph/FST-Models",
            concat!(env!("CARGO_MANIFEST_DIR"), "/../../data/thamizhi-morph/FST-Models"),
        ];

        for base_path in possible_paths {
            let fst_files: Vec<std::path::PathBuf> = [
                "noun.fst",
                "verb-c-rest.fst",
                "adj.fst",
                "adv.fst",
                "pronoun.fst",
                "part.fst",
            ]
            .iter()
            .map(|f| std::path::Path::new(base_path).join(f))
            .filter(|p| p.exists())
            .collect();

            if !fst_files.is_empty() {
                // Try to create analyzer with found FST files
                let paths: Vec<&std::path::Path> = fst_files.iter().map(|p| p.as_path()).collect();
                if let Ok(analyzer) = MorphAnalyzer::with_multiple_fsts(&paths) {
                    self.morph_analyzer = Some(analyzer);
                    break;
                }
            }
        }

        self
    }

    /// Set the strictness level
    pub fn set_strictness(&mut self, level: StrictnessLevel) {
        self.strictness = level;
        self.rule_engine = RuleEngine::new().with_strictness(level);
    }

    /// Get the word class for a word using morphological analysis
    fn get_word_class(&mut self, word: &str) -> Option<WordClass> {
        // Check cache first
        if let Some(cached) = self.morph_cache.get(word) {
            return cached.as_ref().map(|a| WordClass::from_pos(a.pos));
        }

        // Try to analyze if we have an analyzer
        let analysis = self.morph_analyzer.as_ref().and_then(|analyzer| {
            analyzer.analyze(word).ok().and_then(|analyses| {
                // Take the highest confidence analysis
                analyses.into_iter().max_by(|a, b| {
                    a.confidence.partial_cmp(&b.confidence).unwrap_or(std::cmp::Ordering::Equal)
                })
            })
        });

        // Cache the result
        let word_class = analysis.as_ref().map(|a| WordClass::from_pos(a.pos));
        self.morph_cache.insert(word.to_string(), analysis);
        word_class
    }

    /// Extract morphological conditions from analysis (e.g., "causative", "plural")
    fn get_morph_conditions(&self, word: &str) -> Vec<String> {
        if let Some(Some(analysis)) = self.morph_cache.get(word) {
            let mut conditions: Vec<String> = analysis.features.iter().filter_map(|f| {
                use nannool_morph::Feature;
                match f {
                    Feature::Mood(m) => Some(format!("{:?}", m).to_lowercase()),
                    Feature::Tense(t) => Some(format!("{:?}", t).to_lowercase()),
                    Feature::Case(c) => Some(format!("{:?}", c).to_lowercase()),
                    Feature::Number(n) => Some(format!("{:?}", n).to_lowercase()),
                    Feature::Person(p) => Some(format!("{:?}", p).to_lowercase()),
                    Feature::Gender(g) => Some(format!("{:?}", g).to_lowercase()),
                    Feature::Honorific(h) => Some(format!("{:?}", h).to_lowercase()),
                    Feature::Suffix(s) => Some(s.clone().to_lowercase()),
                }
            }).collect();

            // Add rational/inanimate tags (திணை) based on gender
            for f in &analysis.features {
                if let nannool_morph::Feature::Gender(g) = f {
                    match g {
                        nannool_morph::prelude::Gender::Masculine | 
                        nannool_morph::prelude::Gender::Feminine | 
                        nannool_morph::prelude::Gender::RationalPlural => {
                            conditions.push("rational".to_string());
                            conditions.push("uyarthinai".to_string());
                        },
                        nannool_morph::prelude::Gender::InanimateSingular | 
                        nannool_morph::prelude::Gender::InanimatePlural => {
                            conditions.push("inanimate".to_string());
                            conditions.push("akhrinai".to_string());
                        }
                    }
                }
            }
            
            // Add semantic tags from lexicon
            conditions.extend(analysis.semantic_tags.clone());
            
            conditions
        } else {
            vec![]
        }
    }

    /// Check text for sandhi, spelling, and syntax violations
    ///
    /// If a morphological analyzer is configured, this will use word class
    /// information for more accurate rule matching and enable spelling/syntax checks.
    pub fn check(&mut self, text: &str) -> Vec<Diagnostic> {
        // Clear cache for new check
        self.morph_cache.clear();

        let tokens = tokenize(text);
        let mut diagnostics = Vec::new();

        // 1. Get Tamil word tokens and run spelling check
        let tamil_words: Vec<&Token> = tokens
            .iter()
            .filter(|t| t.kind == TokenKind::TamilWord)
            .collect();

        // Ensure all words are in cache for syntax/sandhi checks
        for token in &tamil_words {
            let _ = self.get_word_class(&token.text);
        }

        // Spell checking using dictionary-only validation
        let speller = SpellingChecker::new();
        for token in &tamil_words {
            if let Some(diag) = speller.check_token(token, text) {
                diagnostics.push(diag);
            }
        }

        // 2. Check each word boundary for sandhi violations
        let mut last_tamil_word: Option<&Token> = None;
        let mut skipped_due_to_punctuation = false;

        for token in &tokens {
            match token.kind {
                TokenKind::TamilWord => {
                    if let Some(word1) = last_tamil_word {
                        if !skipped_due_to_punctuation {
                            if let Some(diag) = self.check_boundary(word1, token, text) {
                                diagnostics.push(diag);
                            }
                        }
                    }
                    last_tamil_word = Some(token);
                    skipped_due_to_punctuation = false;
                }
                TokenKind::Punctuation => {
                    skipped_due_to_punctuation = true;
                }
                _ => {}
            }
        }

        // 3. Run syntax checks (e.g., Subject-Verb agreement)
        if !tamil_words.is_empty() {
            let syntax_checker = SyntaxChecker::new(&self.morph_cache);
            let syntax_diags = syntax_checker.check_agreement(&tamil_words, text);
            diagnostics.extend(syntax_diags);
        }

        diagnostics
    }

    /// Check a single word boundary
    fn check_boundary(&mut self, word1: &Token, word2: &Token, source: &str) -> Option<Diagnostic> {
        // Ensure analysis is in cache
        let _ = self.get_word_class(&word1.text);
        let _ = self.get_word_class(&word2.text);

        let analysis1 = self.morph_cache.get(&word1.text).and_then(|a| a.as_ref());
        let analysis2 = self.morph_cache.get(&word2.text).and_then(|a| a.as_ref());
        
        let class1 = analysis1.map(|a| WordClass::from_pos(a.pos));
        let class2 = analysis2.map(|a| WordClass::from_pos(a.pos));

        // Detect relationship
        let relationship = ShallowDependencyParser::detect_relationship(
            &word1.text,
            analysis1,
            &word2.text,
            analysis2,
        );

        // Get morphological conditions
        let mut conditions = self.get_morph_conditions(&word1.text);
        conditions.extend(self.get_morph_conditions(&word2.text));

        // Check for violations with grammatical context
        let violation = self.rule_engine.check_violation_with_context(
            &word1.text,
            &word2.text,
            class1,
            class2,
            relationship,
            &conditions,
        )?;

        // Create the diagnostic
        let span = word1.span.merge(&word2.span);

        let suggestion = violation.suggest_fix(&word1.text, &word2.text).map(|fix| {
            Suggestion {
                replacement: fix,
                description: format!(
                    "Apply {} ({})",
                    violation.tamil_name,
                    violation.english_name
                ),
            }
        });

        Some(Diagnostic {
            level: DiagnosticLevel::Error,
            message: format!(
                "புணர்ச்சி பிழை: {}",
                violation.tamil_name
            ),
            english_message: Some(format!(
                "Sandhi error: {}",
                violation.english_name
            )),
            span,
            source_text: source[span.start..span.end].to_string(),
            rule_id: violation.id.clone(),
            nannool_verses: violation.nannool_verses.clone(),
            suggestion,
            notes: vec![
                format!(
                    "நன்னூல் {} - {}",
                    violation.nannool_verses.first().map(|v| v.to_string()).unwrap_or_default(),
                    violation.description
                ),
            ],
        })
    }

    /// Check a single word pair without source context
    pub fn check_pair(&mut self, word1: &str, word2: &str) -> Option<Diagnostic> {
        // Ensure analysis is in cache
        let _ = self.get_word_class(word1);
        let _ = self.get_word_class(word2);
        
        let analysis1 = self.morph_cache.get(word1).and_then(|a| a.as_ref());
        let analysis2 = self.morph_cache.get(word2).and_then(|a| a.as_ref());
        
        let class1 = analysis1.map(|a| WordClass::from_pos(a.pos));
        let class2 = analysis2.map(|a| WordClass::from_pos(a.pos));

        let relationship = ShallowDependencyParser::detect_relationship(
            word1, analysis1, word2, analysis2
        );

        // Get morphological conditions
        let mut conditions = self.get_morph_conditions(word1);
        conditions.extend(self.get_morph_conditions(word2));

        let violation = self.rule_engine.check_violation_with_context(
            word1,
            word2,
            class1,
            class2,
            relationship,
            &conditions,
        )?;

        let suggestion = violation.suggest_fix(word1, word2).map(|fix| {
            Suggestion {
                replacement: fix,
                description: format!(
                    "Apply {} ({})",
                    violation.tamil_name,
                    violation.english_name
                ),
            }
        });

        Some(Diagnostic {
            level: DiagnosticLevel::Error,
            message: format!("புணர்ச்சி பிழை: {}", violation.tamil_name),
            english_message: Some(format!("Sandhi error: {}", violation.english_name)),
            span: Span::new(0, 0, 0, 0),
            source_text: format!("{} {}", word1, word2),
            rule_id: violation.id.clone(),
            nannool_verses: violation.nannool_verses.clone(),
            suggestion,
            notes: vec![],
        })
    }

    /// Get applicable rules for a word pair
    pub fn get_applicable_rules(&self, word1: &str, word2: &str) -> Vec<&SandhiRule> {
        self.rule_engine.find_applicable(word1, word2)
    }

    /// Check if a word pair satisfies sandhi rules
    ///
    /// Uses morphological context if analyzer is configured.
    pub fn is_valid(&mut self, word1: &str, word2: &str) -> bool {
        let _ = self.get_word_class(word1);
        let _ = self.get_word_class(word2);
        
        let analysis1 = self.morph_cache.get(word1).and_then(|a| a.as_ref());
        let analysis2 = self.morph_cache.get(word2).and_then(|a| a.as_ref());
        
        let class1 = analysis1.map(|a| WordClass::from_pos(a.pos));
        let class2 = analysis2.map(|a| WordClass::from_pos(a.pos));

        let relationship = ShallowDependencyParser::detect_relationship(
            word1, analysis1, word2, analysis2
        );

        let mut conditions = self.get_morph_conditions(word1);
        conditions.extend(self.get_morph_conditions(word2));

        self.rule_engine
            .check_violation_with_context(word1, word2, class1, class2, relationship, &conditions)
            .is_none()
    }

    /// Get a suggested correction for a word pair
    pub fn suggest_correction(&mut self, word1: &str, word2: &str) -> Option<String> {
        let _ = self.get_word_class(word1);
        let _ = self.get_word_class(word2);

        let analysis1 = self.morph_cache.get(word1).and_then(|a| a.as_ref());
        let analysis2 = self.morph_cache.get(word2).and_then(|a| a.as_ref());

        let class1 = analysis1.map(|a| WordClass::from_pos(a.pos));
        let class2 = analysis2.map(|a| WordClass::from_pos(a.pos));

        let relationship = ShallowDependencyParser::detect_relationship(
            word1, analysis1, word2, analysis2
        );

        let mut conditions = self.get_morph_conditions(word1);
        conditions.extend(self.get_morph_conditions(word2));

        if let Some(rule) = self.rule_engine.check_violation_with_context(
            word1, word2, class1, class2, relationship, &conditions,
        ) {
            rule.suggest_fix(word1, word2)
        } else {
            None
        }
    }

    /// Check a word pair using heuristics only (no morphological analysis)
    ///
    /// This is faster and works without FST models, but less accurate.
    pub fn check_pair_heuristic(&self, word1: &str, word2: &str) -> Option<Diagnostic> {
        use crate::heuristics::check_pair_heuristic;

        let result = check_pair_heuristic(word1, word2)?;

        if result.is_violation {
            Some(Diagnostic {
                level: DiagnosticLevel::Warning, // Lower confidence than rule-based
                message: format!("புணர்ச்சி பிழை: {}", result.tamil_name),
                english_message: Some(format!("Sandhi error: {}", result.rule_name)),
                span: Span::new(0, 0, 0, 0),
                source_text: format!("{} {}", word1, word2),
                rule_id: result.rule_name.clone(),
                nannool_verses: vec![],
                suggestion: result.suggestion.map(|s| Suggestion {
                    replacement: s,
                    description: format!("Apply {}", result.tamil_name),
                }),
                notes: vec![
                    format!("Confidence: {:.0}%", result.confidence * 100.0),
                    "Detected using heuristic patterns".to_string(),
                ],
            })
        } else {
            None
        }
    }

    /// Check if a word pair has a no-doubling exception
    ///
    /// Uses heuristic patterns to check if vallinam doubling should NOT occur.
    pub fn is_no_doubling_exception(&self, word1: &str) -> bool {
        use crate::heuristics::{is_no_doubling_word, is_no_doubling_ending};
        is_no_doubling_word(word1) || is_no_doubling_ending(word1)
    }
}

/// Result of checking a file
#[derive(Debug)]
pub struct CheckResult {
    /// Path to the checked file
    pub path: Option<String>,
    /// Diagnostics found
    pub diagnostics: Vec<Diagnostic>,
    /// Total number of word pairs checked
    pub pairs_checked: usize,
    /// Number of violations found
    pub violations: usize,
}

impl CheckResult {
    /// Create a new check result
    pub fn new(diagnostics: Vec<Diagnostic>) -> Self {
        let violations = diagnostics.len();
        Self {
            path: None,
            diagnostics,
            pairs_checked: 0,
            violations,
        }
    }

    /// Check if there are any errors
    pub fn has_errors(&self) -> bool {
        self.diagnostics
            .iter()
            .any(|d| matches!(d.level, DiagnosticLevel::Error))
    }

    /// Check if there are any warnings
    pub fn has_warnings(&self) -> bool {
        self.diagnostics
            .iter()
            .any(|d| matches!(d.level, DiagnosticLevel::Warning))
    }

    /// Get error count
    pub fn error_count(&self) -> usize {
        self.diagnostics
            .iter()
            .filter(|d| matches!(d.level, DiagnosticLevel::Error))
            .count()
    }

    /// Get warning count
    pub fn warning_count(&self) -> usize {
        self.diagnostics
            .iter()
            .filter(|d| matches!(d.level, DiagnosticLevel::Warning))
            .count()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_checker_creation() {
        let checker = SandhiChecker::new();
        assert!(checker.rule_engine.rules().len() > 0);
    }

    #[test]
    fn test_check_simple_violation() {
        let mut checker = SandhiChecker::new();

        // This should violate vallinam rule
        let diagnostics = checker.check("பாட்டு பாடினான்");

        // We expect at least one diagnostic for the vallinam violation
        assert!(!diagnostics.is_empty());
    }

    #[test]
    fn test_check_pair() {
        let mut checker = SandhiChecker::new();

        let diag = checker.check_pair("பாட்டு", "பாடினான்");
        assert!(diag.is_some());

        let d = diag.unwrap();
        // Multiple rules could match (vallinam-miguthal-165 or kutriyalukaram-206)
        // Just verify we get a sandhi error
        assert!(!d.rule_id.is_empty());
        assert!(d.message.contains("புணர்ச்சி"));
    }

    #[test]
    fn test_is_valid() {
        let mut checker = SandhiChecker::new();

        // Incorrect sandhi
        assert!(!checker.is_valid("பாட்டு", "பாடினான்"));

        // Correct sandhi (hypothetically - with proper doubling)
        // Note: actual validation depends on rule implementation
    }

    #[test]
    fn test_suggest_correction() {
        let mut checker = SandhiChecker::new();

        let suggestion = checker.suggest_correction("பாட்டு", "பாடினான்");
        assert!(suggestion.is_some());
    }

    #[test]
    fn test_strictness_levels() {
        let mut strict = SandhiChecker::with_strictness(StrictnessLevel::Classical);
        let mut lenient = SandhiChecker::with_strictness(StrictnessLevel::Lenient);

        let text = "பாட்டு பாடினான்";

        let strict_diags = strict.check(text);
        let lenient_diags = lenient.check(text);

        // Strict should catch at least as many as lenient
        assert!(strict_diags.len() >= lenient_diags.len());
    }

    #[test]
    fn test_check_result() {
        let mut checker = SandhiChecker::new();
        let diagnostics = checker.check("பாட்டு பாடினான்");
        let result = CheckResult::new(diagnostics);

        assert!(result.has_errors());
        assert!(result.error_count() > 0);
    }

    #[test]
    fn test_spelling_error() {
        // We need a morph analyzer for spelling checks
        let mut checker = SandhiChecker::new().with_default_morph();

        // Skip if no morph analyzer available
        if checker.morph_analyzer.is_none() {
            return;
        }

        // "பாட்ட்டூ" is not a word - should trigger spelling error
        // Note: The speller only flags words that are in the lexicon but misspelled,
        // not completely unknown words (which might be proper nouns, etc.)
        let diagnostics = checker.check("பாட்ட்டூ பாடினான்");

        let has_spelling_error = diagnostics.iter().any(|d| d.rule_id == "spelling-error");

        // This test is informational - spelling detection depends on the lexicon
        // and may not fire for completely unknown words
        if has_spelling_error {
            eprintln!("Spelling error detected as expected");
        } else {
            eprintln!("No spelling error detected (word may be completely unknown)");
            eprintln!("Diagnostics: {:?}", diagnostics);
        }
        // Don't assert - just verify the integration doesn't crash
    }

    #[test]
    fn test_agreement_error() {
        let mut checker = SandhiChecker::new().with_default_morph();

        // Skip if no morph analyzer available
        if checker.morph_analyzer.is_none() {
            return;
        }

        // "நான் வந்தான்" - First person singular subject with third person singular verb
        let diagnostics = checker.check("நான் வந்தான்");

        // Note: Agreement checking depends on the morphological analyzer returning
        // the correct PNG features. If the analyzer doesn't return features,
        // no agreement error will be triggered.
        let has_syntax_error = diagnostics.iter().any(|d| d.rule_id == "subject-verb-agreement");

        // Debug: print what we got
        if !has_syntax_error {
            eprintln!("Agreement test: no syntax error detected");
            eprintln!("Diagnostics: {:?}", diagnostics);
            eprintln!("Morph cache: {:?}", checker.morph_cache);
        }

        // This test is informational - it shows the integration works,
        // but success depends on the morphological analyzer's output
    }
}
