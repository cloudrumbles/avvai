use std::collections::{HashMap, HashSet};
use std::path::Path;
use std::fs;

/// A simple semantic lexicon for tagging words
#[derive(Debug, Clone, Default)]
pub struct SemanticLexicon {
    /// Map of category name to set of lemmas
    categories: HashMap<String, HashSet<String>>,
}

impl SemanticLexicon {
    pub fn new() -> Self {
        Self::default()
    }

    /// Load a category from a newline-separated file
    pub fn load_category<P: AsRef<Path>>(&mut self, category: &str, path: P) -> std::io::Result<()> {
        if !path.as_ref().exists() {
            return Ok(()); // Gracefully ignore missing files
        }

        let content = fs::read_to_string(path)?;
        let mut set = HashSet::new();
        for line in content.lines() {
            let word = line.trim();
            if !word.is_empty() {
                set.insert(word.to_string());
            }
        }
        self.categories.insert(category.to_string(), set);
        Ok(())
    }

    /// Get tags for a given lemma
    pub fn get_tags(&self, lemma: &str) -> Vec<String> {
        let mut tags = Vec::new();
        for (category, words) in &self.categories {
            if words.contains(lemma) {
                tags.push(category.clone());
            }
        }
        tags
    }
}
