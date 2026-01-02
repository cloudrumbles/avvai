//! # Nannool CLI
//!
//! Command-line interface for the Nannool Tamil grammar checker.
//!
//! Usage:
//!   nannool check <file>       Check a file for sandhi errors
//!   nannool explain <rule-id>  Explain a rule
//!   nannool analyze <word>     Analyze a word morphologically
//!   nannool benchmark <file>   Run spelling benchmark

use clap::{Parser, Subcommand, ValueEnum};
use miette::{IntoDiagnostic, Result, WrapErr};
use nannool_checker::{SandhiChecker, SpellingChecker, format_diagnostics, to_sarif_report};
use nannool_rules::StrictnessLevel;
use serde::Deserialize;
use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;

/// Get the rule engine, using bundled rules or loading from environment/file if specified
fn get_rule_engine() -> nannool_rules::RuleEngine {
    if let Ok(rules_path) = std::env::var("NANNOOL_RULES_PATH") {
        return nannool_rules::RuleEngine::from_file_or_builtin(rules_path);
    }
    
    nannool_rules::RuleEngine::new()
}

#[derive(Parser)]
#[command(name = "nannool")]
#[command(author = "Avvai Project")]
#[command(version)]
#[command(about = "Tamil grammar checker based on Nannool (நன்னூல்)", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Check a file or text for sandhi errors
    Check {
        /// Input file to check (or use --text for inline text)
        #[arg(required_unless_present = "text")]
        input: Option<PathBuf>,

        /// Inline text to check
        #[arg(short, long)]
        text: Option<String>,

        /// Output format
        #[arg(short, long, value_enum, default_value = "human")]
        format: OutputFormat,

        /// Strictness level
        #[arg(short, long, value_enum, default_value = "standard")]
        level: Strictness,

        /// Disable colored output
        #[arg(long)]
        no_color: bool,
    },

    /// Explain a sandhi rule
    Explain {
        /// Rule ID (e.g., vallinam-miguthal-165)
        rule_id: String,
    },

    /// Analyze a word morphologically
    Analyze {
        /// The Tamil word to analyze
        word: String,

        /// Output format
        #[arg(short, long, value_enum, default_value = "human")]
        format: OutputFormat,
    },

    /// Check a word pair for sandhi
    Pair {
        /// First word
        word1: String,
        /// Second word
        word2: String,
    },

    /// List all available rules
    Rules {
        /// Show only rules from a specific Nannool verse
        #[arg(short, long)]
        verse: Option<u32>,
    },

    /// Generate paradigm (all forms) for a word
    Paradigm {
        /// The Tamil word (lemma/dictionary form)
        word: String,

        /// Part of speech
        #[arg(short, long, value_enum, default_value = "noun")]
        pos: WordType,

        /// For verbs: which tense to show
        #[arg(short, long, value_enum)]
        tense: Option<TenseArg>,

        /// Output format
        #[arg(short, long, value_enum, default_value = "human")]
        format: OutputFormat,
    },

    /// Generate a specific inflected form
    Inflect {
        /// The Tamil word (lemma/dictionary form)
        word: String,

        /// Case to apply (for nouns)
        #[arg(short, long, value_enum)]
        case: Option<CaseArg>,

        /// Tense to apply (for verbs)
        #[arg(short, long, value_enum)]
        tense: Option<TenseArg>,

        /// Person (for verbs)
        #[arg(long, value_enum)]
        person: Option<PersonArg>,

        /// Number
        #[arg(short, long, value_enum)]
        number: Option<NumberArg>,
    },

    /// Run spelling benchmark against a test corpus
    Benchmark {
        /// Path to benchmark JSON file
        input: PathBuf,

        /// Output format
        #[arg(short, long, value_enum, default_value = "human")]
        format: OutputFormat,

        /// Show detailed per-word results
        #[arg(short, long)]
        verbose: bool,

        /// Filter by error type (e.g., mayangoli_la_zha)
        #[arg(long)]
        error_type: Option<String>,
    },
}

#[derive(Clone, Copy, ValueEnum)]
enum OutputFormat {
    /// Human-readable output
    Human,
    /// JSON output
    Json,
    /// SARIF format (for IDE integration)
    Sarif,
}

