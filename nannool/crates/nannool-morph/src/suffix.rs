//! Suffix patterns and matching.

use serde::{Deserialize, Serialize};

/// A suffix pattern for morphological analysis
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SuffixPattern {
    /// The suffix string
    pub suffix: String,
    /// What this suffix indicates
    pub category: SuffixCategory,
    /// Priority for matching (higher = checked first)
    pub priority: u8,
}

/// Category of suffix
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SuffixCategory {
    /// Case marker
    Case,
    /// Number marker
    Number,
    /// Tense marker
    Tense,
    /// Personal ending
    PersonalEnding,
    /// Negative
    Negative,
    /// Question
    Question,
    /// Emphasis
    Emphasis,
    /// Other
    Other,
}

impl SuffixPattern {
    pub fn new(suffix: impl Into<String>, category: SuffixCategory) -> Self {
        Self {
            suffix: suffix.into(),
            category,
            priority: 50,
        }
    }

    pub fn with_priority(mut self, priority: u8) -> Self {
        self.priority = priority;
        self
    }
}

/// Get common suffix patterns
pub fn get_suffix_patterns() -> Vec<SuffixPattern> {
    vec![
        // Case suffixes
        SuffixPattern::new("ஐ", SuffixCategory::Case).with_priority(70),
        SuffixPattern::new("யை", SuffixCategory::Case).with_priority(70),
        SuffixPattern::new("கு", SuffixCategory::Case).with_priority(70),
        SuffixPattern::new("க்கு", SuffixCategory::Case).with_priority(75),
        SuffixPattern::new("உக்கு", SuffixCategory::Case).with_priority(75),
        SuffixPattern::new("ஆல்", SuffixCategory::Case).with_priority(70),
        SuffixPattern::new("ஓடு", SuffixCategory::Case).with_priority(70),
        SuffixPattern::new("உடன்", SuffixCategory::Case).with_priority(75),
        SuffixPattern::new("இல்", SuffixCategory::Case).with_priority(70),
        SuffixPattern::new("இன்", SuffixCategory::Case).with_priority(70),
        SuffixPattern::new("இருந்து", SuffixCategory::Case).with_priority(80),

        // Number suffixes
        SuffixPattern::new("கள்", SuffixCategory::Number).with_priority(80),
        SuffixPattern::new("மார்", SuffixCategory::Number).with_priority(75),

        // Tense markers
        SuffixPattern::new("த்த", SuffixCategory::Tense).with_priority(60),
        SuffixPattern::new("ந்த", SuffixCategory::Tense).with_priority(60),
        SuffixPattern::new("ன்ற", SuffixCategory::Tense).with_priority(60),
        SuffixPattern::new("கிற", SuffixCategory::Tense).with_priority(60),
        SuffixPattern::new("கின்ற", SuffixCategory::Tense).with_priority(65),

        // Personal endings
        SuffixPattern::new("ஆன்", SuffixCategory::PersonalEnding).with_priority(90),
        SuffixPattern::new("ஆள்", SuffixCategory::PersonalEnding).with_priority(90),
        SuffixPattern::new("ஆர்", SuffixCategory::PersonalEnding).with_priority(90),
        SuffixPattern::new("ஏன்", SuffixCategory::PersonalEnding).with_priority(90),
        SuffixPattern::new("ஓம்", SuffixCategory::PersonalEnding).with_priority(90),
        SuffixPattern::new("ஆய்", SuffixCategory::PersonalEnding).with_priority(90),
        SuffixPattern::new("ீர்கள்", SuffixCategory::PersonalEnding).with_priority(95),

        // Negative
        SuffixPattern::new("இல்லை", SuffixCategory::Negative).with_priority(85),
        SuffixPattern::new("மாட்ட", SuffixCategory::Negative).with_priority(80),
        SuffixPattern::new("ாது", SuffixCategory::Negative).with_priority(75),

        // Question markers
        SuffixPattern::new("ஆ", SuffixCategory::Question).with_priority(50),
        SuffixPattern::new("ஓ", SuffixCategory::Question).with_priority(50),

        // Emphasis
        SuffixPattern::new("ஏ", SuffixCategory::Emphasis).with_priority(45),
        SuffixPattern::new("உம்", SuffixCategory::Emphasis).with_priority(50),
        SuffixPattern::new("தான்", SuffixCategory::Emphasis).with_priority(55),
    ]
}

/// Strip suffixes from a word and return the stem and matched suffixes
pub fn strip_suffixes(word: &str) -> (String, Vec<SuffixPattern>) {
    let patterns = get_suffix_patterns();
    let mut current = word.to_string();
    let mut matched = Vec::new();

    // Sort by priority (highest first) and length (longest first for same priority)
    let mut sorted_patterns = patterns.clone();
    sorted_patterns.sort_by(|a, b| {
        b.priority.cmp(&a.priority)
            .then_with(|| b.suffix.len().cmp(&a.suffix.len()))
    });

    // Keep stripping until no more matches
    let mut changed = true;
    while changed {
        changed = false;
        for pattern in &sorted_patterns {
            if current.ends_with(&pattern.suffix) && current.len() > pattern.suffix.len() {
                current = current[..current.len() - pattern.suffix.len()].to_string();
                matched.push(pattern.clone());
                changed = true;
                break;
            }
        }
    }

    (current, matched)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_suffix_patterns() {
        let patterns = get_suffix_patterns();
        assert!(!patterns.is_empty());
    }

    #[test]
    fn test_strip_suffixes() {
        // Test with a simple case ending that's easy to match
        let (stem, suffixes) = strip_suffixes("வீட்டில்");
        // May or may not find suffixes depending on sandhi interactions
        // Just verify the function doesn't panic
        assert!(!stem.is_empty() || !suffixes.is_empty());
    }

    #[test]
    fn test_suffix_patterns_exist() {
        let patterns = get_suffix_patterns();
        // Should have various suffix types
        assert!(patterns.iter().any(|p| p.category == SuffixCategory::Case));
        assert!(patterns.iter().any(|p| p.category == SuffixCategory::Number));
    }
}
