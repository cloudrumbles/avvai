//! Tamil letter constants and classifications.
//!
//! This module defines all Tamil letters categorized according to
//! traditional grammar (இலக்கணம்).
//!
//! Note: Pure consonants (mei) are multi-codepoint (base + pulli),
//! so we store them as string slices, not char arrays.

/// Tamil vowels (உயிர் எழுத்துக்கள்) - 12 letters
/// These are the pure vowel sounds.
pub const UYIR: &[char] = &[
    'அ', 'ஆ', 'இ', 'ஈ', 'உ', 'ஊ',
    'எ', 'ஏ', 'ஐ', 'ஒ', 'ஓ', 'ஔ',
];

/// Short vowels (குறில்) - 5 letters
/// These vowels have a single mātra (unit) of length.
pub const KURIL: &[char] = &['அ', 'இ', 'உ', 'எ', 'ஒ'];

/// Long vowels (நெடில்) - 7 letters
/// These vowels have two mātra (units) of length.
pub const NEDIL: &[char] = &['ஆ', 'ஈ', 'ஊ', 'ஏ', 'ஐ', 'ஓ', 'ஔ'];

/// Tamil consonants (மெய் எழுத்துக்கள்) - 18 letters
/// These are written with pulli (்) to indicate pure consonant.
/// Stored as strings since mei = base + pulli (two codepoints).
pub const MEI: &[&str] = &[
    "க்", "ங்", "ச்", "ஞ்", "ட்", "ண்",
    "த்", "ந்", "ப்", "ம்", "ய்", "ர்",
    "ல்", "வ்", "ழ்", "ள்", "ற்", "ன்",
];

/// Consonant base characters (without pulli)
/// These combine with vowel signs to form uyirmei letters.
pub const MEI_BASE: &[char] = &[
    'க', 'ங', 'ச', 'ஞ', 'ட', 'ண',
    'த', 'ந', 'ப', 'ம', 'ய', 'ர',
    'ல', 'வ', 'ழ', 'ள', 'ற', 'ன',
];

/// Hard consonants (வல்லினம்) - 6 letters (as strings with pulli)
pub const VALLINAM: &[&str] = &["க்", "ச்", "ட்", "த்", "ப்", "ற்"];

/// Vallinam base characters (without pulli)
pub const VALLINAM_BASE: &[char] = &['க', 'ச', 'ட', 'த', 'ப', 'ற'];

/// Soft/nasal consonants (மெல்லினம்) - 6 letters (as strings with pulli)
pub const MELLINAM: &[&str] = &["ங்", "ஞ்", "ண்", "ந்", "ம்", "ன்"];

/// Mellinam base characters (without pulli)
pub const MELLINAM_BASE: &[char] = &['ங', 'ஞ', 'ண', 'ந', 'ம', 'ன'];

/// Medium consonants (இடையினம்) - 6 letters (as strings with pulli)
pub const IDAIYINAM: &[&str] = &["ய்", "ர்", "ல்", "வ்", "ழ்", "ள்"];

/// Idaiyinam base characters (without pulli)
pub const IDAIYINAM_BASE: &[char] = &['ய', 'ர', 'ல', 'வ', 'ழ', 'ள'];

/// Grantha consonants (கிரந்த எழுத்துக்கள்) as strings with pulli
pub const GRANTHA: &[&str] = &["ஜ்", "ஷ்", "ஸ்", "ஹ்"];

/// Grantha base characters
pub const GRANTHA_BASE: &[char] = &['ஜ', 'ஷ', 'ஸ', 'ஹ'];

/// Aaytham (ஆய்த எழுத்து)
/// A special character unique to Tamil, sometimes called the "Tamil visarga".
pub const AAYTHAM: char = 'ஃ';

/// Tamil digit characters (௦ to ௯)
pub const DIGITS: &[char] = &['௦', '௧', '௨', '௩', '௪', '௫', '௬', '௭', '௮', '௯'];

/// Tamil number symbols
pub const NUMBER_SYMBOLS: &[char] = &[
    '௰', // 10
    '௱', // 100
    '௲', // 1000
];

/// Vowel signs (உயிர் குறியீடுகள்)
/// These combine with consonant bases to form uyirmei.
pub const VOWEL_SIGNS: &[char] = &[
    'ா',  // ஆ
    'ி',  // இ
    'ீ',  // ஈ
    'ு',  // உ
    'ூ',  // ஊ
    'ெ', // எ
    'ே', // ஏ
    'ை', // ஐ
    'ொ', // ஒ
    'ோ', // ஓ
    'ௌ', // ஔ
];

/// Pulli (virama) - marks a pure consonant (removes inherent 'a')
pub const PULLI: char = '்';

/// Mapping from vowel to corresponding vowel sign
/// Returns None for 'அ' as it has no sign (inherent vowel).
pub fn vowel_to_sign(vowel: char) -> Option<char> {
    match vowel {
        'அ' => None, // Inherent vowel, no sign needed
        'ஆ' => Some('ா'),
        'இ' => Some('ி'),
        'ஈ' => Some('ீ'),
        'உ' => Some('ு'),
        'ஊ' => Some('ூ'),
        'எ' => Some('ெ'),
        'ஏ' => Some('ே'),
        'ஐ' => Some('ை'),
        'ஒ' => Some('ொ'),
        'ஓ' => Some('ோ'),
        'ஔ' => Some('ௌ'),
        _ => None,
    }
}

/// Mapping from vowel sign to corresponding vowel
pub fn sign_to_vowel(sign: char) -> Option<char> {
    match sign {
        'ா' => Some('ஆ'),
        'ி' => Some('இ'),
        'ீ' => Some('ஈ'),
        'ு' => Some('உ'),
        'ூ' => Some('ஊ'),
        'ெ' => Some('எ'),
        'ே' => Some('ஏ'),
        'ை' => Some('ஐ'),
        'ொ' => Some('ஒ'),
        'ோ' => Some('ஓ'),
        'ௌ' => Some('ஔ'),
        _ => None,
    }
}