#[derive(Clone, Copy, ValueEnum)]
enum Strictness {
    /// Classical Tamil rules only
    Classical,
    /// Standard modern Tamil
    Standard,
    /// Lenient - allow common deviations
    Lenient,
}

impl From<Strictness> for StrictnessLevel {
    fn from(s: Strictness) -> Self {
        match s {
            Strictness::Classical => StrictnessLevel::Classical,
            Strictness::Standard => StrictnessLevel::Standard,
            Strictness::Lenient => StrictnessLevel::Lenient,
        }
    }
}

#[derive(Clone, Copy, ValueEnum)]
enum WordType {
    /// Noun (பெயர்ச்சொல்)
    Noun,
    /// Verb (வினைச்சொல்)
    Verb,
}

#[derive(Clone, Copy, ValueEnum)]
enum CaseArg {
    /// Nominative (எழுவாய்)
    Nom,
    /// Accusative (ஐ வேற்றுமை)
    Acc,
    /// Instrumental (ஆல் வேற்றுமை)
    Ins,
    /// Dative (கு வேற்றுமை)
    Dat,
    /// Ablative (இன் வேற்றுமை)
    Abl,
    /// Genitive (அது வேற்றுமை)
    Gen,
    /// Locative (கண் வேற்றுமை)
    Loc,
    /// Vocative (விளி வேற்றுமை)
    Voc,
}

impl From<CaseArg> for nannool_morph::Case {
    fn from(c: CaseArg) -> Self {
        match c {
            CaseArg::Nom => nannool_morph::Case::Nominative,
            CaseArg::Acc => nannool_morph::Case::Accusative,
            CaseArg::Ins => nannool_morph::Case::Instrumental,
            CaseArg::Dat => nannool_morph::Case::Dative,
            CaseArg::Abl => nannool_morph::Case::Ablative,
            CaseArg::Gen => nannool_morph::Case::Genitive,
            CaseArg::Loc => nannool_morph::Case::Locative,
            CaseArg::Voc => nannool_morph::Case::Vocative,
        }
    }
}

#[derive(Clone, Copy, Debug, ValueEnum)]
enum TenseArg {
    /// Past (இறந்த காலம்)
    Past,
    /// Present (நிகழ் காலம்)
    Present,
    /// Future (எதிர் காலம்)
    Future,
}

impl From<TenseArg> for nannool_morph::Tense {
    fn from(t: TenseArg) -> Self {
        match t {
            TenseArg::Past => nannool_morph::Tense::Past,
            TenseArg::Present => nannool_morph::Tense::Present,
            TenseArg::Future => nannool_morph::Tense::Future,
        }
    }
}

#[derive(Clone, Copy, ValueEnum)]
enum PersonArg {
    /// First person (தன்மை)
    First,
    /// Second person (முன்னிலை)
    Second,
    /// Third person (படர்க்கை)
    Third,
}

impl From<PersonArg> for nannool_morph::Person {
    fn from(p: PersonArg) -> Self {
        match p {
            PersonArg::First => nannool_morph::Person::First,
            PersonArg::Second => nannool_morph::Person::Second,
            PersonArg::Third => nannool_morph::Person::Third,
        }
    }
}

#[derive(Clone, Copy, ValueEnum)]
enum NumberArg {
    /// Singular (ஒருமை)
    Sg,
    /// Plural (பன்மை)
    Pl,
}

impl From<NumberArg> for nannool_morph::Number {
    fn from(n: NumberArg) -> Self {
        match n {
            NumberArg::Sg => nannool_morph::Number::Singular,
            NumberArg::Pl => nannool_morph::Number::Plural,
        }
    }
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Check {
            input,
            text,
            format,
            level,
            no_color,
        } => {
            run_check(input, text, format, level.into(), !no_color)
        }

        Commands::Explain { rule_id } => {
            run_explain(&rule_id)
        }

        Commands::Analyze { word, format } => {
            run_analyze(&word, format)
        }

        Commands::Pair { word1, word2 } => {
            run_pair(&word1, &word2)
        }

        Commands::Rules { verse } => {
            run_list_rules(verse)
        }

        Commands::Paradigm { word, pos, tense, format } => {
            run_paradigm(&word, pos, tense, format)
        }

        Commands::Inflect { word, case, tense, person, number } => {
            run_inflect(&word, case, tense, person, number)
        }

        Commands::Benchmark { input, format, verbose, error_type } => {
            run_benchmark(&input, format, verbose, error_type.as_deref())
        }
    }
}

