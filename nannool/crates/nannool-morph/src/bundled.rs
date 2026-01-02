use std::path::{Path, PathBuf};
use std::fs;
use std::sync::OnceLock;
use tempfile::TempDir;

/// Bundled FST models
pub const BUNDLED_FSTS: &[(&str, &[u8])] = &[
    // Core analyzers
    ("adj.fst", include_bytes!("../../../data/thamizhi-morph/FST-Models/adj.fst")),
    ("adv.fst", include_bytes!("../../../data/thamizhi-morph/FST-Models/adv.fst")),
    ("noun.fst", include_bytes!("../../../data/thamizhi-morph/FST-Models/noun.fst")),
    ("pronoun.fst", include_bytes!("../../../data/thamizhi-morph/FST-Models/pronoun.fst")),
    ("verb-c-rest.fst", include_bytes!("../../../data/thamizhi-morph/FST-Models/verb-c-rest.fst")),
    ("verb-c3.fst", include_bytes!("../../../data/thamizhi-morph/FST-Models/verb-c3.fst")),
    ("verb-c4.fst", include_bytes!("../../../data/thamizhi-morph/FST-Models/verb-c4.fst")),
    ("verb-c11.fst", include_bytes!("../../../data/thamizhi-morph/FST-Models/verb-c11.fst")),
    ("verb-c12.fst", include_bytes!("../../../data/thamizhi-morph/FST-Models/verb-c12.fst")),
    ("verb-c62.fst", include_bytes!("../../../data/thamizhi-morph/FST-Models/verb-c62.fst")),
    ("part.fst", include_bytes!("../../../data/thamizhi-morph/FST-Models/part.fst")),
    // Guessers for OOV handling
    ("adj-guess.fst", include_bytes!("../../../data/thamizhi-morph/FST-Models/adj-guess.fst")),
    ("adv-guess.fst", include_bytes!("../../../data/thamizhi-morph/FST-Models/adv-guess.fst")),
    ("noun-guess.fst", include_bytes!("../../../data/thamizhi-morph/FST-Models/noun-guess.fst")),
    ("verb-guess.fst", include_bytes!("../../../data/thamizhi-morph/FST-Models/verb-guess.fst")),
];

/// Bundled lexicon files
pub const BUNDLED_LEXICONS: &[(&str, &[u8])] = &[
    ("Nouns-animals", include_bytes!("../../../data/thamizhi-morph/Lexicons/Nouns-animals")),
    ("Nouns-birds", include_bytes!("../../../data/thamizhi-morph/Lexicons/Nouns-birds")),
    ("Nouns-directions", include_bytes!("../../../data/thamizhi-morph/Lexicons/Nouns-directions")),
    ("Nouns-flowers", include_bytes!("../../../data/thamizhi-morph/Lexicons/Nouns-flowers")),
    ("Nouns-trees", include_bytes!("../../../data/thamizhi-morph/Lexicons/Nouns-trees")),
    ("Nouns-AUKBC", include_bytes!("../../../data/thamizhi-morph/Lexicons/Nouns-AUKBC")),
    ("Nouns-Propernouns", include_bytes!("../../../data/thamizhi-morph/Lexicons/Nouns-Propernouns")),
    // TamilVU dictionary - 63k common Tamil words
    ("tamilvu-dictionary", include_bytes!("../../../data/thamizhi-morph/Lexicons/tamilvu-dictionary")),
];

/// Word frequency data for spell suggestion ranking (top 200k words)
pub const BUNDLED_WORD_FREQUENCIES: &[u8] = include_bytes!("../../../data/thamizhi-morph/Lexicons/word-frequencies");

static BUNDLED_DIR: OnceLock<TempDir> = OnceLock::new();

/// Ensure bundled models are extracted to a temporary directory and return the path
pub fn get_bundled_dir() -> &'static Path {
    let temp_dir = BUNDLED_DIR.get_or_init(|| {
        let dir = TempDir::new().expect("Failed to create temp dir for bundled models");
        
        // Extract FSTs
        let fst_dir = dir.path().join("FST-Models");
        fs::create_dir_all(&fst_dir).expect("Failed to create FST-Models temp dir");
        for (name, bytes) in BUNDLED_FSTS {
            fs::write(fst_dir.join(name), bytes).expect("Failed to extract bundled FST");
        }

        // Extract Lexicons
        let lex_dir = dir.path().join("Lexicons");
        fs::create_dir_all(&lex_dir).expect("Failed to create Lexicons temp dir");
        for (name, bytes) in BUNDLED_LEXICONS {
            fs::write(lex_dir.join(name), bytes).expect("Failed to extract bundled lexicon");
        }

        dir
    });

    temp_dir.path()
}

/// Get the path to a bundled FST file
pub fn get_bundled_fst_path(name: &str) -> PathBuf {
    get_bundled_dir().join("FST-Models").join(name)
}

/// Get the path to the bundled Lexicons directory
pub fn get_bundled_lexicons_dir() -> PathBuf {
    get_bundled_dir().join("Lexicons")
}
