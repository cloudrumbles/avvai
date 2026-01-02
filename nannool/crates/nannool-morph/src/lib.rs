//! # Nannool Morphological Analyzer
//!
//! This crate provides morphological analysis for Tamil words using
//! ThamizhiMorph's FST models via the `flookup` subprocess.
//!
//! The analysis is based on traditional Tamil grammar as codified
//! in Nannool's சொல்லதிகாரம் (verses 258-462).
//!
//! ## Example
//!
//! ```rust,ignore
//! use nannool_morph::MorphAnalyzer;
//!
//! let analyzer = MorphAnalyzer::new("/path/to/FST-Models")?;
//! let results = analyzer.analyze("வந்தான்")?;
//! for analysis in results {
//!     println!("{}: {:?}", analysis.lemma, analysis.features);
//! }
//! ```
//!
//! ## Requirements
//!
//! - `flookup` binary from Foma toolkit must be installed
//! - ThamizhiMorph FST models (download from github.com/sarves/thamizhi-morph)

pub mod analyzer;
pub mod backend;
pub mod bundled;
pub mod error;
pub mod ffi;
pub mod foma_ffi;
pub mod fst_wrapper;
pub mod lexicon;

pub use analyzer::*;
pub use error::{FstError, MorphError, MorphResult};
pub use fst_wrapper::FomaAnalyzer;

/// Type alias for the main analyzer
pub type MorphAnalyzer = FomaAnalyzer;

/// Re-export common types
pub mod prelude {
    pub use crate::analyzer::{
        Case, Feature, Gender, Honorific, Mood, MorphAnalysis, Number, PartOfSpeech, Person, Tense,
    };
    pub use crate::error::{FstError, MorphError, MorphResult};
    pub use crate::fst_wrapper::FomaAnalyzer;
    pub use crate::MorphAnalyzer;
}
