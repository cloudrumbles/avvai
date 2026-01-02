//! Heuristic-based sandhi checking without morphological analysis.
//!
//! This module provides pattern-based rules that work without an FST analyzer,
//! based on common Tamil grammar patterns from tamilvu.org and practical usage.

use tamil_unicode::grapheme::{get_graphemes, get_final_grapheme};

/// Check if a word ending indicates no vallinam doubling should occur (மிகாமை)
pub fn is_no_doubling_ending(word: &str) -> bool {
    // Check suffix patterns that block doubling
    let suffixes_no_double = [
        // Case suffixes (யொடு, யோடு for words like கத்தியோடு)
        "ொடு", "ோடு", "ஒடு", "ஓடு", "உடன்",
        // Compound verb suffixes
        "கொண்டு", "விட்டு", "போட்டு",
        // Locative + irundu
        "லிருந்து", "னிருந்து",
        // Ablative
        "னின்று", "லின்று",
        // Possessive
        "உடைய",
        // Verbal participle soft clusters
        "ண்டு", "ந்து", "ன்று", "ய்து",
        // Relative participle endings
        "ஆத", "ஆததொரு",
        "கின்ற", "கிற",
        // Other
        "ஆக", "ஆன", "உம்", "போது",
    ];

    for suffix in &suffixes_no_double {
        if word.ends_with(suffix) {
            return true;
        }
    }

    // Check for relative participle pattern: ends in அ after ற்ற, ல்ல, ட்ட, etc.
    if word.ends_with("ற்ற") || word.ends_with("ல்ல") || word.ends_with("ட்ட") ||
       word.ends_with("ன்ற") || word.ends_with("ந்த") {
        return true;
    }

    // Check for இய ending (பெரிய, சிறிய, etc.)
    if word.ends_with("இய") || word.ends_with("ிய") {
        return true;
    }

    false
}

/// Check if a word is in the no-doubling word list
pub fn is_no_doubling_word(word: &str) -> bool {
    // Demonstrative/Interrogative derived words
    const SUTTU_VINA_DERIVED: &[&str] = &[
        "அந்த", "இந்த", "எந்த",
        "அங்கு", "இங்கு", "எங்கு", "ஆங்கு", "ஈங்கு", "யாங்கு",
        "அப்படி", "இப்படி", "எப்படி",
        "அப்போது", "இப்போது", "எப்போது",
        "அவ்வகை", "இவ்வகை", "எவ்வகை",
        "அத்துணை", "இத்துணை", "எத்துணை",
        "அத்தனை", "இத்தனை", "எத்தனை",
        "அவ்வளவு", "இவ்வளவு", "எவ்வளவு",
        "அவ்வாறு", "இவ்வாறு", "எவ்வாறு",
        "அது", "இது", "எது", "யாது",
        "அவை", "இவை", "எவை",
        "அன்று", "இன்று", "என்று",
    ];

    // Numerals
    const NUMERALS: &[&str] = &[
        "ஒன்று", "இரண்டு", "மூன்று", "நான்கு", "ஐந்து",
        "ஆறு", "ஏழு", "எட்டு", "ஒன்பது", "பத்து",
        "நூறு", "ஆயிரம்", "இலக்கம்", "கோடி",
        "ஒரு", "இரு", "மூ", "நா", "ஐ", "அறு", "எழு", "எண்",
    ];

    // Optative verb forms
    const VIYANGOL: &[&str] = &[
        "கற்க", "நில்", "கவனி", "செல்", "செல்க",
        "வெல்க", "வாழ்க", "வாழி", "வாழிய", "வாழியர்",
        "வருக", "ஒழிக", "உண்க", "அருள்க", "வீழ்க",
    ];

    // Quantifiers
    const QUANTIFIERS: &[&str] = &["பல", "சில", "கல", "உள", "இல"];

    // Imperatives
    const IMPERATIVES: &[&str] = &["வா", "போ", "செல்", "நில்", "எழு", "பார்"];

    // Common particles
    const PARTICLES: &[&str] = &[
        "என்று", "என்ற", "என்னும்", "என்பது",
        "ஆனால்", "ஆகவே", "ஆதலால்", "எனவே", "எனினும்",
        "மிக", "மிகவும்", "நன்கு", "நன்றாக",
    ];

    // Possessives
    const POSSESSIVES: &[&str] = &[
        "எனது", "உனது", "அவனது", "அவளது", "நமது", "எமது", "தனது",
        "என்னுடைய", "உன்னுடைய", "அவனுடைய", "அவளுடைய",
        "நம்முடைய", "எம்முடைய", "தன்னுடைய",
    ];

    SUTTU_VINA_DERIVED.contains(&word) ||
    NUMERALS.contains(&word) ||
    VIYANGOL.contains(&word) ||
    QUANTIFIERS.contains(&word) ||
    IMPERATIVES.contains(&word) ||
    PARTICLES.contains(&word) ||
    POSSESSIVES.contains(&word)
}