fn run_check(
    input: Option<PathBuf>,
    text: Option<String>,
    format: OutputFormat,
    level: StrictnessLevel,
    use_colors: bool,
) -> Result<()> {
    // Get the text to check
    let (content, file_path) = if let Some(path) = input {
        let content = fs::read_to_string(&path)
            .into_diagnostic()
            .wrap_err_with(|| format!("Failed to read file: {}", path.display()))?;
        (content, Some(path.display().to_string()))
    } else if let Some(t) = text {
        (t, None)
    } else {
        return Err(miette::miette!("Either input file or --text must be provided"));
    };

    // Run the checker with morphological analysis
    let engine = get_rule_engine().with_strictness(level);
    let mut checker = SandhiChecker::with_engine(engine).with_default_morph();

    let diagnostics = checker.check(&content);

    // Format output
    match format {
        OutputFormat::Human => {
            let output = format_diagnostics(
                &diagnostics,
                &content,
                file_path.as_deref(),
                use_colors,
            );
            print!("{}", output);

            if diagnostics.is_empty() {
                if use_colors {
                    println!("\x1b[32m✓ பிழைகள் இல்லை\x1b[0m");
                } else {
                    println!("✓ No errors found");
                }
            }
        }

        OutputFormat::Json => {
            let json = serde_json::to_string_pretty(&diagnostics)
                .into_diagnostic()?;
            println!("{}", json);
        }

        OutputFormat::Sarif => {
            let file = file_path.as_deref().unwrap_or("<input>");
            let sarif = to_sarif_report(&diagnostics, file);
            let json = serde_json::to_string_pretty(&sarif)
                .into_diagnostic()?;
            println!("{}", json);
        }
    }

    // Exit with error code if there are errors
    if diagnostics.iter().any(|d| matches!(d.level, nannool_checker::DiagnosticLevel::Error)) {
        std::process::exit(1);
    }

    Ok(())
}

fn run_explain(rule_id: &str) -> Result<()> {
    let engine = get_rule_engine();

    if let Some(rule) = engine.get_rule(rule_id) {
        println!("விதி: {}", rule.tamil_name);
        println!("Rule: {}", rule.english_name);
        println!();
        println!("ID: {}", rule.id);
        println!("நன்னூல்: {:?}", rule.nannool_verses);
        println!("வகை: {} ({})", rule.category.tamil_name(), rule.category.description());
        println!("முன்னுரிமை: {}", rule.priority);
        println!();

        if !rule.description.is_empty() {
            println!("விளக்கம்:");
            println!("  {}", rule.description);
            println!();
        }

        let verse_text = rule.verse_text();
        if !verse_text.is_empty() {
            println!("நூற்பா:");
            for line in verse_text.lines() {
                println!("  {}", line);
            }
        }

        Ok(())
    } else {
        Err(miette::miette!("Rule not found: {}", rule_id))
    }
}

fn run_analyze(word: &str, format: OutputFormat) -> Result<()> {
    let analyzer = nannool_morph::FomaAnalyzer::bundled()
        .into_diagnostic()
        .wrap_err("Failed to load bundled FST models")?;

    let all_analyses = analyzer.analyze(word)
        .into_diagnostic()
        .wrap_err_with(|| format!("Failed to analyze word: {}", word))?;

    match format {
        OutputFormat::Human => {
            println!("சொல்: {}", word);
            println!();

            if all_analyses.is_empty() {
                println!("பகுப்பாய்வு இல்லை (word not found in lexicon)");
            } else {
                for (i, analysis) in all_analyses.iter().enumerate() {
                    println!("பகுப்பாய்வு {}:", i + 1);
                    println!("  வேர்: {}", analysis.lemma);
                    println!("  வகை: {} ({})",
                        analysis.pos.tamil_name(),
                        format!("{:?}", analysis.pos).to_lowercase()
                    );

                    if !analysis.features.is_empty() {
                        println!("  அம்சங்கள்:");
                        for feature in &analysis.features {
                            println!("    - {:?}", feature);
                        }
                    }

                    if !analysis.semantic_tags.is_empty() {
                        println!("  பொருள் வகை (Semantic tags):");
                        for tag in &analysis.semantic_tags {
                            println!("    - {}", tag);
                        }
                    }

                    println!("  நம்பிக்கை: {:.0}%", analysis.confidence * 100.0);
                    println!();
                }
            }
        }

        OutputFormat::Json | OutputFormat::Sarif => {
            let json = serde_json::to_string_pretty(&all_analyses)
                .into_diagnostic()?;
            println!("{}", json);
        }
    }

    Ok(())
}

