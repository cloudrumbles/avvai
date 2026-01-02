//! Unicode normalization for Tamil text.
//!
//! Tamil text can have different Unicode representations for the same visual grapheme.
//! This module provides functions to normalize Tamil text to a canonical form (NFC)
//! and validate that text contains valid Tamil sequences.

use unicode_normalization::UnicodeNormalization;

use crate::letters::{is_mei_base, is_grantha_base, is_vowel_sign, PULLI};

/// Error types for Tamil text validation
#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
pub enum TamilError {
    /// A vowel sign appears without a preceding consonant base
    #[error("orphan vowel sign '{sign}' at position {position}")]
    OrphanVowelSign { sign: char, position: usize },

    /// A pulli appears without a preceding consonant base
    #[error("orphan pulli at position {position}")]
    OrphanPulli { position: usize },

    /// Multiple vowel signs or pulli in sequence
    #[error("consecutive modifiers at position {position}")]
    ConsecutiveModifiers { position: usize },

    /// Invalid character sequence
    #[error("invalid sequence at position {position}: {description}")]
    InvalidSequence { position: usize, description: String },
}

/// Normalize Tamil text to NFC (Canonical Decomposition, followed by Canonical Composition).
///
/// This ensures that text with different Unicode representations of the same
/// grapheme is normalized to a consistent form. For example, precomposed vs
/// decomposed representations of vowel signs.
///
/// # Example
/// ```
/// use tamil_unicode::normalize::normalize_tamil;
///
/// let text = "தமிழ்";
/// let normalized = normalize_tamil(text);
/// assert_eq!(normalized, "தமிழ்");
/// ```
pub fn normalize_tamil(text: &str) -> String {
    text.nfc().collect()
}

/// Normalize Tamil text and validate that it contains valid Tamil sequences.
///
/// This function:
/// 1. Applies NFC normalization
/// 2. Checks for orphan vowel signs (vowel signs without preceding consonants)
/// 3. Checks for orphan pulli (pulli without preceding consonants)
/// 4. Checks for consecutive modifiers
///
/// # Example
/// ```
/// use tamil_unicode::normalize::normalize_and_validate;
///
/// let text = "தமிழ்";
/// let result = normalize_and_validate(text);
/// assert!(result.is_ok());
/// ```
///
/// # Errors
/// Returns a `TamilError` if the text contains invalid Tamil sequences.
pub fn normalize_and_validate(text: &str) -> Result<String, TamilError> {
    let normalized: String = text.nfc().collect();
    validate_tamil_sequences(&normalized)?;
    Ok(normalized)
}

/// Validate that a string contains valid Tamil character sequences.
///
/// This checks for:
/// - Orphan vowel signs (must follow consonant base)
/// - Orphan pulli (must follow consonant base)
/// - Consecutive modifiers (vowel sign/pulli cannot follow another modifier)
fn validate_tamil_sequences(text: &str) -> Result<(), TamilError> {
    let chars: Vec<char> = text.chars().collect();

    for (i, &ch) in chars.iter().enumerate() {
        // Check for vowel signs and pulli
        if is_vowel_sign(ch) || ch == PULLI {
            // Must have a preceding character
            if i == 0 {
                if ch == PULLI {
                    return Err(TamilError::OrphanPulli { position: i });
                } else {
                    return Err(TamilError::OrphanVowelSign { sign: ch, position: i });
                }
            }

            let prev = chars[i - 1];

            // Check for consecutive modifiers
            if is_vowel_sign(prev) || prev == PULLI {
                return Err(TamilError::ConsecutiveModifiers { position: i });
            }

            // Vowel signs and pulli must follow a consonant base
            if !is_mei_base(prev) && !is_grantha_base(prev) {
                if ch == PULLI {
                    return Err(TamilError::OrphanPulli { position: i });
                } else {
                    return Err(TamilError::OrphanVowelSign { sign: ch, position: i });
                }
            }
        }
    }

    Ok(())
}

/// Check if text is already in NFC normalized form.
///
/// # Example
/// ```
/// use tamil_unicode::normalize::is_normalized;
///
/// assert!(is_normalized("தமிழ்"));
/// ```
pub fn is_normalized(text: &str) -> bool {
    unicode_normalization::is_nfc(text)
}

