//! Tamil grapheme cluster handling.
//!
//! A Tamil grapheme is the minimal visual unit of Tamil script.
//! This module handles splitting Tamil text into grapheme clusters
//! and classifying them.

use crate::letters::*;

/// A Tamil grapheme (visual letter unit)
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum TamilGrapheme {
    /// Pure vowel: அ, ஆ, இ, ஈ, உ, ஊ, எ, ஏ, ஐ, ஒ, ஓ, ஔ
    Uyir(char),
    /// Pure consonant (with pulli): க், ங், ச், etc.
    Mei(String),
    /// Combined consonant-vowel: க, கா, கி, கீ, etc.
    UyirMei {
        /// The consonant base (e.g., 'க')
        mei_base: char,
        /// The vowel component (e.g., 'அ' for க, 'ஆ' for கா)
        uyir: char,
    },
    /// Aaytham: ஃ
    Aaytham,
    /// Grantha consonant (Sanskrit-origin)
    Grantha {
        base: char,
        vowel: Option<char>,
    },
    /// Tamil digit
    Digit(char),
    /// Non-Tamil character (punctuation, space, other scripts)
    Other(char),
}

impl TamilGrapheme {
    /// Get the string representation of this grapheme
    pub fn as_str(&self) -> String {
        match self {
            TamilGrapheme::Uyir(ch) => ch.to_string(),
            TamilGrapheme::Mei(s) => s.clone(),
            TamilGrapheme::UyirMei { mei_base, uyir } => {
                let mut s = mei_base.to_string();
                if let Some(sign) = vowel_to_sign(*uyir) {
                    s.push(sign);
                }
                // If uyir is 'அ', no sign is needed (inherent vowel)
                s
            }
            TamilGrapheme::Aaytham => AAYTHAM.to_string(),
            TamilGrapheme::Grantha { base, vowel } => {
                let mut s = base.to_string();
                if let Some(v) = vowel {
                    if let Some(sign) = vowel_to_sign(*v) {
                        s.push(sign);
                    }
                } else {
                    s.push(PULLI);
                }
                s
            }
            TamilGrapheme::Digit(ch) => ch.to_string(),
            TamilGrapheme::Other(ch) => ch.to_string(),
        }
    }

    /// Check if this grapheme ends with a vowel sound
    pub fn ends_with_vowel(&self) -> bool {
        matches!(
            self,
            TamilGrapheme::Uyir(_)
                | TamilGrapheme::UyirMei { .. }
                | TamilGrapheme::Grantha { vowel: Some(_), .. }
        )
    }

    /// Check if this grapheme ends with a consonant
    pub fn ends_with_consonant(&self) -> bool {
        matches!(
            self,
            TamilGrapheme::Mei(_) | TamilGrapheme::Grantha { vowel: None, .. }
        )
    }

    /// Get the final vowel if this grapheme ends with one
    pub fn final_vowel(&self) -> Option<char> {
        match self {
            TamilGrapheme::Uyir(v) => Some(*v),
            TamilGrapheme::UyirMei { uyir, .. } => Some(*uyir),
            TamilGrapheme::Grantha { vowel: Some(v), .. } => Some(*v),
            _ => None,
        }
    }

    /// Get the consonant base if this is a mei or uyirmei
    pub fn consonant_base(&self) -> Option<char> {
        match self {
            TamilGrapheme::Mei(s) => s.chars().next(),
            TamilGrapheme::UyirMei { mei_base, .. } => Some(*mei_base),
            TamilGrapheme::Grantha { base, .. } => Some(*base),
            _ => None,
        }
    }

    /// Check if this is a vallinam consonant
    pub fn is_vallinam(&self) -> bool {
        self.consonant_base()
            .is_some_and(|c| VALLINAM_BASE.contains(&c))
    }

    /// Check if this is a mellinam consonant
    pub fn is_mellinam(&self) -> bool {
        self.consonant_base()
            .is_some_and(|c| MELLINAM_BASE.contains(&c))
    }

