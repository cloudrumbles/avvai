//! # Nannool Checker
//!
//! The main grammar checking logic that validates Tamil text
//! against Nannool sandhi rules.
//!
//! This crate provides:
//! - Text tokenization
//! - Sandhi violation detection
//! - Diagnostic reporting (rust-style errors)
//! - Suggestions for corrections
//! - Heuristic-based checking (without morphological analyzer)

pub mod tokenizer;
pub mod checker;
pub mod diagnostic;
pub mod heuristics;
pub mod sdp;
pub mod speller;
pub mod syntax;

pub use tokenizer::*;
pub use checker::*;
pub use diagnostic::*;
pub use heuristics::*;
pub use sdp::*;
pub use speller::*;
pub use syntax::*;

/// Re-export common types
pub mod prelude {
    pub use crate::tokenizer::{Token, Span, TokenKind};
    pub use crate::checker::SandhiChecker;
    pub use crate::diagnostic::{Diagnostic, DiagnosticLevel, Suggestion};
    pub use crate::heuristics::{
        check_pair_heuristic, is_no_doubling_word, is_no_doubling_ending,
        should_double_vallinam, HeuristicResult,
    };
    pub use crate::syntax::{SyntaxChecker, AgreementFeatures, AgreementMismatch};
}