fn run_pair(word1: &str, word2: &str) -> Result<()> {
    let mut checker = SandhiChecker::new();

    println!("சொல் 1: {}", word1);
    println!("சொல் 2: {}", word2);
    println!();

    // Get applicable rules
    let rules = checker.get_applicable_rules(word1, word2);
    if rules.is_empty() {
        println!("பொருந்தும் விதிகள் இல்லை");
    } else {
        println!("பொருந்தும் விதிகள்:");
        for rule in &rules {
            println!("  - {} ({})", rule.tamil_name, rule.id);
        }
        println!();
    }

    // Check for violations
    if let Some(diag) = checker.check_pair(word1, word2) {
        println!("\x1b[31mபிழை:\x1b[0m {}", diag.message);
        if let Some(suggestion) = &diag.suggestion {
            println!("\x1b[32mதிருத்தம்:\x1b[0m {}", suggestion.replacement);
        }
    } else {
        println!("\x1b[32m✓ புணர்ச்சி சரி\x1b[0m");
    }

    Ok(())
}

fn run_list_rules(verse: Option<u32>) -> Result<()> {
    let engine = get_rule_engine();

    let rules: Vec<_> = if let Some(v) = verse {
        engine.get_rules_by_verse(v)
    } else {
        engine.rules().iter().collect()
    };

    if rules.is_empty() {
        if let Some(v) = verse {
            println!("நன்னூல் {} விதிகள் இல்லை", v);
        } else {
            println!("விதிகள் இல்லை");
        }
        return Ok(());
    }

    println!("விதிகள் ({}):", rules.len());
    println!();

    for rule in rules {
        println!("{}", rule.id);
        println!("  தமிழ்: {}", rule.tamil_name);
        println!("  English: {}", rule.english_name);
        println!("  நன்னூல்: {:?}", rule.nannool_verses);
        println!("  வகை: {}", rule.category.tamil_name());
        println!();
    }

    Ok(())
}

fn get_fst_analyzer() -> Result<nannool_morph::FomaAnalyzer> {
    nannool_morph::FomaAnalyzer::bundled()
        .into_diagnostic()
        .wrap_err("Failed to load bundled FST models")
}

fn run_paradigm(
    word: &str,
    pos: WordType,
    tense: Option<TenseArg>,
    format: OutputFormat,
) -> Result<()> {
    let analyzer = get_fst_analyzer()?;

    match pos {
        WordType::Noun => {
            let paradigm = analyzer.generate_noun_paradigm(word)
                .into_diagnostic()
                .wrap_err("Failed to generate paradigm")?;

            match format {
                OutputFormat::Human => {
                    println!("பெயர்ச்சொல் வேற்றுமை அட்டவணை: {}", word);
                    println!();
                    println!("{:<20} {:<20} {}", "வேற்றுமை", "Case", "வடிவம்");
                    println!("{}", "-".repeat(60));

                    for (case, forms) in paradigm {
                        let forms_str = if forms.is_empty() {
                            "-".to_string()
                        } else {
                            forms.join(", ")
                        };
                        println!("{:<20} {:<20} {}",
                            case.tamil_name(),
                            format!("{:?}", case).to_lowercase(),
                            forms_str
                        );
                    }
                }
                OutputFormat::Json | OutputFormat::Sarif => {
                    let json_paradigm: Vec<_> = paradigm.iter()
                        .map(|(case, forms)| serde_json::json!({
                            "case": format!("{:?}", case).to_lowercase(),
                            "tamil_name": case.tamil_name(),
                            "forms": forms
                        }))
                        .collect();
                    let json = serde_json::to_string_pretty(&json_paradigm)
                        .into_diagnostic()?;
                    println!("{}", json);
                }
            }
        }
        WordType::Verb => {
            let tense = tense.unwrap_or(TenseArg::Past);
            let paradigm = analyzer.generate_verb_paradigm(word, tense.into())
                .into_diagnostic()
                .wrap_err("Failed to generate verb paradigm")?;

            match format {
                OutputFormat::Human => {
                    println!("வினைச்சொல் வடிவங்கள்: {} ({:?})", word, tense);
                    println!();
                    println!("{:<10} {:<10} {:<15} {}", "Person", "Number", "Gender", "Form");
                    println!("{}", "-".repeat(60));

                    for (person, number, gender, forms) in paradigm {
                        let forms_str = if forms.is_empty() {
                            "-".to_string()
                        } else {
                            forms.join(", ")
                        };
                        println!("{:<10} {:<10} {:<15} {}",
                            format!("{:?}", person),
                            format!("{:?}", number),
                            format!("{:?}", gender),
                            forms_str
                        );
                    }
                }
                OutputFormat::Json | OutputFormat::Sarif => {
                    let json_paradigm: Vec<_> = paradigm.iter()
                        .map(|(person, number, gender, forms)| serde_json::json!({
                            "person": format!("{:?}", person).to_lowercase(),
                            "number": format!("{:?}", number).to_lowercase(),
                            "gender": format!("{:?}", gender).to_lowercase(),
                            "forms": forms
                        }))
                        .collect();
                    let json = serde_json::to_string_pretty(&json_paradigm)
                        .into_diagnostic()?;
                    println!("{}", json);
                }
            }
        }
    }

    Ok(())
}

