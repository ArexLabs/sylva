#!/bin/bash
# test.windows.sh - Windows-compatible version of test script
# NOTE: Core DE logic relies on Linux system paths. This script focuses on Build/Lint.

set -e

echo "Checking JS for errors..."
python3 utils/check-js

echo "Preparing build directory (tmp)..."
rm -rf tmp
meson setup tmp

echo "Compiling CSS..."
meson compile -C tmp cinnamon_css

echo "Build check complete."
echo "Skipping system-level file copies (Linux-only: /usr/share/cinnamon/)."
