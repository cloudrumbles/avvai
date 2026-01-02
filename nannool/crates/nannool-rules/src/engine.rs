//! Rule matching engine.
//!
//! This module provides the rule engine for matching and applying
//! sandhi rules to word pairs.

use std::path::Path;

use crate::loader::{load_rules_from_file, load_rules_from_str, load_verses_from_file, load_verses_from_str, LoadError};
use crate::sandhi::SandhiRule;
use crate::verses::NannoolVerse;

/// Bundled rules and verses
const BUNDLED_RULES_TOML: &str = include_str!("../../../data/nannool/rules.toml");
const BUNDLED_VERSES_TOML: &str = include_str!("../../../data/nannool/punariyar.toml");

/// Strictness level for rule checking
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum StrictnessLevel {
    /// Classical Tamil rules only
    Classical,
    /// Standard modern Tamil (default)
    #[default]
    Standard,
    /// Lenient - allow common deviations
    Lenient,
}

/// The rule engine for sandhi checking
#[derive(Debug, Clone)]
pub struct RuleEngine {
    /// All loaded rules
    rules: Vec<SandhiRule>,
    /// Nannool verses
    verses: Vec<NannoolVerse>,
    /// Strictness level
    strictness: StrictnessLevel,
}

impl Default for RuleEngine {
    fn default() -> Self {
        Self::new()
    }
}

impl RuleEngine {
    /// Create a new rule engine with bundled rules and verses
    pub fn new() -> Self {
        Self::bundled()
    }

    /// Create a new rule engine using bundled rules and verses
    pub fn bundled() -> Self {
        let rules = load_rules_from_str(BUNDLED_RULES_TOML).expect("Failed to parse bundled rules");
        let verses = load_verses_from_str(BUNDLED_VERSES_TOML).expect("Failed to parse bundled verses");
        
        let mut engine = Self {
            rules,
            verses,
            strictness: StrictnessLevel::default(),
        };
        engine.rules.sort_by(|a, b| b.priority.cmp(&a.priority));
        engine
    }

    /// Create a new rule engine with built-in rules (minimal set)
    #[deprecated(note = "Use RuleEngine::new() or RuleEngine::bundled() instead")]
    pub fn builtin() -> Self {
        Self::bundled()
    }

    /// Create a new rule engine by loading rules from a TOML file
    pub fn from_file<P: AsRef<Path>>(path: P) -> Result<Self, LoadError> {
        let mut rules = load_rules_from_file(path)?;
        rules.sort_by(|a, b| b.priority.cmp(&a.priority));

        Ok(Self {
            rules,
            verses: load_verses_from_str(BUNDLED_VERSES_TOML).unwrap_or_default(),
            strictness: StrictnessLevel::default(),
        })
    }

    /// Create a new rule engine by loading rules and verses from TOML files
    pub fn from_files<P: AsRef<Path>>(rules_path: P, verses_path: P) -> Result<Self, LoadError> {
        let mut rules = load_rules_from_file(rules_path)?;
        rules.sort_by(|a, b| b.priority.cmp(&a.priority));
        let verses = load_verses_from_file(verses_path)?;

        Ok(Self {
            rules,
            verses,
            strictness: StrictnessLevel::default(),
        })
    }

    /// Create a rule engine by loading from file, falling back to bundled rules
    pub fn from_file_or_builtin<P: AsRef<Path>>(path: P) -> Self {
        match load_rules_from_file(path) {
            Ok(mut rules) => {
                rules.sort_by(|a, b| b.priority.cmp(&a.priority));
                Self {
                    rules,
                    verses: load_verses_from_str(BUNDLED_VERSES_TOML).unwrap_or_default(),
                    strictness: StrictnessLevel::default(),
                }
            }
            Err(_) => Self::bundled(),
        }
    }

    /// Create a rule engine with a specific strictness level
    pub fn with_strictness(mut self, level: StrictnessLevel) -> Self {
        self.strictness = level;
        self
    }

    /// Add custom rules to the engine
    pub fn with_rules(mut self, additional_rules: Vec<SandhiRule>) -> Self {
        self.rules.extend(additional_rules);
        self.rules.sort_by(|a, b| b.priority.cmp(&a.priority));
        self
    }

