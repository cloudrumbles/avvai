import subprocess
import time
import statistics
import json
import os

# Paths should be relative to nannool root
PROJECT_ROOT = "/home/shah/workspace/avvai/nannool"
NANNOOL_BIN = "./target/release/nannool"
DATA_DIR = "data/tests"

TEST_FILES = {
    "small": (os.path.join(DATA_DIR, "clean_sample.txt"), 22),
    "medium": (os.path.join(DATA_DIR, "medium.txt"), 440),
    "large": (os.path.join(DATA_DIR, "large.txt"), 4400),
}

WORDS = ["வந்தான்", "மரம்", "பூனை", "ஓடினான்", "பெரிய"]
PAIRS = [("பாட்டு", "பாடினான்"), ("வீடு", "கட்டினான்"), ("தமிழ்", "மொழி")]
RULES = ["vallinam-miguthal-165", "kutriyalukaram-206"]

def run_cmd(args):
    start = time.perf_counter()
    result = subprocess.run([NANNOOL_BIN] + args, capture_output=True, text=True, cwd=PROJECT_ROOT)
    end = time.perf_counter()
    if result.returncode != 0 and result.returncode != 1:
        print(f"Error running {args}: {result.stderr}")
    return end - start

def benchmark_command(name, args, iterations=10, word_count=None):
    latencies = []
    # Warmup
    run_cmd(args)
    
    for _ in range(iterations):
        latencies.append(run_cmd(args))
    
    avg = statistics.mean(latencies)
    std_dev = statistics.stdev(latencies) if len(latencies) > 1 else 0
    
    res = {
        "name": name,
        "avg_ms": avg * 1000,
        "std_dev_ms": std_dev * 1000,
        "min_ms": min(latencies) * 1000,
        "max_ms": max(latencies) * 1000
    }
    
    if word_count:
        res["words_per_sec"] = word_count / avg
        
    return res

def main():
    results = []

    print(f"Starting benchmarks in {PROJECT_ROOT}...")

    print("Benchmarking 'check' command...")
    for size, (path, count) in TEST_FILES.items():
        print(f"  Checking {size} file ({count} words)...")
        results.append(benchmark_command(f"check_file_{size}", ["check", path], word_count=count))

    print("Benchmarking 'check --morph' command...")
    results.append(benchmark_command("check_file_small_morph", ["check", "--morph", TEST_FILES["small"][0]], iterations=3, word_count=TEST_FILES["small"][1]))
    results.append(benchmark_command("check_file_medium_morph", ["check", "--morph", TEST_FILES["medium"][0]], iterations=3, word_count=TEST_FILES["medium"][1]))

    print("Benchmarking 'check --text' command...")
    results.append(benchmark_command("check_text_short", ["check", "--text", "பாட்டு பாடினான்"]))

    print("Benchmarking 'analyze' command...")
    for word in WORDS:
        print(f"  Analyzing '{word}'...")
        results.append(benchmark_command(f"analyze_{word}", ["analyze", word]))

    print("Benchmarking 'pair' command...")
    for w1, w2 in PAIRS:
        print(f"  Checking pair '{w1} {w2}'...")
        results.append(benchmark_command(f"pair_{w1}_{w2}", ["pair", w1, w2]))

    print("Benchmarking 'rules' command...")
    results.append(benchmark_command("rules_list", ["rules"]))

    print("Benchmarking 'explain' command...")
    for rule in RULES:
        print(f"  Explaining '{rule}'...")
        results.append(benchmark_command(f"explain_{rule}", ["explain", rule]))

    print("Benchmarking 'paradigm' command...")
    results.append(benchmark_command("paradigm_noun_மரம்", ["paradigm", "மரம்", "--pos", "noun"], iterations=3))

    print("Benchmarking 'inflect' command...")
    results.append(benchmark_command("inflect_noun_மரம்_acc", ["inflect", "மரம்", "--case", "acc"], iterations=3))

    print("\n--- Benchmark Results ---")
    print(f"{'Command':<30} | {'Avg (ms)':>10} | {'WPS':>10} | {'StdDev':>10} | {'Min/Max (ms)':>20}")
    print("-" * 100)
    for r in results:
        wps = f"{r['words_per_sec']:>10.0f}" if "words_per_sec" in r else f"{'N/A':>10}"
        min_max = f"{r['min_ms']:>8.2f}/{r['max_ms']:<8.2f}"
        print(f"{r['name']:<30} | {r['avg_ms']:>10.2f} | {wps} | {r['std_dev_ms']:>10.2f} | {min_max}")

    # Output to JSON
    with open("benchmark_results.json", "w") as f:
        json.dump(results, f, indent=2)

if __name__ == "__main__":
    main()
