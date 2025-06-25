#!/bin/bash

# Setup FlameGraph tools if they don't exist
FLAMEGRAPH_DIR="./flamegraph_tools"
if [ ! -d "$FLAMEGRAPH_DIR" ]; then
    echo "FlameGraph tools not found. Setting up..."
    mkdir -p "$FLAMEGRAPH_DIR"
    git clone --depth 1 https://github.com/brendangregg/FlameGraph.git "$FLAMEGRAPH_DIR"
    echo "FlameGraph tools installed."
fi

# Run perf record on the specific test
echo "Running perf record..."
RUSTFLAGS='-g' cargo build --release --tests

# Record performance data using the specific perft_tests binary
echo "Recording performance data..."
sudo perf record -g --call-graph dwarf \
    ./target/release/deps/perft_tests-18017576f8847c4f \
    --exact perft_tests::test_perft_depth_6

# Generate perf report
echo "Generating perf report..."
sudo perf report -g 'graph,0.5,caller' > perf_report.txt

# Extract perf data to a format compatible with multiple visualization tools
echo "Processing performance data..."
sudo perf script > perf_output.stacks

# Generate original SVG flamegraph (keeping for comparison)
echo "Generating original SVG flamegraph..."
cat perf_output.stacks | \
    "$FLAMEGRAPH_DIR/stackcollapse-perf.pl" | \
    "$FLAMEGRAPH_DIR/flamegraph.pl" \
        --width 1800 \
        --height 24 \
        --fontsize 11 \
        --minwidth 0.5 \
        --title "Chess Perft Flame Graph" > chess_perft_flame_original.svg

# Generate Inferno flamegraphs (better for Rust code)
echo "Generating Inferno flamegraphs (better for Rust code)..."

# Create standard SVG flamegraph with Inferno using improved settings
cat perf_output.stacks | \
    inferno-collapse-perf | \
    inferno-flamegraph \
        --width 2400 \
        --height 28 \
        --fontsize 10 \
        --minwidth 0.1 \
        --nametype "Function:" \
        --countname "samples" \
        --title "Chess Perft Performance (Inferno)" \
        --colors "rust" \
        --truncate-text-right \
        > chess_perft_inferno.svg

# Try an alternative visualization for more details
echo "Generating alternative detailed flamegraph..."
cat perf_output.stacks | \
    "$FLAMEGRAPH_DIR/stackcollapse-perf.pl" | \
    "$FLAMEGRAPH_DIR/flamegraph.pl" \
        --width 2400 \
        --height 32 \
        --fontsize 8 \
        --minwidth 0.2 \
        --title "Chess Perft Detailed Flame Graph" \
        --colors java \
        > chess_perft_detailed.svg

# Convert SVG to PNG (for easy sharing)
echo "Converting SVG to PNG..."
rsvg-convert -o chess_perft_inferno.png chess_perft_inferno.svg

sudo chown dgrant:dgrant perf.data
sudo chown dgrant:dgrant perf_output.stacks

echo "Done! Results available in:"
echo "- perf_report.txt (text-based report)"
echo "- chess_perft_flame_original.svg (original flamegraph)"
echo "- chess_perft_inferno.svg (improved flamegraph with better text rendering)"
echo "- chess_perft_detailed.svg (detailed flamegraph with smaller font for more text)"
echo "- chess_perft_inferno.png (PNG version of improved flamegraph)"
echo ""
echo "TIP: To see full function names, try the following:"
echo "  1. Open the SVG files directly in a browser - they're often interactive"
echo "  2. Use your browser's zoom feature to zoom in on specific areas"
echo "  3. Review the detailed text report in perf_report.txt"