fn run_inflect(
    word: &str,
    case: Option<CaseArg>,
    tense: Option<TenseArg>,
    person: Option<PersonArg>,
    number: Option<NumberArg>,
) -> Result<()> {
    let analyzer = get_fst_analyzer()?;

    // Build features list
    let mut features: Vec<nannool_morph::Feature> = Vec::new();

    if let Some(c) = case {
        features.push(nannool_morph::Feature::Case(c.into()));
    }

    if let Some(t) = tense {
        features.push(nannool_morph::Feature::Tense(t.into()));
    }

    if let Some(p) = person {
        features.push(nannool_morph::Feature::Person(p.into()));
    }

    if let Some(n) = number {
        features.push(nannool_morph::Feature::Number(n.into()));
    }

    if features.is_empty() {
        return Err(miette::miette!("At least one feature (--case, --tense, --person, --number) must be specified"));
    }

    // Determine POS based on features
    let pos = if case.is_some() {
        nannool_morph::PartOfSpeech::Noun
    } else {
        nannool_morph::PartOfSpeech::Verb
    };

    let forms = analyzer.generate(word, pos, &features)
        .into_diagnostic()
        .wrap_err("Failed to generate form")?;

    if forms.is_empty() {
        println!("வடிவம் கிடைக்கவில்லை (form not found in lexicon)");
    } else {
        println!("வேர்: {}", word);
        println!("வடிவம்: {}", forms.join(", "));
    }

    Ok(())
}

// ============================================================================
// Benchmark types and runner
// ============================================================================

#[derive(Debug, Deserialize)]
struct BenchmarkData {
    #[allow(dead_code)]
    dataset_info: DatasetInfo,
    incorrect_words: Vec<IncorrectWord>,
    correct_words: Vec<CorrectWord>,
}

#[derive(Debug, Deserialize)]
struct DatasetInfo {
    #[allow(dead_code)]
    language: String,
    #[allow(dead_code)]
    total_count: u32,
    description: String,
}

#[derive(Debug, Deserialize)]
struct IncorrectWord {
    #[allow(dead_code)]
    id: u32,
    input: String,
    expected: String,
    error_type: String,
    #[allow(dead_code)]
    note: String,
}

#[derive(Debug, Deserialize)]
struct CorrectWord {
    #[allow(dead_code)]
    id: u32,
    word: String,
}

#[derive(Debug, Default)]
struct BenchmarkResults {
    total_incorrect: usize,
    detected_as_incorrect: usize,
    correct_in_top1: usize,
    correct_in_top3: usize,
    correct_in_top5: usize,
    no_suggestions: usize,
    total_correct: usize,
    false_positives: usize,
    by_error_type: HashMap<String, ErrorTypeStats>,
}

#[derive(Debug, Default, Clone)]
struct ErrorTypeStats {
    total: usize,
    detected: usize,
    top1: usize,
    top5: usize,
}

#[derive(Debug)]
struct WordResult {
    input: String,
    expected: String,
    error_type: String,
    detected: bool,
    suggestions: Vec<String>,
    correct_rank: Option<usize>,
}

