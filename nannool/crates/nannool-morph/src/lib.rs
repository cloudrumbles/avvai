//! # Nannool Morphological Analyzer
//!
//! This crate provides morphological analysis for Tamil words,
//! decomposing them into root + grammatical features.
//!
//! The analysis is based on traditional Tamil grammar as codified
//! in Nannool's சொல்லதிகாரம் (verses 258-462).

pub mod analyzer;
pub mod noun;
pub mod verb;
pub mod suffix;

pub use analyzer::*;

/// Re-export common types
pub mod prelude {
    pub use crate::analyzer::{MorphAnalyzer, MorphAnalysis};
    pub use crate::noun::NounFeatures;
    pub use crate::verb::VerbFeatures;
}
