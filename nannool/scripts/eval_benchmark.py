#!/usr/bin/env python3
"""Evaluate nannool against tamilgeminibenchmark.txt"""

import json
import subprocess
import sys
from pathlib import Path
from collections import Counter

def run_nannool_check(text: str, level: str = "standard") -> tuple[bool, str]:
    """Run nannool check on text, return (has_errors, output)"""
    result = subprocess.run(
        ["./target/release/nannool", "check", "--level", level, "--text", text],
        capture_output=True,
        text=True
    )
    # Check for errors - returncode != 0 means errors found
    # "பிழைகள் இல்லை" means "no errors" so we need to exclude that
    output = result.stdout + result.stderr
    has_errors = result.returncode != 0
    return has_errors, output

def main():
    import argparse
    parser = argparse.ArgumentParser()
    parser.add_argument("--level", default="standard", choices=["classical", "standard", "lenient"])
    parser.add_argument("--quiet", "-q", action="store_true", help="Only show summary")
    args = parser.parse_args()

    benchmark_path = Path("data/tamilgeminibenchmark.txt")
    with open(benchmark_path) as f:
        data = json.load(f)

    sentences = data[0]["sentences"]

    # Counters
    true_positives = 0   # Correctly detected errors
    false_positives = 0  # Flagged correct sentences as errors
    true_negatives = 0   # Correctly passed valid sentences
    false_negatives = 0  # Missed errors

    # Track by error type
    detected_by_type = Counter()
    missed_by_type = Counter()

    print("=" * 80)
    print(f"NANNOOL BENCHMARK EVALUATION - தமிழ் இலக்கண சோதனை (level={args.level})")
    print("=" * 80)
    print(f"Total sentences: {len(sentences)}")
    print()

    for sent in sentences:
        text = sent["text"]
        label = sent["label"]  # "correct" or "incorrect"
        error_type = sent.get("error_type")

        has_errors, output = run_nannool_check(text, args.level)

        if label == "incorrect":
            if has_errors:
                true_positives += 1
                detected_by_type[error_type] += 1
            else:
                false_negatives += 1
                missed_by_type[error_type] += 1
                if not args.quiet:
                    print(f"[MISS] #{sent['id']} ({error_type})")
                    print(f"       {text}")
                    print(f"       Expected: {sent.get('correction', 'N/A')}")
                    print()
        else:  # correct
            if has_errors:
                false_positives += 1
                if not args.quiet:
                    print(f"[FP] #{sent['id']} False positive")
                    print(f"     {text}")
                    print(f"     {output.strip()[:100]}")
                    print()
            else:
                true_negatives += 1

    # Calculate metrics
    total = len(sentences)
    incorrect_count = sum(1 for s in sentences if s["label"] == "incorrect")
    correct_count = total - incorrect_count

    precision = true_positives / (true_positives + false_positives) if (true_positives + false_positives) > 0 else 0
    recall = true_positives / (true_positives + false_negatives) if (true_positives + false_negatives) > 0 else 0
    f1 = 2 * precision * recall / (precision + recall) if (precision + recall) > 0 else 0
    accuracy = (true_positives + true_negatives) / total

    print("=" * 80)
    print("RESULTS - முடிவுகள்")
    print("=" * 80)
    print(f"Total sentences:     {total}")
    print(f"  - Incorrect:       {incorrect_count}")
    print(f"  - Correct:         {correct_count}")
    print()
    print(f"True Positives:      {true_positives} (correctly detected errors)")
    print(f"False Positives:     {false_positives} (flagged valid as error)")
    print(f"True Negatives:      {true_negatives} (correctly passed valid)")
    print(f"False Negatives:     {false_negatives} (missed errors)")
    print()
    print(f"Precision:           {precision:.1%}")
    print(f"Recall:              {recall:.1%}")
    print(f"F1 Score:            {f1:.1%}")
    print(f"Accuracy:            {accuracy:.1%}")
    print()

    print("DETECTION BY ERROR TYPE:")
    all_types = set(detected_by_type.keys()) | set(missed_by_type.keys())
    for etype in sorted(all_types):
        detected = detected_by_type[etype]
        missed = missed_by_type[etype]
        total_t = detected + missed
        rate = detected / total_t * 100 if total_t > 0 else 0
        print(f"  {etype:40} {detected}/{total_t} ({rate:.0f}%)")

if __name__ == "__main__":
    main()
