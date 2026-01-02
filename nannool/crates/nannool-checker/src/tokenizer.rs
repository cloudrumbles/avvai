//! Text tokenization for Tamil text.
//!
//! Splits Tamil text into word tokens with span information
//! for error reporting.

use serde::{Deserialize, Serialize};
use tamil_unicode::is_tamil_char;

/// A span in the source text
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct Span {
    /// Byte offset of start
    pub start: usize,
    /// Byte offset of end
    pub end: usize,
    /// Line number (1-indexed)
    pub line: u32,
    /// Column number (1-indexed)
    pub column: u32,
}

impl Span {
    /// Create a new span
    pub fn new(start: usize, end: usize, line: u32, column: u32) -> Self {
        Self { start, end, line, column }
    }

    /// Get the length in bytes
    pub fn len(&self) -> usize {
        self.end - self.start
    }

    /// Check if span is empty
    pub fn is_empty(&self) -> bool {
        self.start == self.end
    }

    /// Merge two spans into one covering both
    pub fn merge(&self, other: &Span) -> Span {
        Span {
            start: self.start.min(other.start),
            end: self.end.max(other.end),
            line: self.line.min(other.line),
            column: if self.line <= other.line { self.column } else { other.column },
        }
    }
}

/// Kind of token
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TokenKind {
    /// A Tamil word
    TamilWord,
    /// English or other script word
    OtherWord,
    /// Punctuation mark
    Punctuation,
    /// Whitespace (space, tab)
    Whitespace,
    /// Newline
    Newline,
    /// Number
    Number,
    /// Unknown/other
    Unknown,
}

/// A token in the source text
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Token {
    /// The token text
    pub text: String,
    /// Location in source
    pub span: Span,
    /// Token kind
    pub kind: TokenKind,
}

impl Token {
    /// Create a new token
    pub fn new(text: String, span: Span, kind: TokenKind) -> Self {
        Self { text, span, kind }
    }

    /// Check if this is a Tamil word
    pub fn is_tamil_word(&self) -> bool {
        self.kind == TokenKind::TamilWord
    }

    /// Check if this is whitespace or newline
    pub fn is_whitespace(&self) -> bool {
        matches!(self.kind, TokenKind::Whitespace | TokenKind::Newline)
    }
}

/// Tokenize Tamil text into tokens
pub fn tokenize(text: &str) -> Vec<Token> {
    let mut tokens = Vec::new();
    let mut current_word = String::new();
    let mut word_start = 0;
    let mut word_line = 1u32;
    let mut word_column = 1u32;

    let mut line = 1u32;
    let mut column = 1u32;
    let mut has_tamil = false;

    let chars: Vec<(usize, char)> = text.char_indices().collect();
    let mut i = 0;

    while i < chars.len() {
        let (byte_pos, ch) = chars[i];

        // Check for word boundary
        if ch.is_whitespace() || is_punctuation(ch) {
            // Emit current word if any
            if !current_word.is_empty() {
                let kind = if has_tamil {
                    TokenKind::TamilWord
                } else if current_word.chars().all(|c| c.is_ascii_digit()) {
                    TokenKind::Number
                } else {
                    TokenKind::OtherWord
                };

                tokens.push(Token::new(
                    std::mem::take(&mut current_word),
                    Span::new(word_start, byte_pos, word_line, word_column),
                    kind,
                ));
                has_tamil = false;
            }

            // Emit whitespace or punctuation
            if ch == '\n' {
                tokens.push(Token::new(
                    ch.to_string(),
                    Span::new(byte_pos, byte_pos + ch.len_utf8(), line, column),
                    TokenKind::Newline,
                ));
                line += 1;
                column = 1;
            } else if ch.is_whitespace() {
                // Collect consecutive whitespace
                let ws_start = byte_pos;
                let ws_line = line;
                let ws_column = column;
                let mut ws = ch.to_string();

                while i + 1 < chars.len() {
                    let (_, next_ch) = chars[i + 1];
                    if next_ch.is_whitespace() && next_ch != '\n' {
                        ws.push(next_ch);
                        column += 1;
                        i += 1;
                    } else {
                        break;
                    }
                }

                tokens.push(Token::new(
                    ws.clone(),
                    Span::new(ws_start, ws_start + ws.len(), ws_line, ws_column),
                    TokenKind::Whitespace,
                ));
                column += 1;
            } else {
                // Punctuation
                tokens.push(Token::new(
                    ch.to_string(),
                    Span::new(byte_pos, byte_pos + ch.len_utf8(), line, column),
                    TokenKind::Punctuation,
                ));
                column += 1;
            }
        } else {
            // Part of a word
            if current_word.is_empty() {
                word_start = byte_pos;
                word_line = line;
                word_column = column;
            }
            current_word.push(ch);
            if is_tamil_char(ch) {
                has_tamil = true;
            }
            column += 1;
        }

        i += 1;
    }

    // Emit final word if any
    if !current_word.is_empty() {
        let kind = if has_tamil {
            TokenKind::TamilWord
        } else if current_word.chars().all(|c| c.is_ascii_digit()) {
            TokenKind::Number
        } else {
            TokenKind::OtherWord
        };

        tokens.push(Token::new(
            current_word,
            Span::new(word_start, text.len(), word_line, word_column),
            kind,
        ));
    }

    tokens
}

