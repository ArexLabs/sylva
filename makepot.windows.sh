#!/bin/bash
# makepot.windows.sh - Windows-compatible version of makepot
set -e
POT="cinnamon.pot"

# Check for xgettext
if ! command -v xgettext &> /dev/null; then
    echo "Error: xgettext not found. Install gettext utilities."
    exit 1
fi

echo "Generating POT file..."
xgettext -o $POT --language=C --keyword=_ --keyword=N_ src/*.c src/*/*.c

echo "Collecting UI files..."
find files/usr/share/cinnamon -name "*.ui" -exec xgettext -o $POT --join-existing --language=glade --from-code=UTF-8 {} +

echo "Collecting JS files..."
xgettext -o $POT --join-existing --language=javascript -cTranslators --keyword=_ --keyword=N_ --from-code=UTF-8 js/*/*.js files/usr/share/cinnamon/*/*/*.js

echo "Collecting Python files..."
xgettext -o $POT --join-existing --language=python -c --keyword=_ generate_additional_files.py files/usr/share/cinnamon/*/*.py files/usr/share/cinnamon/*/*/*.py files/usr/bin/*

echo "Running xlet-makepot..."
# Note: assuming python3 is in PATH and handles relative paths
python3 files/usr/bin/cinnamon-xlet-makepot -o $POT -p -m -j files/usr/share/cinnamon/applets/
python3 files/usr/bin/cinnamon-xlet-makepot -o $POT -p -m -j files/usr/share/cinnamon/desklets/

echo "Done: $POT"