fn run_benchmark(
    input: &std::path::Path,
    format: OutputFormat,
    verbose: bool,
    error_type_filter: Option<&str>,
) -> Result<()> {
    let content = fs::read_to_string(input)
        .into_diagnostic()
        .wrap_err_with(|| format!("Failed to read benchmark file: {}", input.display()))?;

    let benchmark: BenchmarkData = serde_json::from_str(&content)
        .into_diagnostic()
        .wrap_err("Failed to parse benchmark JSON")?;

    println!("எழுத்துப் பிழை திறன் சோதனை / Spelling Benchmark");
    println!("================================================");
    println!("Dataset: {}", benchmark.dataset_info.description);
    println!();

    let speller = SpellingChecker::new();

    let mut results = BenchmarkResults::default();
    let mut word_results: Vec<WordResult> = Vec::new();

    let incorrect_words: Vec<_> = benchmark.incorrect_words.iter()
        .filter(|w| {
            if let Some(filter) = error_type_filter {
                w.error_type.contains(filter)
            } else {
                !matches!(w.error_type.as_str(), "correct" | "none" | "correct_marked_wrong")
            }
        })
        .collect();

    results.total_incorrect = incorrect_words.len();
    println!("Testing {} incorrect words...", results.total_incorrect);

    for word in &incorrect_words {
        // Detection: word is detected as misspelled if NOT in dictionary
        let detected = !speller.is_valid(&word.input);
        let suggestions = speller.suggest(&word.input);
        let correct_rank = suggestions.iter()
            .position(|s| s == &word.expected)
            .map(|p| p + 1);

        if detected { results.detected_as_incorrect += 1; }
        if suggestions.is_empty() { results.no_suggestions += 1; }

        match correct_rank {
            Some(1) => {
                results.correct_in_top1 += 1;
                results.correct_in_top3 += 1;
                results.correct_in_top5 += 1;
            }
            Some(2) | Some(3) => {
                results.correct_in_top3 += 1;
                results.correct_in_top5 += 1;
            }
            Some(4) | Some(5) => {
                results.correct_in_top5 += 1;
            }
            _ => {}
        }

        let type_stats = results.by_error_type.entry(word.error_type.clone()).or_default();
        type_stats.total += 1;
        if detected { type_stats.detected += 1; }
        if correct_rank == Some(1) { type_stats.top1 += 1; }
        if correct_rank.map(|r| r <= 5).unwrap_or(false) { type_stats.top5 += 1; }

        word_results.push(WordResult {
            input: word.input.clone(),
            expected: word.expected.clone(),
            error_type: word.error_type.clone(),
            detected,
            suggestions,
            correct_rank,
        });
    }

    results.total_correct = benchmark.correct_words.len();
    println!("Testing {} correct words for false positives...", results.total_correct);

    for word in &benchmark.correct_words {
        // False positive: correct word NOT in dictionary (incorrectly flagged)
        if !speller.is_valid(&word.word) {
            results.false_positives += 1;
            if verbose {
                println!("  FALSE POSITIVE: {} (flagged as error but is correct)", word.word);
            }
        }
    }

    println!();

    match format {
        OutputFormat::Human => print_benchmark_results(&results, &word_results, verbose),
        OutputFormat::Json | OutputFormat::Sarif => {
            let json_results = serde_json::json!({
                "detection": {
                    "total": results.total_incorrect,
                    "detected": results.detected_as_incorrect,
                    "rate": pct(results.detected_as_incorrect, results.total_incorrect)
                },
                "suggestions": {
                    "recall_at_1": pct(results.correct_in_top1, results.total_incorrect),
                    "recall_at_3": pct(results.correct_in_top3, results.total_incorrect),
                    "recall_at_5": pct(results.correct_in_top5, results.total_incorrect),
                    "no_suggestions": results.no_suggestions
                },
                "false_positives": {
                    "total": results.total_correct,
                    "flagged": results.false_positives,
                    "rate": pct(results.false_positives, results.total_correct)
                },
                "by_error_type": results.by_error_type.iter().map(|(k, v)| {
                    (k.clone(), serde_json::json!({
                        "total": v.total, "detected": v.detected, "top1": v.top1, "top5": v.top5
                    }))
                }).collect::<HashMap<_, _>>()
            });
            println!("{}", serde_json::to_string_pretty(&json_results).unwrap());
        }
    }

    Ok(())
}

