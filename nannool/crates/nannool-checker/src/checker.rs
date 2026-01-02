//! Main sandhi checking logic.
//!
//! Validates Tamil text against Nannool sandhi rules and
//! produces diagnostics for violations.

use nannool_rules::{RuleEngine, SandhiRule, StrictnessLevel};
use nannool_morph::MorphAnalyzer;
use crate::tokenizer::{tokenize, Token, TokenKind, Span};
use crate::diagnostic::{Diagnostic, DiagnosticLevel, Suggestion};

/// The main sandhi checker
#[derive(Debug)]
pub struct SandhiChecker {
    /// Rule engine
    rule_engine: RuleEngine,
    /// Morphological analyzer
    morph_analyzer: MorphAnalyzer,
    /// Strictness level
    strictness: StrictnessLevel,
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
            morph_analyzer: MorphAnalyzer::new(),
            strictness: StrictnessLevel::Standard,
        }
    }

    /// Create a checker with a specific strictness level
    pub fn with_strictness(strictness: StrictnessLevel) -> Self {
        Self {
            rule_engine: RuleEngine::new().with_strictness(strictness),
            morph_analyzer: MorphAnalyzer::new(),
            strictness,
        }
    }

    /// Set the strictness level
    pub fn set_strictness(&mut self, level: StrictnessLevel) {
        self.strictness = level;
        self.rule_engine = RuleEngine::new().with_strictness(level);
    }

    /// Check text for sandhi violations
    pub fn check(&self, text: &str) -> Vec<Diagnostic> {
        let tokens = tokenize(text);
        let mut diagnostics = Vec::new();

        // Get Tamil word tokens
        let tamil_words: Vec<&Token> = tokens
            .iter()
            .filter(|t| t.kind == TokenKind::TamilWord)
            .collect();

        // Check each word boundary
        for window in tamil_words.windows(2) {
            let word1 = window[0];
            let word2 = window[1];

            if let Some(diag) = self.check_boundary(word1, word2, text) {
                diagnostics.push(diag);
            }
        }

        diagnostics
    }

    /// Check a single word boundary
    fn check_boundary(&self, word1: &Token, word2: &Token, source: &str) -> Option<Diagnostic> {
        // Check for violations
        let violation = self.rule_engine.check_violation(&word1.text, &word2.text)?;

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
    pub fn check_pair(&self, word1: &str, word2: &str) -> Option<Diagnostic> {
        let violation = self.rule_engine.check_violation(word1, word2)?;

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
    pub fn is_valid(&self, word1: &str, word2: &str) -> bool {
        self.rule_engine.check_violation(word1, word2).is_none()
    }

    /// Get a suggested correction for a word pair
    pub fn suggest_correction(&self, word1: &str, word2: &str) -> Option<String> {
        self.rule_engine.suggest_fix(word1, word2)
    }

    /// Analyze morphology of a word
    pub fn analyze_word(&self, word: &str) -> Vec<nannool_morph::MorphAnalysis> {
        self.morph_analyzer.analyze(word)
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
        let checker = SandhiChecker::new();

        // This should violate vallinam rule
        let diagnostics = checker.check("பாட்டு பாடினான்");

        // We expect at least one diagnostic for the vallinam violation
        assert!(!diagnostics.is_empty());
    }

    #[test]
    fn test_check_pair() {
        let checker = SandhiChecker::new();

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
        let checker = SandhiChecker::new();

        // Incorrect sandhi
        assert!(!checker.is_valid("பாட்டு", "பாடினான்"));

        // Correct sandhi (hypothetically - with proper doubling)
        // Note: actual validation depends on rule implementation
    }

    #[test]
    fn test_suggest_correction() {
        let checker = SandhiChecker::new();

        let suggestion = checker.suggest_correction("பாட்டு", "பாடினான்");
        assert!(suggestion.is_some());
    }

    #[test]
    fn test_strictness_levels() {
        let strict = SandhiChecker::with_strictness(StrictnessLevel::Classical);
        let lenient = SandhiChecker::with_strictness(StrictnessLevel::Lenient);

        let text = "பாட்டு பாடினான்";

        let strict_diags = strict.check(text);
        let lenient_diags = lenient.check(text);

        // Strict should catch at least as many as lenient
        assert!(strict_diags.len() >= lenient_diags.len());
    }

    #[test]
    fn test_check_result() {
        let checker = SandhiChecker::new();
        let diagnostics = checker.check("பாட்டு பாடினான்");
        let result = CheckResult::new(diagnostics);

        assert!(result.has_errors());
        assert!(result.error_count() > 0);
    }
}
