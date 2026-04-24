@echo off
:: makepot.bat - Windows Batch version of makepot
set POT=cinnamon.pot

where xgettext >nul 2>nul
if %ERRORLEVEL% neq 0 (
    echo Error: xgettext not found. Please install gettext for Windows.
    exit /b 1
)

echo Generating POT file from C sources...
xgettext -o %POT% --language=C --keyword=_ --keyword=N_ src\*.c src\*\*.c

echo Collecting UI files...
for /r files\usr\share\cinnamon %%f in (*.ui) do (
    xgettext -o %POT% --join-existing --language=glade --from-code=UTF-8 "%%f"
)

echo Collecting JS files...
xgettext -o %POT% --join-existing --language=javascript -cTranslators --keyword=_ --keyword=N_ --from-code=UTF-8 js\*\*.js files\usr\share\cinnamon\*\*\*\*.js

echo Collecting Python files...
xgettext -o %POT% --join-existing --language=python -c --keyword=_ generate_additional_files.py files\usr\share\cinnamon\*\*.py files\usr\bin\*

echo Running xlet-makepot...
python files\usr\bin\cinnamon-xlet-makepot -o %POT% -p -m -j files\usr\share\cinnamon\applets\
python files\usr\bin\cinnamon-xlet-makepot -o %POT% -p -m -j files\usr\share\cinnamon\desklets\

echo Done: %POT%