    /// Check if this is an idaiyinam consonant
    pub fn is_idaiyinam(&self) -> bool {
        self.consonant_base()
            .is_some_and(|c| IDAIYINAM_BASE.contains(&c))
    }
}

/// Split a Tamil word into grapheme clusters
///
/// # Example
/// ```
/// use tamil_unicode::grapheme::get_graphemes;
///
/// let graphemes = get_graphemes("தமிழ்");
/// assert_eq!(graphemes.len(), 3);
/// // த = த் + அ (uyirmei)
/// // மி = ம் + இ (uyirmei)
/// // ழ் = pure mei
/// ```
pub fn get_graphemes(text: &str) -> Vec<TamilGrapheme> {
    let mut graphemes = Vec::new();
    let chars: Vec<char> = text.chars().collect();
    let mut i = 0;

    while i < chars.len() {
        let ch = chars[i];

        // Check for aaytham
        if ch == AAYTHAM {
            graphemes.push(TamilGrapheme::Aaytham);
            i += 1;
            continue;
        }

        // Check for uyir (pure vowel)
        if is_uyir(ch) {
            graphemes.push(TamilGrapheme::Uyir(ch));
            i += 1;
            continue;
        }

        // Check for Tamil digit
        if is_tamil_digit(ch) {
            graphemes.push(TamilGrapheme::Digit(ch));
            i += 1;
            continue;
        }

        // Check for consonant base (mei base or grantha base)
        if is_mei_base(ch) || is_grantha_base(ch) {
            let is_grantha = is_grantha_base(ch);

            // Look ahead for vowel sign or pulli
            if i + 1 < chars.len() {
                let next = chars[i + 1];

                // Check for pulli (pure consonant)
                if next == PULLI {
                    if is_grantha {
                        graphemes.push(TamilGrapheme::Grantha {
                            base: ch,
                            vowel: None,
                        });
                    } else {
                        graphemes.push(TamilGrapheme::Mei(format!("{}{}", ch, PULLI)));
                    }
                    i += 2;
                    continue;
                }

                // Check for vowel sign
                if is_vowel_sign(next) {
                    let vowel = sign_to_vowel(next).unwrap_or('அ');
                    if is_grantha {
                        graphemes.push(TamilGrapheme::Grantha {
                            base: ch,
                            vowel: Some(vowel),
                        });
                    } else {
                        graphemes.push(TamilGrapheme::UyirMei {
                            mei_base: ch,
                            uyir: vowel,
                        });
                    }
                    i += 2;
                    continue;
                }
            }

            // Consonant base with inherent 'அ' vowel
            if is_grantha {
                graphemes.push(TamilGrapheme::Grantha {
                    base: ch,
                    vowel: Some('அ'),
                });
            } else {
                graphemes.push(TamilGrapheme::UyirMei {
                    mei_base: ch,
                    uyir: 'அ',
                });
            }
            i += 1;
            continue;
        }

        // Non-Tamil character
        graphemes.push(TamilGrapheme::Other(ch));
        i += 1;
    }

    graphemes
}

/// Get the final grapheme of a Tamil word
pub fn get_final_grapheme(word: &str) -> Option<TamilGrapheme> {
    get_graphemes(word).pop()
}

/// Get the initial grapheme of a Tamil word
pub fn get_initial_grapheme(word: &str) -> Option<TamilGrapheme> {
    get_graphemes(word).into_iter().next()
}

/// Reconstruct a string from graphemes
pub fn graphemes_to_string(graphemes: &[TamilGrapheme]) -> String {
    graphemes.iter().map(|g| g.as_str()).collect()
}

