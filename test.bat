@echo off
:: test.bat - Windows Batch version of test script
setlocal

echo Checking JS for errors...
python utils\check-js

echo Preparing build directory (tmp)...
if exist tmp rd /s /q tmp
meson setup tmp

echo Compiling CSS...
meson compile -C tmp cinnamon_css

echo Build check complete.
echo Skipping system-level file copies (Linux-only: /usr/share/cinnamon/).
