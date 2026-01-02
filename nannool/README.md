# நன்னூல் (Nannool) - Tamil Grammar Checker

A Rust-based syntax/grammar checker for the Tamil language, modeled after how `rustc` reports errors. Validates Tamil text against the grammatical rules codified in **Nannool (நன்னூல்)**, a 13th-century Tamil grammar text by Pavananthi Munivar.

## Features

- **Sandhi checking** - Validates word boundaries against 77 Nannool புணர்ச்சி rules
- **Morphological analysis** - Uses ThamizhiMorph FST models (80k+ nouns, 3.3k+ verbs)
- **Rule-based Engine** - Rules are defined in a structured TOML format (`data/nannool/rules.toml`)
- **Rust-style diagnostics** - Error messages with source context and precise fix suggestions
- **Multiple output formats** - Human-readable, JSON, SARIF (for IDE integration)

## Quick Start

```bash
# Build
cargo build --release

# Download ThamizhiMorph FST models (required for analysis)
git clone --depth 1 https://github.com/sarves/thamizhi-morph data/thamizhi-morph

# Check a file
./target/release/nannool check input.txt

# Check inline text
./target/release/nannool check --text "பாட்டு பாடினான்"

# Analyze a word
./target/release/nannool analyze வந்தான்
```

## Example Output

```
error[vallinam-miguthal-165]: புணர்ச்சி பிழை: வல்லினம் மிகுதல்
       Sandhi error: Vallinam doubling
  --> input.txt:1:1
   |
 1 | பாட்டு பாடினான்
   | ^^^^^^^^^^^^^^^^ expected: பாட்டுப்பாடினான்
   |
   = note: நன்னூல் 165 - Hard consonants (க ச ட த ப ற) double after vowel endings
   = help: Apply வல்லினம் மிகுதல் (Vallinam doubling)
```

## Morphological Analysis

```
$ nannool analyze மரத்தை

சொல்: மரத்தை

பகுப்பாய்வு 1:
  வேர்: மரம்
  வகை: பெயர்ச்சொல் (noun)
  அம்சங்கள்:
    - Case(Accusative)
  நம்பிக்கை: 100%
```

## Project Structure

```
nannool/
├── crates/
│   ├── tamil-unicode/      # Tamil Unicode handling, grapheme clusters, NFC normalization
│   ├── nannool-rules/      # Sandhi rule definitions & logic engine
│   ├── nannool-morph/      # Morphological analysis (ThamizhiMorph wrapper)
│   ├── nannool-checker/    # Main checking logic, diagnostics & tokenization
│   └── nannool-cli/        # CLI binary
└── data/
    ├── nannool/            # Rule database (rules.toml)
    └── thamizhi-morph/     # ThamizhiMorph FST models (external dependency)
```

## Requirements

- Rust 1.70+
- Foma FST toolkit (`flookup` binary) - `paru -S foma` on Arch
- ThamizhiMorph FST models (placed in `data/thamizhi-morph`)

## CLI Commands

```bash
# Check a file for sandhi errors
nannool check <file>
nannool check --text "..."
nannool check --format json input.txt
nannool check --level classical input.txt

# Analyze word morphology
nannool analyze <word>
nannool analyze --format json வந்தான்

# Check a word pair
nannool pair பாட்டு பாடினான்

# Explain a rule
nannool explain vallinam-miguthal-165

# List all rules
nannool rules
nannool rules --verse 165
```

## Implemented Rules

The engine currently implements **77 rules** covering:

- **உயிரீற்றுப் புணரியல் (Vowel-ending Sandhi)**: Buffer consonants (162), Short u elision (164), Vallinam doubling (165), etc.
- **மெய்யீற்றுப் புணரியல் (Consonant-ending Sandhi)**: Consonant-vowel combination (204), Consonant doubling (205), Alveolar n assimilation (209), Mam ending (217), etc.
- **உருபு புணரியல் (Case Marker Sandhi)**: Glide/epenthesis (243-244), Pronoun shortening (247), etc.

See `data/nannool/rules.toml` for the full list of formalized rules.

## External Resources

- **Nannool text**: [Project Madurai](https://www.projectmadurai.org/pm_etexts/utf8/pmuni0147.html)
- **ThamizhiMorph**: [github.com/sarves/thamizhi-morph](https://github.com/sarves/thamizhi-morph) (Apache 2.0)
- **Tamil Virtual Academy**: [tamilvu.org](https://www.tamilvu.org/library/l0900/html/l0900ind.htm)

## License

AGPL-3.0
