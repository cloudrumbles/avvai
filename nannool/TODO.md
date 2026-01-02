# Nannool Project TODO

## Current Tasks

### Core Rules Engine
- [x] Refactor `Transformation` enum to support compound operations (e.g., "ன் + த → ன்ற")
- [x] Fix rule overlap bug: `kutriyalukaram-206` vs `vallinam-miguthal-165`
- [x] Implement `check_satisfied` for all transformation types
- [ ] Support for "optional" rules (விகற்பம்) with `Info`/`Hint` severity
- [x] Implement morphological context awareness (using FST feature tags)
- [x] Implement Shallow Dependency Parser for relationship detection (அல்வழி vs வேற்றுமை)

### Data & Rules
- [x] Formalize all vowel-ending rules (verses 151-203)
- [x] Formalize all consonant-ending rules (verses 204-239)
- [x] Formalize all case-marker rules (verses 240-257)
- [x] Add unit tests for each formalized rule
- [ ] Integrate Lexicon/WordNet for semantic categories (Tree names, Directions, திணை)
- [ ] Formalize remaining Nannool verses and edge cases

### Checker & CLI
- [ ] Improve diagnostic messages with Tamil grammatical terms
- [ ] Add `explain` command to CLI to show Nannool verses and examples
- [ ] Implement Language Server Protocol (LSP) for real-time IDE integration
- [ ] Add web-based playground for sandhi checking

### Morphological Analyzer
- [x] Integration with `thamizhi-morph` for better word class detection
- [ ] Support for common inflections

## Completed Tasks
- [x] Project structure and crate setup
- [x] Basic rule engine with priority-based matching
- [x] TOML-based rule loading
- [x] Basic Tamil word tokenizer
- [x] Initial set of core sandhi rules (162, 165, 204, 205, 206)
- [x] Basic CLI for checking text files
- [x] SARIF diagnostic format (initial implementation)
