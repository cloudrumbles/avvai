//! # Nannool CLI
//!
//! Command-line interface for the Nannool Tamil grammar checker.
//!
//! Usage:
//!   nannool check <file>       Check a file for sandhi errors
//!   nannool explain <rule-id>  Explain a rule
//!   nannool analyze <word>     Analyze a word morphologically

use clap::{Parser, Subcommand, ValueEnum};
use miette::{IntoDiagnostic, Result, WrapErr};
use nannool_checker::{SandhiChecker, format_diagnostics, to_sarif_report};
use nannool_rules::StrictnessLevel;
use std::fs;
use std::path::PathBuf;

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

    // Run the checker
    let checker = SandhiChecker::with_strictness(level);
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
    let engine = nannool_rules::RuleEngine::new();

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
    let analyzer = nannool_morph::MorphAnalyzer::new();
    let analyses = analyzer.analyze(word);

    match format {
        OutputFormat::Human => {
            println!("சொல்: {}", word);
            println!();

            if analyses.is_empty() {
                println!("பகுப்பாய்வு இல்லை");
            } else {
                for (i, analysis) in analyses.iter().enumerate() {
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

                    println!("  நம்பிக்கை: {:.0}%", analysis.confidence * 100.0);
                    println!();
                }
            }
        }

        OutputFormat::Json | OutputFormat::Sarif => {
            let json = serde_json::to_string_pretty(&analyses)
                .into_diagnostic()?;
            println!("{}", json);
        }
    }

    Ok(())
}

fn run_pair(word1: &str, word2: &str) -> Result<()> {
    let checker = SandhiChecker::new();

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
    let engine = nannool_rules::RuleEngine::new();

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
