benchmark_file=""
if [[ -n "$1" ]]; then
    if [[ "$1" =~ ^.*benchmark.py$ ]]; then
        benchmark_file="$1"
    else
        benchmark_file="$1"/benchmark.py
    fi
else
    benchmark_file="$PWD"/benchmark.py
fi

if [[ ! -f $benchmark_file ]]; then
    echo "Benchmark test file not found : '$benchmark_file'"
    exit
fi

echo "Benchmark test file found : '$benchmark_file'"

output_file="$(dirname "$benchmark_file")"/benchmark.txt
echo "saving benchmark output in '$output_file'"
exec > >(tee "$output_file") 2>&1

# can be one of: 'min', 'max', 'mean', 'stddev', 'name', 'fullname'
sort_by="mean"
# pytest-benchmark default: 'min, max, mean, stddev, median, iqr,
# outliers, ops, rounds, iterations'
columns="median, iqr, mean, stddev, min, max, outliers, ops, rounds, iterations"

echo "--- starting benchmark script ---"
uv run pytest --benchmark-only --benchmark-sort="$sort_by" --benchmark-columns="$columns" "$benchmark_file"
echo "--- benchmark script finished ---"
