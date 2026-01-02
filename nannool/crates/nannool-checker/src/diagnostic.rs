//! Diagnostic types and formatting.
//!
//! Provides rust-style error formatting for sandhi violations.

use serde::{Deserialize, Serialize};
use crate::tokenizer::Span;
use std::fmt;

/// Diagnostic severity level
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DiagnosticLevel {
    /// Error - must be fixed
    Error,
    /// Warning - should be fixed
    Warning,
    /// Info - informational
    Info,
    /// Hint - suggestion
    Hint,
}

impl DiagnosticLevel {
    /// Get the color code for terminal output
    pub fn color_code(&self) -> &'static str {
        match self {
            DiagnosticLevel::Error => "\x1b[31m",   // Red
            DiagnosticLevel::Warning => "\x1b[33m", // Yellow
            DiagnosticLevel::Info => "\x1b[36m",    // Cyan
            DiagnosticLevel::Hint => "\x1b[32m",    // Green
        }
    }

    /// Get the label
    pub fn label(&self) -> &'static str {
        match self {
            DiagnosticLevel::Error => "error",
            DiagnosticLevel::Warning => "warning",
            DiagnosticLevel::Info => "info",
            DiagnosticLevel::Hint => "hint",
        }
    }
}

/// A suggestion for fixing a diagnostic
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Suggestion {
    /// The replacement text
    pub replacement: String,
    /// Description of the fix
    pub description: String,
}

/// A diagnostic message
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Diagnostic {
    /// Severity level
    pub level: DiagnosticLevel,
    /// Main message (Tamil)
    pub message: String,
    /// English message (optional)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub english_message: Option<String>,
    /// Source location
    pub span: Span,
    /// The source text that triggered this diagnostic
    pub source_text: String,
    /// Rule ID that was violated
    pub rule_id: String,
    /// Nannool verse numbers
    pub nannool_verses: Vec<u32>,
    /// Suggested fix
    #[serde(skip_serializing_if = "Option::is_none")]
    pub suggestion: Option<Suggestion>,
    /// Additional notes
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub notes: Vec<String>,
}

impl Diagnostic {
    /// Format the diagnostic for terminal output (rust-style)
    pub fn format(&self, source: &str, file_path: Option<&str>) -> String {
        let mut output = String::new();
        let reset = "\x1b[0m";
        let bold = "\x1b[1m";
        let blue = "\x1b[34m";

        // Header: error[rule-id]: message
        output.push_str(&format!(
            "{}{}{bold}[{}]{reset}: {bold}{}{reset}",
            self.level.color_code(),
            self.level.label(),
            self.rule_id,
            self.message,
        ));

        // English message if available
        if let Some(eng) = &self.english_message {
            output.push_str(&format!("\n       {}", eng));
        }

        output.push('\n');

        // Location: --> file:line:column
        let path = file_path.unwrap_or("<input>");
        output.push_str(&format!(
            "  {blue}-->{reset} {}:{}:{}\n",
            path, self.span.line, self.span.column
        ));

        // Source context
        if !source.is_empty() {
            let lines: Vec<&str> = source.lines().collect();
            let line_idx = (self.span.line as usize).saturating_sub(1);

            if line_idx < lines.len() {
                let line = lines[line_idx];
                let line_num = format!("{}", self.span.line);
                let padding = " ".repeat(line_num.len());

                // Empty line above
                output.push_str(&format!("   {blue}{}{reset} |\n", padding));

                // The source line
                output.push_str(&format!(
                    "   {blue}{}{reset} | {}\n",
                    line_num, line
                ));

                // The underline/pointer
                let col = (self.span.column as usize).saturating_sub(1);
                let underline_start = " ".repeat(col);

                // Calculate underline length (approximate for now)
                let underline_len = self.source_text.chars().count().max(1);
                let underline = "^".repeat(underline_len);

                output.push_str(&format!(
                    "   {blue}{}{reset} | {}{}{}{}{reset}",
                    padding,
                    underline_start,
                    self.level.color_code(),
                    underline,
                    reset,
                ));

                // Label on underline
                if let Some(ref suggestion) = self.suggestion {
                    output.push_str(&format!(
                        " expected: {}",
                        suggestion.replacement
                    ));
                }
                output.push('\n');
            }
        }

        // Notes
        for note in &self.notes {
            output.push_str(&format!("   {blue}={reset} {}: {}\n", "note", note));
        }

        // Nannool reference
        if !self.nannool_verses.is_empty() {
            let verses: Vec<String> = self.nannool_verses
                .iter()
                .map(|v| v.to_string())
                .collect();
            output.push_str(&format!(
                "   {blue}={reset} நன்னூல்: {}\n",
                verses.join(", ")
            ));
        }

        // Suggestion
        if let Some(ref suggestion) = self.suggestion {
            output.push_str(&format!(
                "   {blue}={reset} help: {}\n",
                suggestion.description
            ));
        }

        output
    }

    /// Format as plain text (no colors)
    pub fn format_plain(&self, file_path: Option<&str>) -> String {
        let mut output = String::new();

        let path = file_path.unwrap_or("<input>");
        output.push_str(&format!(
            "{}[{}]: {}\n",
            self.level.label(),
            self.rule_id,
            self.message,
        ));

        output.push_str(&format!(
            "  --> {}:{}:{}\n",
            path, self.span.line, self.span.column
        ));

        if let Some(ref suggestion) = self.suggestion {
            output.push_str(&format!(
                "  = expected: {}\n",
                suggestion.replacement
            ));
        }

        for note in &self.notes {
            output.push_str(&format!("  = note: {}\n", note));
        }

        output
    }