fn pct(num: usize, denom: usize) -> f64 {
    if denom > 0 { num as f64 / denom as f64 * 100.0 } else { 0.0 }
}

fn print_benchmark_results(results: &BenchmarkResults, word_results: &[WordResult], verbose: bool) {
    println!("═══════════════════════════════════════════════════════════════");
    println!("                     DETECTION METRICS");
    println!("═══════════════════════════════════════════════════════════════");
    println!("  Misspelled words detected: {}/{} ({:.1}%)",
        results.detected_as_incorrect, results.total_incorrect,
        pct(results.detected_as_incorrect, results.total_incorrect));

    println!();
    println!("═══════════════════════════════════════════════════════════════");
    println!("                    SUGGESTION METRICS");
    println!("═══════════════════════════════════════════════════════════════");
    println!("  Recall@1 (correct is top suggestion):  {}/{} ({:.1}%)",
        results.correct_in_top1, results.total_incorrect,
        pct(results.correct_in_top1, results.total_incorrect));
    println!("  Recall@3 (correct in top 3):           {}/{} ({:.1}%)",
        results.correct_in_top3, results.total_incorrect,
        pct(results.correct_in_top3, results.total_incorrect));
    println!("  Recall@5 (correct in top 5):           {}/{} ({:.1}%)",
        results.correct_in_top5, results.total_incorrect,
        pct(results.correct_in_top5, results.total_incorrect));
    println!("  No suggestions generated:              {}", results.no_suggestions);

    println!();
    println!("═══════════════════════════════════════════════════════════════");
    println!("                   FALSE POSITIVE METRICS");
    println!("═══════════════════════════════════════════════════════════════");
    println!("  Correct words flagged as errors: {}/{} ({:.1}%)",
        results.false_positives, results.total_correct,
        pct(results.false_positives, results.total_correct));

    println!();
    println!("═══════════════════════════════════════════════════════════════");
    println!("                   BY ERROR TYPE");
    println!("═══════════════════════════════════════════════════════════════");

    let mut types: Vec<_> = results.by_error_type.iter().collect();
    types.sort_by(|a, b| b.1.total.cmp(&a.1.total));

    println!("  {:<30} {:>6} {:>8} {:>8} {:>8}", "Error Type", "Total", "Detect%", "Top1%", "Top5%");
    println!("  {}", "-".repeat(62));

    for (error_type, stats) in types {
        println!("  {:<30} {:>6} {:>7.1}% {:>7.1}% {:>7.1}%",
            error_type, stats.total,
            pct(stats.detected, stats.total),
            pct(stats.top1, stats.total),
            pct(stats.top5, stats.total));
    }

    if verbose {
        println!();
        println!("═══════════════════════════════════════════════════════════════");
        println!("                   DETAILED RESULTS");
        println!("═══════════════════════════════════════════════════════════════");

        for result in word_results {
            let status = if result.correct_rank == Some(1) { "\x1b[32m✓\x1b[0m" }
                else if result.correct_rank.is_some() { "\x1b[33m◐\x1b[0m" }
                else if result.detected { "\x1b[31m✗\x1b[0m" }
                else { "\x1b[31m⊘\x1b[0m" };

            println!("{} {} → {} [{}]", status, result.input, result.expected, result.error_type);

            if !result.suggestions.is_empty() {
                let suggestions_str: Vec<_> = result.suggestions.iter().enumerate()
                    .map(|(i, s)| if s == &result.expected {
                        format!("\x1b[32m{}. {}\x1b[0m", i + 1, s)
                    } else {
                        format!("{}. {}", i + 1, s)
                    }).collect();
                println!("    Suggestions: {}", suggestions_str.join(", "));
            } else if result.detected {
                println!("    Suggestions: (none)");
            } else {
                println!("    NOT DETECTED as misspelling");
            }
        }
    }

    println!();
    println!("═══════════════════════════════════════════════════════════════");
    println!("                       SUMMARY");
    println!("═══════════════════════════════════════════════════════════════");
    println!("  Overall Recall@5: {:.1}%", pct(results.correct_in_top5, results.total_incorrect));
    println!("  (Paper claims 91% - this is our comparable metric)");
}