/// Check if a character is punctuation
fn is_punctuation(ch: char) -> bool {
    matches!(ch,
        '.' | ',' | ';' | ':' | '!' | '?' |
        '"' | '\'' | '(' | ')' | '[' | ']' | '{' | '}' |
        '-' | '–' | '—' | '/' | '\\' |
        '।' | '॥' // Tamil/Devanagari punctuation
    )
}

/// Get only Tamil word tokens from text
pub fn get_tamil_words(text: &str) -> Vec<Token> {
    tokenize(text)
        .into_iter()
        .filter(|t| t.kind == TokenKind::TamilWord)
        .collect()
}

/// Get pairs of adjacent Tamil words with their intervening whitespace
pub fn get_word_pairs(text: &str) -> Vec<(Token, Token)> {
    let tokens = tokenize(text);
    let tamil_words: Vec<&Token> = tokens
        .iter()
        .filter(|t| t.kind == TokenKind::TamilWord)
        .collect();

    tamil_words
        .windows(2)
        .map(|w| (w[0].clone(), w[1].clone()))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tokenize_simple() {
        let tokens = tokenize("தமிழ் மொழி");
        assert_eq!(tokens.len(), 3); // word, space, word

        assert_eq!(tokens[0].text, "தமிழ்");
        assert_eq!(tokens[0].kind, TokenKind::TamilWord);

        assert_eq!(tokens[1].kind, TokenKind::Whitespace);

        assert_eq!(tokens[2].text, "மொழி");
        assert_eq!(tokens[2].kind, TokenKind::TamilWord);
    }

    #[test]
    fn test_tokenize_with_punctuation() {
        let tokens = tokenize("வணக்கம்!");
        assert_eq!(tokens.len(), 2);
        assert_eq!(tokens[0].text, "வணக்கம்");
        assert_eq!(tokens[1].text, "!");
        assert_eq!(tokens[1].kind, TokenKind::Punctuation);
    }

    #[test]
    fn test_tokenize_mixed() {
        let tokens = tokenize("Hello தமிழ்");
        assert_eq!(tokens.len(), 3);
        assert_eq!(tokens[0].kind, TokenKind::OtherWord);
        assert_eq!(tokens[2].kind, TokenKind::TamilWord);
    }

    #[test]
    fn test_span_positions() {
        let tokens = tokenize("அ ஆ");
        assert_eq!(tokens[0].span.line, 1);
        assert_eq!(tokens[0].span.column, 1);
    }

    #[test]
    fn test_multiline() {
        let tokens = tokenize("ஒன்று\nஇரண்டு");
        let newline = tokens.iter().find(|t| t.kind == TokenKind::Newline);
        assert!(newline.is_some());

        let second_word = tokens.iter()
            .filter(|t| t.kind == TokenKind::TamilWord)
            .nth(1);
        assert!(second_word.is_some());
        assert_eq!(second_word.unwrap().span.line, 2);
    }

    #[test]
    fn test_get_word_pairs() {
        let pairs = get_word_pairs("பாட்டு பாடினான் வந்தான்");
        assert_eq!(pairs.len(), 2);
        assert_eq!(pairs[0].0.text, "பாட்டு");
        assert_eq!(pairs[0].1.text, "பாடினான்");
    }
}
