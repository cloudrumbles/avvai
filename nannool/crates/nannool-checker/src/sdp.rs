//! Shallow Dependency Parser for Tamil.
//!
//! Detects relationships between words (அல்வழி vs வேற்றுமை)
//! to provide context for sandhi rules.

use nannool_morph::{MorphAnalysis, prelude::Case};
use nannool_rules::Relationship;

/// Parser for detecting linguistic relationships between word pairs.
pub struct ShallowDependencyParser;

impl ShallowDependencyParser {
    /// Detect the relationship between two words based on their morphological analysis.
    pub fn detect_relationship(
        _word1: &str,
        analysis1: Option<&MorphAnalysis>,
        _word2: &str,
        _analysis2: Option<&MorphAnalysis>,
    ) -> Relationship {
        // If we don't have analysis, we can't be sure, default to Any or Alvazhi
        let analysis1 = match analysis1 {
            Some(a) => a,
            None => return Relationship::Any,
        };

        // If word1 is a noun with a case marker (other than Nominative), it's Vetrumai
        if let Some(case) = analysis1.get_case() {
            if case != Case::Nominative {
                return Relationship::Vetrumai;
            }
        }

        // Default to Alvazhi for now (subject-verb, adjective-noun, etc.)
        Relationship::Alvazhi
    }
}