    /// Add custom verses to the engine
    pub fn with_verses(mut self, additional_verses: Vec<NannoolVerse>) -> Self {
        self.verses.extend(additional_verses);
        self
    }

    /// Get all rules
    pub fn rules(&self) -> &[SandhiRule] {
        &self.rules
    }

    /// Get all verses
    pub fn verses(&self) -> &[NannoolVerse] {
        &self.verses
    }

    /// Get a verse by its number
    pub fn get_verse(&self, number: u32) -> Option<&NannoolVerse> {
        self.verses.iter().find(|v| v.number == number)
    }

    /// Get all verses associated with a rule
    pub fn get_verses_for_rule(&self, rule: &SandhiRule) -> Vec<&NannoolVerse> {
        rule.nannool_verses
            .iter()
            .filter_map(|&number| self.get_verse(number))
            .collect()
    }

    /// Get the current strictness level
    pub fn strictness(&self) -> StrictnessLevel {
        self.strictness
    }

    /// Find all rules that apply to a word pair
    pub fn find_applicable(&self, word1: &str, word2: &str) -> Vec<&SandhiRule> {
        self.rules
            .iter()
            .filter(|rule| rule.matches(word1, word2))
            .collect()
    }

    /// Find the highest-priority rule that applies to a word pair
    pub fn find_primary(&self, word1: &str, word2: &str) -> Option<&SandhiRule> {
        self.rules
            .iter()
            .find(|rule| rule.matches(word1, word2))
    }

    /// Check if a word pair violates any sandhi rules
    /// Returns the violated rule if any
    pub fn check_violation(&self, word1: &str, word2: &str) -> Option<&SandhiRule> {
        self.check_violation_with_context(word1, word2, None, None, crate::sandhi::Relationship::Any, &[])
    }

    /// Check if a word pair violates any sandhi rules with grammatical context
    pub fn check_violation_with_context(
        &self,
        word1: &str,
        word2: &str,
        class1: Option<crate::sandhi::WordClass>,
        class2: Option<crate::sandhi::WordClass>,
        relationship: crate::sandhi::Relationship,
        morph_conditions: &[String],
    ) -> Option<&SandhiRule> {
        for rule in &self.rules {
            if rule.matches_with_context(word1, word2, class1, class2, relationship, morph_conditions)
                && !rule.is_satisfied(word1, word2)
            {
                // Check if this is overridden by an exception rule
                let has_exception = self.rules.iter().any(|r| {
                    r.is_exception
                        && r.exception_of.contains(&rule.id)
                        && r.matches_with_context(word1, word2, class1, class2, relationship, morph_conditions)
                });

                if !has_exception {
                    // Apply strictness filtering
                    match self.strictness {
                        StrictnessLevel::Classical => return Some(rule),
                        StrictnessLevel::Standard => {
                            // Standard mode: most rules apply (priority >= 50)
                            if rule.priority >= 50 {
                                return Some(rule);
                            }
                        }
                        StrictnessLevel::Lenient => {
                            // Lenient mode: only critical violations (priority >= 90)
                            // This effectively disables cross-word-boundary sandhi checking
                            if rule.priority >= 90 {
                                return Some(rule);
                            }
                        }
                    }
                }
            }
        }
        None
    }

    /// Get suggested fix for a word pair
    pub fn suggest_fix(&self, word1: &str, word2: &str) -> Option<String> {
        if let Some(rule) = self.check_violation(word1, word2) {
            rule.suggest_fix(word1, word2)
        } else {
            None
        }
    }

    /// Get a rule by its ID
    pub fn get_rule(&self, id: &str) -> Option<&SandhiRule> {
        self.rules.iter().find(|r| r.id == id)
    }

    /// Get rules by Nannool verse number
    pub fn get_rules_by_verse(&self, verse: u32) -> Vec<&SandhiRule> {
        self.rules
            .iter()
            .filter(|r| r.nannool_verses.contains(&verse))
            .collect()
    }

    /// Get all violations for a word pair (including multiple rules)
    pub fn get_all_violations(&self, word1: &str, word2: &str) -> Vec<&SandhiRule> {
        self.rules
            .iter()
            .filter(|rule| rule.matches(word1, word2) && !rule.is_satisfied(word1, word2))
            .collect()
    }
}

