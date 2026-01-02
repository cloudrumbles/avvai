//! # Nannool Rules
//!
//! This crate contains the formalized grammatical rules from Nannool (நன்னூல்),
//! a 13th-century Tamil grammar text by Pavananthi Munivar.
//!
//! The primary focus is on புணர்ச்சி (sandhi) rules from verses 151-257.

pub mod verses;
pub mod sandhi;
pub mod patterns;
pub mod engine;
pub mod loader;
mod tests_new_rules;

pub use verses::*;
pub use sandhi::*;
pub use patterns::*;
pub use engine::*;
pub use loader::*;

/// Re-export commonly used types
pub mod prelude {
    pub use crate::verses::{NannoolVerse, Chapter, Section};
    pub use crate::sandhi::{SandhiRule, SandhiCategory, Transformation};
    pub use crate::patterns::{LeftContext, RightContext};
    pub use crate::engine::RuleEngine;
    pub use crate::loader::{load_rules_from_file, load_rules_or_builtin, LoadError};
}
