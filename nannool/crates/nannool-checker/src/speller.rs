//! Spelling checker for Tamil words.
//!
//! Uses dictionary-based validation to determine if a word is valid,
//! and provides spell suggestions using grapheme-based Levenshtein distance.

use std::collections::{HashSet, HashMap};
use nannool_morph::bundled::{BUNDLED_LEXICONS, BUNDLED_WORD_FREQUENCIES};
use tamil_unicode::grapheme::get_graphemes;
use crate::tokenizer::Token;
use crate::diagnostic::{Diagnostic, DiagnosticLevel, Suggestion};

/// Maximum edit distance for suggestions
const MAX_EDIT_DISTANCE: usize = 3;
/// Maximum number of suggestions to return
const MAX_SUGGESTIONS: usize = 5;

/// Checker for word-level spelling errors.
/// Uses dictionary-only validation (no FST guessers) for accurate error detection.
pub struct SpellingChecker {
    /// Dictionary of known valid words (200k+ words)
    dictionary: HashSet<String>,
    /// Word frequency data for ranking suggestions (word -> probability)
    word_freq: HashMap<String, f64>,
}

impl SpellingChecker {
    /// Create a new spelling checker with bundled dictionary
    pub fn new() -> Self {
        let (dictionary, word_freq) = Self::load_dictionary_and_frequencies();
        Self { dictionary, word_freq }
    }

    /// Load dictionary and word frequencies from bundled data
    /// Uses the word frequency file as the primary dictionary source (200k words)
    /// Also includes lexicon words that might not have frequency data
    fn load_dictionary_and_frequencies() -> (HashSet<String>, HashMap<String, f64>) {
        let mut dict = HashSet::new();
        let mut freq = HashMap::new();

        // Load word frequencies - these become our primary dictionary
        if let Ok(content) = std::str::from_utf8(BUNDLED_WORD_FREQUENCIES) {
            for line in content.lines() {
                if line.starts_with('#') {
                    continue;
                }
                // Format: word<TAB>probability
                if let Some((word, prob_str)) = line.split_once('\t') {
                    if let Ok(prob) = prob_str.parse::<f64>() {
                        dict.insert(word.to_string());
                        freq.insert(word.to_string(), prob);
                    }
                }
            }
        }

        // Also add lexicon words (in case they're not in frequency data)
        for (_name, bytes) in BUNDLED_LEXICONS {
            if let Ok(content) = std::str::from_utf8(bytes) {
                for line in content.lines() {
                    let word = line.trim();
                    if !word.is_empty() && !word.starts_with('#') {
                        dict.insert(word.to_string());
                    }
                }
            }
        }

        (dict, freq)
    }

    /// Check if a word is in the dictionary (valid spelling)
    pub fn is_valid(&self, word: &str) -> bool {
        self.dictionary.contains(word)
    }

    /// Check if a word is spelled correctly using dictionary lookup
    pub fn check_token(&self, token: &Token, _source: &str) -> Option<Diagnostic> {
        // Dictionary-only validation: word must be in our 200k+ word dictionary
        if self.dictionary.contains(&token.text) {
            return None; // Word is valid
        }

        // Word not in dictionary - generate suggestions
        let suggestions = self.suggest(&token.text);
        let suggestion = if !suggestions.is_empty() {
            Some(Suggestion {
                replacement: suggestions[0].clone(),
                description: format!("பரிந்துரைகள்: {}", suggestions.join(", ")),
            })
        } else {
            None
        };

        Some(Diagnostic {
            level: DiagnosticLevel::Error,
            message: format!("எழுத்துப் பிழை: '{}' அறியப்படாத சொல்", token.text),
            english_message: Some(format!("Spelling error: '{}' is an unknown word", token.text)),
            span: token.span,
            source_text: token.text.clone(),
            rule_id: "spelling-error".to_string(),
            nannool_verses: vec![],
            suggestion,
            notes: vec!["இந்தச் சொல் அகராதியில் காணப்படவில்லை".to_string()],
        })
    }

    /// Generate spelling suggestions for a misspelled word
    pub fn suggest(&self, word: &str) -> Vec<String> {
        let word_graphemes = get_graphemes(word);
        let word_len = word_graphemes.len();

        // Collect candidates with their scores (lower is better)
        // Score components:
        // - distance * 1_000_000 (primary factor)
        // - mayangoli bonus: -500_000 for confusable letter swaps
        // - frequency bonus: -log(prob) scaled (common words get lower scores)
        let mut candidates: Vec<(String, i64)> = Vec::new();

        for dict_word in &self.dictionary {
            let dict_graphemes = get_graphemes(dict_word);
            let dict_len = dict_graphemes.len();

            // Quick length filter - words too different in length can't be close
            if dict_len.abs_diff(word_len) > MAX_EDIT_DISTANCE {
                continue;
            }

            let distance = grapheme_levenshtein(&word_graphemes, &dict_graphemes);

            if distance > 0 && distance <= MAX_EDIT_DISTANCE {
                // Check if this is a mayangoli (confusable letter) swap
                let is_mayangoli = is_mayangoli_swap(&word_graphemes, &dict_graphemes);

                // Base score from distance (lower distance = much better)
                let mut score: i64 = (distance as i64) * 1_000_000;

                // Mayangoli bonus: heavily prioritize confusable letter swaps
                if is_mayangoli {
                    score -= 500_000;
                }

                // Frequency bonus: common words get lower scores
                // Probability ranges from ~0.007 (very common) to ~0.0000001 (rare)
                // Use -log(prob) scaled: higher prob = lower -log = lower score
                if let Some(&prob) = self.word_freq.get(dict_word) {
                    // -log10(0.007) ≈ 2.15, -log10(0.0000001) ≈ 7
                    // Scale to give ~200k points difference between common and rare
                    let freq_score = (-prob.log10() * 50_000.0) as i64;
                    score += freq_score;
                } else {
                    // Unknown frequency = assume rare (high penalty)
                    score += 400_000;
                }

                candidates.push((dict_word.clone(), score));
            }
        }

        // Sort by score (lower is better)
        candidates.sort_by(|a, b| a.1.cmp(&b.1));

        // Return top suggestions
        candidates
            .into_iter()
            .take(MAX_SUGGESTIONS)
            .map(|(word, _)| word)
            .collect()
    }
}