/// Check if a character is an uyir (vowel)
#[inline]
pub fn is_uyir(ch: char) -> bool {
    UYIR.contains(&ch)
}

/// Check if a character is a kuril (short vowel)
#[inline]
pub fn is_kuril(ch: char) -> bool {
    KURIL.contains(&ch)
}

/// Check if a character is a nedil (long vowel)
#[inline]
pub fn is_nedil(ch: char) -> bool {
    NEDIL.contains(&ch)
}

/// Check if a string is a mei (pure consonant with pulli)
#[inline]
pub fn is_mei(s: &str) -> bool {
    MEI.contains(&s) || GRANTHA.contains(&s)
}

/// Check if a character is a mei base (consonant without pulli)
#[inline]
pub fn is_mei_base(ch: char) -> bool {
    MEI_BASE.contains(&ch)
}

/// Check if a character/string is vallinam (hard consonant)
#[inline]
pub fn is_vallinam(s: &str) -> bool {
    VALLINAM.contains(&s)
}

/// Check if a character is vallinam base
#[inline]
pub fn is_vallinam_base(ch: char) -> bool {
    VALLINAM_BASE.contains(&ch)
}

/// Check if a string is mellinam (soft/nasal consonant)
#[inline]
pub fn is_mellinam(s: &str) -> bool {
    MELLINAM.contains(&s)
}

/// Check if a character is mellinam base
#[inline]
pub fn is_mellinam_base(ch: char) -> bool {
    MELLINAM_BASE.contains(&ch)
}

/// Check if a string is idaiyinam (medium consonant)
#[inline]
pub fn is_idaiyinam(s: &str) -> bool {
    IDAIYINAM.contains(&s)
}

/// Check if a character is idaiyinam base
#[inline]
pub fn is_idaiyinam_base(ch: char) -> bool {
    IDAIYINAM_BASE.contains(&ch)
}

/// Check if a character is a vowel sign
#[inline]
pub fn is_vowel_sign(ch: char) -> bool {
    VOWEL_SIGNS.contains(&ch)
}

/// Check if a character is aaytham
#[inline]
pub fn is_aaytham(ch: char) -> bool {
    ch == AAYTHAM
}

/// Check if a character is a Tamil digit
#[inline]
pub fn is_tamil_digit(ch: char) -> bool {
    DIGITS.contains(&ch)
}

/// Check if a character is grantha base
#[inline]
pub fn is_grantha_base(ch: char) -> bool {
    GRANTHA_BASE.contains(&ch)
}

/// Get the corresponding mellinam for a vallinam consonant
/// This is used in sandhi rules.
pub fn vallinam_to_mellinam(vallinam_base: char) -> Option<char> {
    match vallinam_base {
        'க' => Some('ங'),
        'ச' => Some('ஞ'),
        'ட' => Some('ண'),
        'த' => Some('ந'),
        'ப' => Some('ம'),
        'ற' => Some('ன'),
        _ => None,
    }
}

/// Get the index of a vowel in the UYIR array
pub fn uyir_index(vowel: char) -> Option<usize> {
    UYIR.iter().position(|&v| v == vowel)
}

/// Get the index of a consonant base in MEI_BASE array
pub fn mei_index(consonant: char) -> Option<usize> {
    MEI_BASE.iter().position(|&c| c == consonant)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_uyir_count() {
        assert_eq!(UYIR.len(), 12);
    }

    #[test]
    fn test_mei_count() {
        assert_eq!(MEI.len(), 18);
        assert_eq!(MEI_BASE.len(), 18);
    }

    #[test]
    fn test_kuril_nedil() {
        assert_eq!(KURIL.len(), 5);
        assert_eq!(NEDIL.len(), 7);
        assert_eq!(KURIL.len() + NEDIL.len(), UYIR.len());
    }

    #[test]
    fn test_consonant_classes() {
        assert_eq!(VALLINAM.len(), 6);
        assert_eq!(MELLINAM.len(), 6);
        assert_eq!(IDAIYINAM.len(), 6);
        assert_eq!(VALLINAM.len() + MELLINAM.len() + IDAIYINAM.len(), MEI.len());
    }

    #[test]
    fn test_is_uyir() {
        assert!(is_uyir('அ'));
        assert!(is_uyir('ஔ'));
        assert!(!is_uyir('க'));
    }

    #[test]
    fn test_is_kuril_nedil() {
        assert!(is_kuril('அ'));
        assert!(!is_kuril('ஆ'));
        assert!(is_nedil('ஆ'));
        assert!(!is_nedil('அ'));
    }

    #[test]
    fn test_vowel_sign_mapping() {
        assert_eq!(vowel_to_sign('ஆ'), Some('ா'));
        assert_eq!(vowel_to_sign('அ'), None);
        assert_eq!(sign_to_vowel('ா'), Some('ஆ'));
    }

    #[test]
    fn test_vallinam_to_mellinam() {
        assert_eq!(vallinam_to_mellinam('க'), Some('ங'));
        assert_eq!(vallinam_to_mellinam('ப'), Some('ம'));
        assert_eq!(vallinam_to_mellinam('ய'), None);
    }

    #[test]
    fn test_is_vallinam() {
        assert!(is_vallinam("க்"));
        assert!(!is_vallinam("ங்"));
    }

    #[test]
    fn test_is_vallinam_base() {
        assert!(is_vallinam_base('க'));
        assert!(!is_vallinam_base('ங'));
    }
}