/// Check if a word is a single-letter word that triggers doubling
pub fn is_one_letter_doubling(word: &str) -> bool {
    const ONE_LETTER: &[&str] = &["கை", "தீ", "தை", "பூ", "மை", "நீ"];
    ONE_LETTER.contains(&word)
}

/// Check if word1 should trigger vallinam doubling before word2
pub fn should_double_vallinam(word1: &str, word2: &str) -> Option<bool> {
    // Get the final grapheme of word1
    let final_g = get_final_grapheme(word1)?;

    // Get the initial grapheme of word2
    let graphemes2 = get_graphemes(word2);
    let initial_g = graphemes2.first()?;

    // Check if word2 starts with vallinam
    if !initial_g.is_vallinam() {
        return Some(false); // No doubling needed for non-vallinam
    }

    // Check மிகாமை conditions first (higher priority)

    // 1. Check if words are identical (இரட்டைக்கிளவி)
    if word1 == word2 {
        return Some(false);
    }

    // 2. Check if word1 is in no-doubling word list
    if is_no_doubling_word(word1) {
        return Some(false);
    }

    // 3. Check suffix patterns
    if is_no_doubling_ending(word1) {
        return Some(false);
    }

    // 4. Check if word1 ends with consonant (no doubling for mei ending)
    if final_g.ends_with_consonant() {
        return Some(false);
    }

    // Now check மிகும் conditions

    // 5. Single letter words trigger doubling
    if is_one_letter_doubling(word1) {
        return Some(true);
    }

    // 6. Word ending in vowel before vallinam - generally doubles
    if final_g.ends_with_vowel() {
        // Check for kutriyalukaram (special u ending)
        if let Some(vowel) = final_g.final_vowel() {
            if vowel == 'உ' {
                // Kutriyalukaram handling
                let graphemes1 = get_graphemes(word1);
                if graphemes1.len() >= 2 {
                    let prev = &graphemes1[graphemes1.len() - 2];
                    // Vanthodar (hard cluster) - doubles
                    if prev.is_vallinam() && prev.ends_with_consonant() {
                        return Some(true);
                    }
                    // Mellinthodar (soft cluster) - no doubling
                    if prev.is_mellinam() && prev.ends_with_consonant() {
                        return Some(false);
                    }
                    // Idaithodar (medial cluster) - no doubling
                    if prev.is_idaiyinam() && prev.ends_with_consonant() {
                        return Some(false);
                    }
                }
            }
        }

        // Default: vowel ending before vallinam = double
        return Some(true);
    }

    None // Uncertain
}

/// Result of heuristic sandhi check
#[derive(Debug, Clone)]
pub struct HeuristicResult {
    /// Whether this is a violation
    pub is_violation: bool,
    /// The rule that was triggered
    pub rule_name: String,
    /// Tamil name of the rule
    pub tamil_name: String,
    /// Suggested correction
    pub suggestion: Option<String>,
    /// Confidence level (0.0 - 1.0)
    pub confidence: f32,
}