/// Check if two grapheme sequences differ only by mayangoli (confusable) letter swaps
/// Confusable pairs in Tamil:
/// - ன/ண (dental/retroflex n)
/// - ல/ள (dental/retroflex l)
/// - ர/ற (alveolar/retroflex r)
/// - ழ/ள (retroflex zh/l)
/// - ந/ன (initial/final n)
fn is_mayangoli_swap(
    a: &[tamil_unicode::grapheme::TamilGrapheme],
    b: &[tamil_unicode::grapheme::TamilGrapheme],
) -> bool {
    if a.len() != b.len() {
        return false;
    }

    // Confusable consonant bases (Unicode codepoints)
    const CONFUSABLES: &[(char, char)] = &[
        ('ன', 'ண'), // dental n / retroflex N
        ('ல', 'ள'), // dental l / retroflex L
        ('ர', 'ற'), // alveolar r / retroflex R
        ('ழ', 'ள'), // retroflex zh / retroflex L
        ('ழ', 'ல'), // retroflex zh / dental l
        ('ந', 'ன'), // initial n / final n
        ('ந', 'ண'), // initial n / retroflex N
    ];

    fn are_confusable(c1: char, c2: char) -> bool {
        // Only return true for DIFFERENT consonants that are confusable pairs
        // Same consonant with different vowel is NOT a confusable swap
        if c1 == c2 { return false; }
        CONFUSABLES.iter().any(|&(a, b)|
            (c1 == a && c2 == b) || (c1 == b && c2 == a)
        )
    }

    let mut diff_count = 0;
    let mut all_diffs_confusable = true;

    for (ga, gb) in a.iter().zip(b.iter()) {
        if ga != gb {
            diff_count += 1;

            // Check if the difference is a confusable consonant swap
            // Use the consonant_base() method from TamilGrapheme
            match (ga.consonant_base(), gb.consonant_base()) {
                (Some(ca), Some(cb)) => {
                    if !are_confusable(ca, cb) {
                        all_diffs_confusable = false;
                    }
                }
                _ => {
                    all_diffs_confusable = false;
                }
            }
        }
    }

    // Return true if there's at least one difference and all differences are confusable
    diff_count > 0 && all_diffs_confusable
}

/// Compute Levenshtein edit distance at the grapheme level
///
/// This operates on Tamil graphemes rather than Unicode codepoints,
/// which gives more linguistically meaningful edit distances for Tamil text.
fn grapheme_levenshtein(
    a: &[tamil_unicode::grapheme::TamilGrapheme],
    b: &[tamil_unicode::grapheme::TamilGrapheme],
) -> usize {
    let m = a.len();
    let n = b.len();

    // Handle edge cases
    if m == 0 {
        return n;
    }
    if n == 0 {
        return m;
    }

    // Use two-row optimization for space efficiency
    let mut prev_row: Vec<usize> = (0..=n).collect();
    let mut curr_row: Vec<usize> = vec![0; n + 1];

    for i in 1..=m {
        curr_row[0] = i;

        for j in 1..=n {
            let cost = if a[i - 1] == b[j - 1] { 0 } else { 1 };

            curr_row[j] = (prev_row[j] + 1)         // deletion
                .min(curr_row[j - 1] + 1)           // insertion
                .min(prev_row[j - 1] + cost);       // substitution
        }

        std::mem::swap(&mut prev_row, &mut curr_row);
    }

    prev_row[n]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_grapheme_levenshtein_identical() {
        let a = get_graphemes("தமிழ்");
        let b = get_graphemes("தமிழ்");
        assert_eq!(grapheme_levenshtein(&a, &b), 0);
    }

    #[test]
    fn test_grapheme_levenshtein_one_char_diff() {
        // தமிழ் vs தமில் - one grapheme different (ழ் -> ல்)
        let a = get_graphemes("தமிழ்");
        let b = get_graphemes("தமில்");
        assert_eq!(grapheme_levenshtein(&a, &b), 1);
    }

    #[test]
    fn test_grapheme_levenshtein_deletion() {
        // ஆடு vs ஆ - one grapheme deleted
        let a = get_graphemes("ஆடு");
        let b = get_graphemes("ஆ");
        assert_eq!(grapheme_levenshtein(&a, &b), 1);
    }

    #[test]
    fn test_grapheme_levenshtein_insertion() {
        // நாய் vs நாய்கள் - insertion
        let a = get_graphemes("நாய்");
        let b = get_graphemes("நாய்கள்");
        assert_eq!(grapheme_levenshtein(&a, &b), 2);
    }

    #[test]
    fn test_grapheme_levenshtein_empty() {
        let empty: Vec<tamil_unicode::grapheme::TamilGrapheme> = vec![];
        let a = get_graphemes("புலி");
        assert_eq!(grapheme_levenshtein(&empty, &a), 2);
        assert_eq!(grapheme_levenshtein(&a, &empty), 2);
    }
}
