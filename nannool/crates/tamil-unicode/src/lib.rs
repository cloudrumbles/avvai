//! # Tamil Unicode
//!
//! A Rust library for handling Tamil Unicode text, including:
//! - Grapheme cluster splitting
//! - Letter classification (uyir, mei, uyirmei, vallinam, etc.)
//! - Mei-uyir transformations
//! - Unicode normalization
//!
//! Based on the Open-Tamil library's utf8.py implementation.

pub mod letters;
pub mod grapheme;
pub mod transform;
pub mod normalize;

pub use letters::*;
pub use grapheme::*;
pub use transform::*;
pub use normalize::*;

/// Tamil Unicode range constants
pub mod unicode_ranges {
    /// Start of Tamil Unicode block
    pub const TAMIL_START: u32 = 0x0B80;
    /// End of Tamil Unicode block
    pub const TAMIL_END: u32 = 0x0BFF;

    /// Vowel signs start (ா to ௌ)
    pub const VOWEL_SIGN_START: char = '\u{0BBE}';
    /// Vowel signs end
    pub const VOWEL_SIGN_END: char = '\u{0BCC}';

    /// Pulli (virama) - marks pure consonant
    pub const PULLI: char = '\u{0BCD}';

    /// Aaytham
    pub const AAYTHAM: char = '\u{0B83}';
}

/// Check if a character is in the Tamil Unicode range
#[inline]
pub fn is_tamil_char(ch: char) -> bool {
    let code = ch as u32;
    code >= unicode_ranges::TAMIL_START && code <= unicode_ranges::TAMIL_END
}

/// Check if a string contains only Tamil characters (and whitespace/punctuation)
pub fn is_tamil(text: &str) -> bool {
    text.chars().all(|ch| {
        is_tamil_char(ch) || ch.is_whitespace() || ch.is_ascii_punctuation()
    })
}

/// Check if a string contains any Tamil characters
pub fn has_tamil(text: &str) -> bool {
    text.chars().any(is_tamil_char)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_tamil_char() {
        assert!(is_tamil_char('அ'));
        assert!(is_tamil_char('க'));
        assert!(is_tamil_char('ழ'));
        assert!(!is_tamil_char('a'));
        assert!(!is_tamil_char('1'));
    }

    #[test]
    fn test_is_tamil() {
        assert!(is_tamil("தமிழ்"));
        assert!(is_tamil("வணக்கம்!"));
        assert!(!is_tamil("Hello தமிழ்"));
    }

    #[test]
    fn test_has_tamil() {
        assert!(has_tamil("தமிழ்"));
        assert!(has_tamil("Hello தமிழ்"));
        assert!(!has_tamil("Hello World"));
    }
}