    /// Convert to SARIF format for IDE integration
    pub fn to_sarif(&self, file_path: &str) -> serde_json::Value {
        let level = match self.level {
            DiagnosticLevel::Error => "error",
            DiagnosticLevel::Warning => "warning",
            DiagnosticLevel::Info | DiagnosticLevel::Hint => "note",
        };

        let mut result = serde_json::json!({
            "ruleId": self.rule_id,
            "level": level,
            "message": {
                "text": self.message
            },
            "locations": [{
                "physicalLocation": {
                    "artifactLocation": {
                        "uri": file_path
                    },
                    "region": {
                        "startLine": self.span.line,
                        "startColumn": self.span.column,
                        "endLine": self.span.line,
                        "endColumn": self.span.column + self.source_text.len() as u32
                    }
                }
            }]
        });

        // Add fixes if there's a suggestion
        if let Some(ref suggestion) = self.suggestion {
            result["fixes"] = serde_json::json!([{
                "description": {
                    "text": suggestion.description.clone()
                },
                "artifactChanges": [{
                    "artifactLocation": {
                        "uri": file_path
                    },
                    "replacements": [{
                        "deletedRegion": {
                            "startLine": self.span.line,
                            "startColumn": self.span.column,
                            "endColumn": self.span.column + self.source_text.len() as u32
                        },
                        "insertedContent": {
                            "text": suggestion.replacement.clone()
                        }
                    }]
                }]
            }]);
        }

        result
    }
}

impl fmt::Display for Diagnostic {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.format_plain(None))
    }
}

/// Format multiple diagnostics
pub fn format_diagnostics(
    diagnostics: &[Diagnostic],
    source: &str,
    file_path: Option<&str>,
    use_colors: bool,
) -> String {
    let mut output = String::new();

    for diag in diagnostics {
        if use_colors {
            output.push_str(&diag.format(source, file_path));
        } else {
            output.push_str(&diag.format_plain(file_path));
        }
        output.push('\n');
    }

    // Summary
    let error_count = diagnostics
        .iter()
        .filter(|d| matches!(d.level, DiagnosticLevel::Error))
        .count();
    let warning_count = diagnostics
        .iter()
        .filter(|d| matches!(d.level, DiagnosticLevel::Warning))
        .count();

    if error_count > 0 || warning_count > 0 {
        let bold = if use_colors { "\x1b[1m" } else { "" };
        let reset = if use_colors { "\x1b[0m" } else { "" };
        let red = if use_colors { "\x1b[31m" } else { "" };
        let yellow = if use_colors { "\x1b[33m" } else { "" };

        output.push_str(&format!(
            "{bold}சுருக்கம்:{reset} ",
        ));

        if error_count > 0 {
            output.push_str(&format!(
                "{red}{} பிழை{}{reset}",
                error_count,
                if error_count > 1 { "கள்" } else { "" }
            ));
        }

        if warning_count > 0 {
            if error_count > 0 {
                output.push_str(", ");
            }
            output.push_str(&format!(
                "{yellow}{} எச்சரிக்கை{}{reset}",
                warning_count,
                if warning_count > 1 { "கள்" } else { "" }
            ));
        }

        output.push('\n');
    }

    output
}

/// Convert diagnostics to SARIF format
pub fn to_sarif_report(diagnostics: &[Diagnostic], file_path: &str) -> serde_json::Value {
    serde_json::json!({
        "$schema": "https://raw.githubusercontent.com/oasis-tcs/sarif-spec/master/Schemata/sarif-schema-2.1.0.json",
        "version": "2.1.0",
        "runs": [{
            "tool": {
                "driver": {
                    "name": "nannool",
                    "version": env!("CARGO_PKG_VERSION"),
                    "informationUri": "https://github.com/avvai/nannool",
                    "rules": []
                }
            },
            "results": diagnostics.iter().map(|d| d.to_sarif(file_path)).collect::<Vec<_>>()
        }]
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample_diagnostic() -> Diagnostic {
        Diagnostic {
            level: DiagnosticLevel::Error,
            message: "புணர்ச்சி பிழை: வல்லினம் மிகுதல்".to_string(),
            english_message: Some("Sandhi error: Vallinam doubling".to_string()),
            span: Span::new(0, 20, 1, 1),
            source_text: "பாட்டு பாடினான்".to_string(),
            rule_id: "vallinam-miguthal-165".to_string(),
            nannool_verses: vec![165],
            suggestion: Some(Suggestion {
                replacement: "பாட்டுப் பாடினான்".to_string(),
                description: "Apply வல்லினம் மிகுதல்".to_string(),
            }),
            notes: vec!["வல்லினம் மிகவேண்டும்".to_string()],
        }
    }

    #[test]
    fn test_diagnostic_format() {
        let diag = sample_diagnostic();
        let source = "பாட்டு பாடினான்";
        let formatted = diag.format(source, Some("test.txt"));

        assert!(formatted.contains("error"));
        assert!(formatted.contains("vallinam-miguthal-165"));
        assert!(formatted.contains("test.txt"));
    }

    #[test]
    fn test_diagnostic_format_plain() {
        let diag = sample_diagnostic();
        let formatted = diag.format_plain(Some("test.txt"));

        assert!(formatted.contains("error"));
        assert!(formatted.contains("vallinam-miguthal-165"));
    }

    #[test]
    fn test_format_diagnostics() {
        let diagnostics = vec![sample_diagnostic()];
        let source = "பாட்டு பாடினான்";

        let output = format_diagnostics(&diagnostics, source, Some("test.txt"), false);
        assert!(output.contains("1 பிழை"));
    }

    #[test]
    fn test_sarif_output() {
        let diag = sample_diagnostic();
        let sarif = diag.to_sarif("test.txt");

        assert_eq!(sarif["ruleId"], "vallinam-miguthal-165");
        assert_eq!(sarif["level"], "error");
    }
}