/// Count Tamil letters (graphemes) in a word
pub fn letter_count(word: &str) -> usize {
    get_graphemes(word)
        .iter()
        .filter(|g| !matches!(g, TamilGrapheme::Other(_)))
        .count()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_graphemes_simple() {
        let graphemes = get_graphemes("தமிழ்");
        assert_eq!(graphemes.len(), 3);

        // த = த் + அ
        assert!(matches!(
            &graphemes[0],
            TamilGrapheme::UyirMei {
                mei_base: 'த',
                uyir: 'அ'
            }
        ));

        // மி = ம் + இ
        assert!(matches!(
            &graphemes[1],
            TamilGrapheme::UyirMei {
                mei_base: 'ம',
                uyir: 'இ'
            }
        ));

        // ழ் = pure mei
        assert!(matches!(&graphemes[2], TamilGrapheme::Mei(s) if s == "ழ்"));
    }

    #[test]
    fn test_get_graphemes_with_long_vowel() {
        let graphemes = get_graphemes("பாடு");
        assert_eq!(graphemes.len(), 2);

        // பா = ப் + ஆ
        assert!(matches!(
            &graphemes[0],
            TamilGrapheme::UyirMei {
                mei_base: 'ப',
                uyir: 'ஆ'
            }
        ));

        // டு = ட் + உ
        assert!(matches!(
            &graphemes[1],
            TamilGrapheme::UyirMei {
                mei_base: 'ட',
                uyir: 'உ'
            }
        ));
    }

    #[test]
    fn test_get_graphemes_pure_vowels() {
        let graphemes = get_graphemes("அஇஉ");
        assert_eq!(graphemes.len(), 3);
        assert!(matches!(&graphemes[0], TamilGrapheme::Uyir('அ')));
        assert!(matches!(&graphemes[1], TamilGrapheme::Uyir('இ')));
        assert!(matches!(&graphemes[2], TamilGrapheme::Uyir('உ')));
    }

    #[test]
    fn test_get_graphemes_aaytham() {
        let graphemes = get_graphemes("அஃது");
        assert_eq!(graphemes.len(), 3);
        assert!(matches!(&graphemes[0], TamilGrapheme::Uyir('அ')));
        assert!(matches!(&graphemes[1], TamilGrapheme::Aaytham));
        assert!(matches!(
            &graphemes[2],
            TamilGrapheme::UyirMei {
                mei_base: 'த',
                uyir: 'உ'
            }
        ));
    }

    #[test]
    fn test_get_graphemes_mixed() {
        let graphemes = get_graphemes("Hello தமிழ்");
        assert_eq!(graphemes.len(), 9); // H e l l o space த மி ழ்
    }

    #[test]
    fn test_final_grapheme() {
        let final_g = get_final_grapheme("தமிழ்");
        assert!(matches!(final_g, Some(TamilGrapheme::Mei(s)) if s == "ழ்"));

        let final_g = get_final_grapheme("பாடு");
        assert!(matches!(
            final_g,
            Some(TamilGrapheme::UyirMei {
                mei_base: 'ட',
                uyir: 'உ'
            })
        ));
    }

    #[test]
    fn test_initial_grapheme() {
        let initial = get_initial_grapheme("தமிழ்");
        assert!(matches!(
            initial,
            Some(TamilGrapheme::UyirMei {
                mei_base: 'த',
                uyir: 'அ'
            })
        ));

        let initial = get_initial_grapheme("அழகு");
        assert!(matches!(initial, Some(TamilGrapheme::Uyir('அ'))));
    }

    #[test]
    fn test_graphemes_to_string() {
        let graphemes = get_graphemes("தமிழ்");
        let reconstructed = graphemes_to_string(&graphemes);
        assert_eq!(reconstructed, "தமிழ்");
    }

    #[test]
    fn test_letter_count() {
        assert_eq!(letter_count("தமிழ்"), 3);
        // வணக்கம் = வ + ண + க் + க + ம் = 5 graphemes
        assert_eq!(letter_count("வணக்கம்"), 5);
    }

    #[test]
    fn test_ends_with_vowel() {
        let graphemes = get_graphemes("பாடு");
        assert!(graphemes.last().unwrap().ends_with_vowel());

        let graphemes = get_graphemes("தமிழ்");
        assert!(!graphemes.last().unwrap().ends_with_vowel());
    }

    #[test]
    fn test_is_vallinam() {
        let graphemes = get_graphemes("கா");
        assert!(graphemes[0].is_vallinam());

        let graphemes = get_graphemes("மா");
        assert!(!graphemes[0].is_vallinam());
        assert!(graphemes[0].is_mellinam());
    }
}