/// Check a word pair using heuristics
pub fn check_pair_heuristic(word1: &str, word2: &str) -> Option<HeuristicResult> {
    match should_double_vallinam(word1, word2) {
        Some(true) => {
            // Should double - check if it's already doubled
            let graphemes2 = get_graphemes(word2);
            if graphemes2.len() >= 2 {
                let first = &graphemes2[0];
                let second = &graphemes2[1];

                // Already doubled?
                if first.ends_with_consonant() {
                    if let (Some(b1), Some(b2)) = (first.consonant_base(), second.consonant_base()) {
                        if b1 == b2 {
                            return None; // Already correct
                        }
                    }
                }
            }

            // Not doubled - this is a violation
            let initial = graphemes2.first()?;
            let doubled = if let Some(base) = initial.consonant_base() {
                format!("{}்{}", base, word2)
            } else {
                return None;
            };

            Some(HeuristicResult {
                is_violation: true,
                rule_name: "vallinam-miguthal".to_string(),
                tamil_name: "வல்லினம் மிகுதல்".to_string(),
                suggestion: Some(format!("{}{}", word1, doubled)),
                confidence: 0.8,
            })
        }
        Some(false) => None, // No doubling needed, no violation
        None => None, // Uncertain
    }
}

/// Detect suffix-based word class (without morphological analysis)
pub fn detect_word_class_heuristic(word: &str) -> Option<&'static str> {
    // Verbal participle endings
    if word.ends_with("து") || word.ends_with("று") || word.ends_with("டு") {
        let graphemes = get_graphemes(word);
        if graphemes.len() >= 2 {
            let prev = &graphemes[graphemes.len() - 2];
            if prev.ends_with_consonant() {
                return Some("verbal_participle");
            }
        }
    }

    // Relative participle endings
    if word.ends_with("த") || word.ends_with("ற") || word.ends_with("ட") {
        return Some("relative_participle");
    }
    if word.ends_with("கின்ற") || word.ends_with("கிற") {
        return Some("relative_participle");
    }

    // Infinitive
    if word.ends_with("அ") || word.ends_with("ய") {
        // Could be infinitive or adjective
        return Some("infinitive_or_adjective");
    }

    // Noun endings (common case markers)
    if word.ends_with("ஐ") || word.ends_with("கு") || word.ends_with("ஆல்") ||
       word.ends_with("ஒடு") || word.ends_with("இல்") || word.ends_with("அது") {
        return Some("noun_with_case");
    }

    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_no_doubling_words() {
        assert!(is_no_doubling_word("ஐந்து"));
        assert!(is_no_doubling_word("அந்த"));
        assert!(is_no_doubling_word("வாழ்க"));
        assert!(is_no_doubling_word("என்று"));
        assert!(!is_no_doubling_word("பாட்டு"));
    }

    #[test]
    fn test_no_doubling_endings() {
        assert!(is_no_doubling_ending("கத்தியோடு"));
        assert!(is_no_doubling_ending("வீட்டிலிருந்து"));
        assert!(is_no_doubling_ending("கண்டு")); // ண்டு ending
        assert!(is_no_doubling_ending("செய்து")); // ய்து ending
        assert!(is_no_doubling_ending("கற்ற")); // ற்ற ending
        assert!(!is_no_doubling_ending("பாட்டு"));
    }

    #[test]
    fn test_should_double() {
        // Should double: வன்தொடர் குற்றியலுகரம்
        assert_eq!(should_double_vallinam("பாட்டு", "பாடினான்"), Some(true));

        // Should not double: numerals
        assert_eq!(should_double_vallinam("ஐந்து", "சிறுவர்கள்"), Some(false));

        // Should not double: மென்தொடர் குற்றியலுகரம்
        assert_eq!(should_double_vallinam("கண்டு", "பேசினார்"), Some(false));

        // Should not double: இரட்டைக்கிளவி
        assert_eq!(should_double_vallinam("கல", "கல"), Some(false));
    }

    #[test]
    fn test_check_pair_violation() {
        let result = check_pair_heuristic("பாட்டு", "பாடினான்");
        assert!(result.is_some());
        let r = result.unwrap();
        assert!(r.is_violation);
        assert!(r.suggestion.is_some());
    }

    #[test]
    fn test_check_pair_no_violation() {
        // Already correct (hypothetically)
        let result = check_pair_heuristic("ஐந்து", "சிறுவர்கள்");
        assert!(result.is_none()); // No violation expected
    }
}
