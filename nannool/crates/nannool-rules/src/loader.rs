//! TOML rule loader.
//!
//! This module handles loading sandhi rules from TOML files.

use std::path::Path;

use serde::Deserialize;
use thiserror::Error;

use crate::patterns::{LeftContext, RightContext};
use crate::sandhi::{SandhiCategory, SandhiRule, Transformation, WordClass};
use crate::verses::{Chapter, NannoolVerse, Section};

/// Errors that can occur when loading rules
#[derive(Debug, Error)]
pub enum LoadError {
    /// IO error reading file
    #[error("Failed to read file: {0}")]
    Io(#[from] std::io::Error),

    /// TOML parsing error
    #[error("Failed to parse TOML: {0}")]
    Toml(#[from] toml::de::Error),

    /// Invalid rule data
    #[error("Invalid rule data: {0}")]
    InvalidRule(String),

    /// Invalid verse data
    #[error("Invalid verse data: {0}")]
    InvalidVerse(String),
}

/// Container for rules loaded from TOML
#[derive(Debug, Deserialize)]
struct RulesFile {
    rules: Vec<TomlRule>,
}

/// Container for verses loaded from TOML
#[derive(Debug, Deserialize)]
struct VersesFile {
    #[serde(rename = "verse")]
    verses: Vec<TomlVerse>,
}

/// A verse as represented in TOML format
#[derive(Debug, Deserialize)]
struct TomlVerse {
    number: u32,
    section: String,
    text: String,
    #[serde(default)]
    meaning: String,
    #[serde(default)]
    commentary: Option<String>,
}

impl From<TomlVerse> for NannoolVerse {
    fn from(toml: TomlVerse) -> Self {
        let section = Section::from_tamil_name(&toml.section).unwrap_or(Section::PayiramGeneral);

        // Chapter is determined by verse number
        let chapter = if toml.number <= 55 {
            Chapter::Payiram
        } else if toml.number <= 257 {
            Chapter::Ezhuthu
        } else {
            Chapter::Sol
        };

        let mut verse = NannoolVerse::new(toml.number, chapter, section, toml.text, toml.meaning);
        verse.commentary = toml.commentary;
        verse
    }
}

/// A rule as represented in TOML format
/// This intermediate struct handles the nested TOML structure
#[derive(Debug, Deserialize)]
struct TomlRule {
    id: String,
    nannool_verses: Vec<u32>,
    tamil_name: String,
    english_name: String,
    #[serde(default)]
    relationship: crate::sandhi::Relationship,
    left_context: LeftContext,
    right_context: RightContext,
    #[serde(default)]
    left_class: Vec<WordClass>,
    #[serde(default)]
    right_class: Vec<WordClass>,
    #[serde(default)]
    conditions: Vec<String>,
    transformation: Transformation,
    category: TomlCategory,
    priority: u8,
    #[serde(default)]
    is_exception: bool,
    #[serde(default)]
    exception_of: Vec<String>,
    description: String,
}

/// Category as string in TOML
#[derive(Debug, Deserialize)]
#[serde(rename_all = "snake_case")]
enum TomlCategory {
    Iyalbu,
    Thondral,
    Thirithal,
    Keduthal,
}

impl From<TomlCategory> for SandhiCategory {
    fn from(cat: TomlCategory) -> Self {
        match cat {
            TomlCategory::Iyalbu => SandhiCategory::Iyalbu,
            TomlCategory::Thondral => SandhiCategory::Thondral,
            TomlCategory::Thirithal => SandhiCategory::Thirithal,
            TomlCategory::Keduthal => SandhiCategory::Keduthal,
        }
    }
}

impl From<TomlRule> for SandhiRule {
    fn from(rule: TomlRule) -> Self {
        SandhiRule {
            id: rule.id,
            nannool_verses: rule.nannool_verses,
            tamil_name: rule.tamil_name,
            english_name: rule.english_name,
            relationship: rule.relationship,
            left_context: rule.left_context,
            right_context: rule.right_context,
            left_class: rule.left_class,
            right_class: rule.right_class,
            conditions: rule.conditions,
            transformation: rule.transformation,
            category: rule.category.into(),
            priority: rule.priority,
            is_exception: rule.is_exception,
            exception_of: rule.exception_of,
            description: rule.description,
        }
    }
}

/// Load rules from a TOML file
///
/// # Arguments
/// * `path` - Path to the TOML file
///
/// # Returns
/// * `Ok(Vec<SandhiRule>)` - The loaded rules
/// * `Err(LoadError)` - If loading fails
///
/// # Example
/// ```ignore
/// use nannool_rules::loader::load_rules_from_file;
///
/// let rules = load_rules_from_file("data/nannool/rules.toml")?;
/// ```
pub fn load_rules_from_file<P: AsRef<Path>>(path: P) -> Result<Vec<SandhiRule>, LoadError> {
    let content = std::fs::read_to_string(path)?;
    load_rules_from_str(&content)
}

/// Load rules from a TOML string
///
/// # Arguments
/// * `content` - TOML content as a string
///
/// # Returns
/// * `Ok(Vec<SandhiRule>)` - The loaded rules
/// * `Err(LoadError)` - If parsing fails
pub fn load_rules_from_str(content: &str) -> Result<Vec<SandhiRule>, LoadError> {
    let rules_file: RulesFile = toml::from_str(content)?;
    let rules = rules_file.rules.into_iter().map(SandhiRule::from).collect();
    Ok(rules)
}

/// Try to load rules from a file, returning empty list on failure
pub fn load_rules_or_builtin<P: AsRef<Path>>(path: P) -> Vec<SandhiRule> {
    load_rules_from_file(path).unwrap_or_default()
}

/// Load Nannool verses from a TOML file
///
/// # Arguments
/// * `path` - Path to the TOML verses file
///
/// # Returns
/// * `Ok(Vec<NannoolVerse>)` - The loaded verses
/// * `Err(LoadError)` - If loading fails
pub fn load_verses_from_file<P: AsRef<Path>>(path: P) -> Result<Vec<NannoolVerse>, LoadError> {
    let content = std::fs::read_to_string(path)?;
    load_verses_from_str(&content)
}

/// Load Nannool verses from a TOML string
///
/// # Arguments
/// * `content` - TOML content as a string
///
/// # Returns
/// * `Ok(Vec<NannoolVerse>)` - The loaded verses
/// * `Err(LoadError)` - If parsing fails
pub fn load_verses_from_str(content: &str) -> Result<Vec<NannoolVerse>, LoadError> {
    let verses_file: VersesFile = toml::from_str(content)?;
    let verses = verses_file.verses.into_iter().map(NannoolVerse::from).collect();
    Ok(verses)
}

/// Try to load verses from a file, returning empty list on failure
pub fn load_verses_or_builtin<P: AsRef<Path>>(path: P) -> Vec<NannoolVerse> {
    load_verses_from_file(path).unwrap_or_default()
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE_TOML: &str = r#"
[[rules]]
id = "test-rule-1"
nannool_verses = [165]
tamil_name = "வல்லினம் மிகுதல்"
english_name = "Vallinam doubling"
category = "thondral"
priority = 50
description = "Test rule description"

[rules.left_context]
type = "any_uyir"

[rules.right_context]
type = "vallinam"

[rules.transformation]
type = "double_initial"
"#;

    #[test]
    fn test_load_rules_from_str() {
        let rules = load_rules_from_str(SAMPLE_TOML).unwrap();
        assert_eq!(rules.len(), 1);

        let rule = &rules[0];
        assert_eq!(rule.id, "test-rule-1");
        assert_eq!(rule.tamil_name, "வல்லினம் மிகுதல்");
        assert_eq!(rule.priority, 50);
        assert_eq!(rule.category, SandhiCategory::Thondral);
    }

    #[test]
    fn test_load_rules_with_insert_transformation() {
        let toml = r#"
[[rules]]
id = "buffer-ya"
nannool_verses = [162]
tamil_name = "உடம்படுமெய் - யகரம்"
english_name = "Buffer consonant - ya"
category = "thondral"
priority = 60
description = "Buffer consonant test"

[rules.left_context]
type = "specific_uyir"
letters = ["இ", "ஈ", "ஐ"]

[rules.right_context]
type = "any_uyir"

[rules.transformation]
type = "insert"
text = "ய்"
"#;

        let rules = load_rules_from_str(toml).unwrap();
        assert_eq!(rules.len(), 1);

        let rule = &rules[0];
        match &rule.left_context {
            LeftContext::SpecificUyir { letters } => {
                assert_eq!(letters.len(), 3);
                assert!(letters.contains(&'இ'));
            }
            _ => panic!("Expected SpecificUyir"),
        }
    }

    #[test]
    fn test_load_rules_with_specific_mei() {
        let toml = r#"
[[rules]]
id = "mam-test"
nannool_verses = [217]
tamil_name = "மகர இறுதி"
english_name = "Mam ending"
category = "thirithal"
priority = 55
description = "Mam test"

[rules.left_context]
type = "specific_mei"
letters = ["ம்"]

[rules.right_context]
type = "vallinam"

[rules.transformation]
type = "final_to_mellinam"
"#;

        let rules = load_rules_from_str(toml).unwrap();
        let rule = &rules[0];

        match &rule.left_context {
            LeftContext::SpecificMei { letters } => {
                assert!(letters.contains(&"ம்".to_string()));
            }
            _ => panic!("Expected SpecificMei"),
        }
    }

    #[test]
    fn test_load_invalid_toml() {
        let result = load_rules_from_str("invalid toml {{{");
        assert!(result.is_err());
    }

    #[test]
    fn test_load_rules_or_builtin_fallback() {
        // Non-existent file should return empty
        let rules = load_rules_or_builtin("/nonexistent/path/rules.toml");
        assert!(rules.is_empty());
    }

    #[test]
    fn test_load_actual_rules_file() {
        // Load the actual rules.toml from the data directory
        let rules_path = concat!(env!("CARGO_MANIFEST_DIR"), "/../../data/nannool/rules.toml");
        let rules = load_rules_from_file(rules_path).unwrap();

        // Should have 55+ rules from the expanded TOML file
        assert!(rules.len() >= 50, "Expected at least 50 rules, got {}", rules.len());

        // Verify key rules exist
        let vallinam_rule = rules.iter().find(|r| r.id == "vallinam-miguthal-165");
        assert!(vallinam_rule.is_some());

        let rule = vallinam_rule.unwrap();
        assert_eq!(rule.tamil_name, "வல்லினம் மிகுதல்");
        assert_eq!(rule.category, SandhiCategory::Thondral);
        assert!(matches!(rule.left_context, LeftContext::AnyUyir));
        assert!(matches!(rule.right_context, RightContext::Vallinam));
    }

    #[test]
    fn test_load_verses_from_str() {
        let toml = r#"
[[verse]]
number = 162
section = "உயிரீற்றுப் புணரியல்"
text = "இ ஈ ஐ வழி யவ்வும்..."
"#;
        let verses = load_verses_from_str(toml).unwrap();
        assert_eq!(verses.len(), 1);
        assert_eq!(verses[0].number, 162);
        assert_eq!(verses[0].section, Section::UyirPunarchi);
        assert_eq!(verses[0].tamil_text, "இ ஈ ஐ வழி யவ்வும்...");
    }

    #[test]
    fn test_load_actual_verses_file() {
        let verses_path = concat!(env!("CARGO_MANIFEST_DIR"), "/../../data/nannool/punariyar.toml");
        let verses = load_verses_from_file(verses_path).unwrap();

        // Should have verses 151-257 (approximately 107 verses, but some might be missing or combined)
        assert!(verses.len() > 100);

        let v162 = verses.iter().find(|v| v.number == 162).unwrap();
        assert_eq!(v162.section, Section::UyirPunarchi);
        assert!(v162.tamil_text.contains("உடம்படு மெய்"));
    }
}