/// Normalize only Tamil portions of mixed text, leaving other scripts unchanged.
///
/// This is useful when processing text that contains Tamil mixed with other scripts,
/// and you want to ensure consistent normalization across the Tamil portions.
///
/// Note: Since NFC normalization is applied character-by-character and is idempotent
/// for non-Tamil text, this effectively normalizes the entire string but is semantically
/// clear about the intent.
///
/// # Example
/// ```
/// use tamil_unicode::normalize::normalize_tamil_preserving_others;
///
/// let mixed = "Hello தமிழ் World";
/// let normalized = normalize_tamil_preserving_others(mixed);
/// ```
pub fn normalize_tamil_preserving_others(text: &str) -> String {
    // NFC normalization is safe for all Unicode text, so we just apply it
    // The function name makes the intent clear
    text.nfc().collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_normalize_already_normalized() {
        let text = "தமிழ்";
        let normalized = normalize_tamil(text);
        assert_eq!(normalized, text);
    }

    #[test]
    fn test_normalize_simple_word() {
        let text = "வணக்கம்";
        let normalized = normalize_tamil(text);
        assert_eq!(normalized, "வணக்கம்");
    }

    #[test]
    fn test_is_normalized() {
        assert!(is_normalized("தமிழ்"));
        assert!(is_normalized("வணக்கம்"));
        assert!(is_normalized("அழகு"));
    }

    #[test]
    fn test_normalize_and_validate_valid() {
        let result = normalize_and_validate("தமிழ்");
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "தமிழ்");
    }

    #[test]
    fn test_normalize_and_validate_with_uyir() {
        let result = normalize_and_validate("அழகு");
        assert!(result.is_ok());
    }

    #[test]
    fn test_normalize_and_validate_with_aaytham() {
        let result = normalize_and_validate("அஃது");
        assert!(result.is_ok());
    }

    #[test]
    fn test_orphan_vowel_sign() {
        // Vowel sign at start
        let result = normalize_and_validate("ா");
        assert!(matches!(result, Err(TamilError::OrphanVowelSign { .. })));

        // Vowel sign after vowel
        let result = normalize_and_validate("அா");
        assert!(matches!(result, Err(TamilError::OrphanVowelSign { .. })));
    }

    #[test]
    fn test_orphan_pulli() {
        // Pulli at start
        let result = normalize_and_validate("்");
        assert!(matches!(result, Err(TamilError::OrphanPulli { .. })));

        // Pulli after vowel
        let result = normalize_and_validate("அ்");
        assert!(matches!(result, Err(TamilError::OrphanPulli { .. })));
    }

    #[test]
    fn test_consecutive_modifiers() {
        // Two vowel signs
        let result = normalize_and_validate("காா");
        assert!(matches!(result, Err(TamilError::ConsecutiveModifiers { .. })));

        // Vowel sign after pulli
        let result = normalize_and_validate("க்ா");
        assert!(matches!(result, Err(TamilError::ConsecutiveModifiers { .. })));
    }

    #[test]
    fn test_mixed_tamil_english() {
        let text = "Hello தமிழ் World";
        let normalized = normalize_tamil(text);
        assert_eq!(normalized, "Hello தமிழ் World");
    }

    #[test]
    fn test_normalize_preserving_others() {
        let text = "Hello தமிழ் 123";
        let normalized = normalize_tamil_preserving_others(text);
        assert_eq!(normalized, text);
    }

    #[test]
    fn test_empty_string() {
        assert_eq!(normalize_tamil(""), "");
        assert!(normalize_and_validate("").is_ok());
    }

    #[test]
    fn test_only_english() {
        let text = "Hello World";
        assert_eq!(normalize_tamil(text), text);
        assert!(normalize_and_validate(text).is_ok());
    }

    #[test]
    fn test_valid_grantha() {
        let result = normalize_and_validate("ஜெய்");
        assert!(result.is_ok());
    }

    #[test]
    fn test_complex_word() {
        // வணக்கம் - a common Tamil greeting
        let result = normalize_and_validate("வணக்கம்");
        assert!(result.is_ok());
    }

    #[test]
    fn test_numbers_and_tamil() {
        let result = normalize_and_validate("௧௨௩");
        assert!(result.is_ok());
    }
}