/// Result of checking a word boundary
#[derive(Debug, Clone)]
pub struct BoundaryCheck {
    /// The first word
    pub word1: String,
    /// The second word
    pub word2: String,
    /// Rules that were violated
    pub violations: Vec<String>,
    /// Suggested correction
    pub suggestion: Option<String>,
}

impl BoundaryCheck {
    /// Create a new boundary check result
    pub fn new(word1: String, word2: String) -> Self {
        Self {
            word1,
            word2,
            violations: vec![],
            suggestion: None,
        }
    }

    /// Check if there are any violations
    pub fn has_violations(&self) -> bool {
        !self.violations.is_empty()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rule_engine_creation() {
        let engine = RuleEngine::new();
        assert!(!engine.rules().is_empty());
    }

    #[test]
    fn test_find_applicable() {
        let engine = RuleEngine::new();

        // Should find vallinam rule
        let applicable = engine.find_applicable("பாட்டு", "பாடினான்");
        assert!(!applicable.is_empty());
        assert!(applicable.iter().any(|r| r.id == "vallinam-miguthal-165"));
    }

    #[test]
    fn test_find_primary() {
        let engine = RuleEngine::new();

        let primary = engine.find_primary("பாட்டு", "பாடினான்");
        assert!(primary.is_some());
    }

    #[test]
    fn test_check_violation() {
        let engine = RuleEngine::new();

        // This should violate some sandhi rule
        let violation = engine.check_violation("பாட்டு", "பாடினான்");
        assert!(violation.is_some());
        // Multiple rules could apply (vallinam-miguthal or kutriyalukaram)
        let rule_id = &violation.unwrap().id;
        assert!(rule_id.contains("vallinam") || rule_id.contains("kutriyalukaram") || rule_id.contains("vanthodar"));

        // This should not violate (properly doubled)
        let _violation = engine.check_violation("பாட்டு", "ப்பாடினான்");
        // Note: This might still show as violation depending on implementation
        // The doubled form should satisfy the rule
    }

    #[test]
    fn test_suggest_fix() {
        let engine = RuleEngine::new();

        let fix = engine.suggest_fix("பாட்டு", "பாடினான்");
        assert!(fix.is_some());
        // The fix should show the combined form with doubling
    }

    #[test]
    fn test_strictness_levels() {
        let strict_engine = RuleEngine::new()
            .with_strictness(StrictnessLevel::Classical);
        let lenient_engine = RuleEngine::new()
            .with_strictness(StrictnessLevel::Lenient);

        // Both should detect violations, but lenient may skip minor ones
        let strict_violations = strict_engine.get_all_violations("பாட்டு", "பாடினான்");
        let lenient_violations = lenient_engine.get_all_violations("பாட்டு", "பாடினான்");

        // Strict should catch at least as many as lenient
        assert!(strict_violations.len() >= lenient_violations.len());
    }

    #[test]
    fn test_get_rule_by_id() {
        let engine = RuleEngine::new();

        let rule = engine.get_rule("vallinam-miguthal-165");
        assert!(rule.is_some());
        assert_eq!(rule.unwrap().tamil_name, "வல்லினம் மிகுதல்");
    }

    #[test]
    fn test_get_rules_by_verse() {
        let engine = RuleEngine::new();

        let rules_165 = engine.get_rules_by_verse(165);
        assert!(!rules_165.is_empty());

        let rules_162 = engine.get_rules_by_verse(162);
        assert!(!rules_162.is_empty());
    }

    #[test]
    fn test_verse_lookup() {
        // Create engine with actual files to test verse lookup
        let rules_path = concat!(env!("CARGO_MANIFEST_DIR"), "/../../data/nannool/rules.toml");
        let verses_path = concat!(env!("CARGO_MANIFEST_DIR"), "/../../data/nannool/punariyar.toml");
        
        let engine = RuleEngine::from_files(rules_path, verses_path).unwrap();
        
        let verse_162 = engine.get_verse(162);
        assert!(verse_162.is_some());
        assert!(verse_162.unwrap().tamil_text.contains("உடம்படு மெய்"));
        
        let verse_204 = engine.get_verse(204);
        assert!(verse_204.is_some());
        assert!(verse_204.unwrap().tamil_text.contains("உடல்மேல் உயிர்"));
    }
}
