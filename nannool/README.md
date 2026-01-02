# நன்னூல் (Nannool) - Tamil Grammar Checker

A Rust-based syntax/grammar checker for the Tamil language, modeled after how `rustc` reports errors. The checker validates Tamil text against the grammatical rules codified in Nannool (நன்னூல்), a 13th-century Tamil grammar text by Pavananthi Munivar.

## Features

- **Tamil Unicode Handling**: Proper grapheme cluster splitting and letter classification
- **Sandhi (புணர்ச்சி) Checking**: Validates word boundaries against Nannool rules
- **Morphological Analysis**: Basic word decomposition into root + features
- **Rust-style Diagnostics**: Clear, colorful error messages with suggestions
- **Multiple Output Formats**: Human-readable, JSON, and SARIF for IDE integration

## Installation

```bash
# From the nannool directory
cargo build --release

# The binary will be at target/release/nannool
```

## Usage

### Check a file

```bash
nannool check input.txt
```

### Check inline text

```bash
nannool check --text "பாட்டு பாடினான்"
```

### Explain a rule

```bash
nannool explain vallinam-miguthal-165
```

### Analyze a word

```bash
nannool analyze பாடினான்
```

### Check a word pair

```bash
nannool pair பாட்டு பாடினான்
```

### List all rules

```bash
nannool rules
```

## Output Example

```
error[vallinam-miguthal-165]: புணர்ச்சி பிழை: வல்லினம் மிகுதல்
       Sandhi error: Vallinam doubling
  --> input.txt:1:1
   |
 1 | பாட்டு பாடினான்
   | ^^^^^^^^^^^^^^^^ expected: பாட்டுப்பாடினான்
   |
   = note: வல்லினம் மிகவேண்டும்
   = நன்னூல்: 165
   = help: Apply வல்லினம் மிகுதல் (Vallinam doubling)
```

## Strictness Levels

- `--level classical`: Strict classical Tamil rules
- `--level standard` (default): Standard modern Tamil
- `--level lenient`: Allow common deviations

## Architecture

```
nannool/
├── crates/
│   ├── tamil-unicode/    # Tamil Unicode handling
│   ├── nannool-rules/    # Sandhi rule definitions
│   ├── nannool-morph/    # Morphological analyzer
│   ├── nannool-checker/  # Main checking logic
│   └── nannool-cli/      # CLI binary
└── data/
    ├── nannool/          # Rule definitions
    └── tests/            # Test cases
```

## Implemented Rules

Currently implements these key Nannool sandhi rules:

| Verse | Tamil Name | English Name | Category |
|-------|------------|--------------|----------|
| 165 | வல்லினம் மிகுதல் | Vallinam doubling | தோன்றல் |
| 162 | உடம்படுமெய் | Buffer consonant | தோன்றல் |
| 204 | மெய்யும் உயிரும் புணர்தல் | Consonant-vowel combination | இயல்பு |
| 205 | ஒற்று இரட்டல் | Consonant doubling | தோன்றல் |
| 206 | குற்றியலுகரப் புணர்ச்சி | Kutriyalukaram sandhi | தோன்றல் |
| 217 | மகர இறுதி புணர்ச்சி | Mam + vallinam sandhi | திரிதல் |

## References

- [Nannool Full Text](https://www.projectmadurai.org/pm_etexts/utf8/pmuni0147.html) - Project Madurai
- [Tamil Virtual Academy](https://www.tamilvu.org/) - Nannool with commentary
- [Open-Tamil](https://github.com/Ezhil-Language-Foundation/open-tamil) - Python Tamil library

## License

MIT License
