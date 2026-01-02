//! FFI backend for Foma FST lookups

use crate::error::FstError;
use crate::foma_ffi::FomaFfiAnalyzer;

/// Backend for Foma FST lookups (FFI only)
pub struct FomaBackend {
    analyzers: Vec<FomaFfiAnalyzer>,
}

impl FomaBackend {
    /// Create a new FFI backend with the given analyzers
    pub fn new(analyzers: Vec<FomaFfiAnalyzer>) -> Self {
        Self { analyzers }
    }

    /// Analyze a word using all loaded FSTs
    pub fn analyze(&self, word: &str) -> Result<Vec<String>, FstError> {
        let mut all_results = Vec::new();
        for analyzer in &self.analyzers {
            let results = analyzer.analyze(word)?;
            all_results.extend(results);
        }
        Ok(all_results)
    }

    /// Generate surface forms from an analysis query
    pub fn generate(&self, query: &str) -> Result<Vec<String>, FstError> {
        let mut all_results = Vec::new();
        for analyzer in &self.analyzers {
            let results = analyzer.generate(query)?;
            all_results.extend(results);
        }
        Ok(all_results)
    }
}
