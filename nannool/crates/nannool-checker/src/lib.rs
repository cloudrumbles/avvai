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

pub mod tokenizer;
pub mod checker;
pub mod diagnostic;

pub use tokenizer::*;
pub use checker::*;
pub use diagnostic::*;

/// Re-export common types
pub mod prelude {
    pub use crate::tokenizer::{Token, Span, TokenKind};
    pub use crate::checker::SandhiChecker;
    pub use crate::diagnostic::{Diagnostic, DiagnosticLevel, Suggestion};
}
