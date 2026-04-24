use crossterm::{
    event::{read, Event, KeyCode, KeyModifiers},
    terminal::{disable_raw_mode, enable_raw_mode},
};
use std::env;
use std::io::{stdout, Write};
use std::process::{exit, Command};
use zeroize::Zeroize;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("Usage: cargo run -- build-iso");
        exit(1);
    }

    if args[1] == "build-iso" {
        build_iso();
    } else {
        println!("Unknown command.");
        exit(1);
    }
}

fn detect_nerd_font() -> bool {
    let term_program = env::var("TERM_PROGRAM").unwrap_or_default();
    let kitty = env::var("KITTY_WINDOW_ID").unwrap_or_default();
    let alacritty = env::var("ALACRITTY_LOG").unwrap_or_default();
    let wezterm = env::var("WEZTERM_PANE").unwrap_or_default();

    env::var("NERD_FONT").is_ok()
        || !kitty.is_empty()
        || !alacritty.is_empty()
        || !wezterm.is_empty()
        || term_program == "WezTerm"
}

use crossterm::event::KeyEventKind;

fn prompt_password() -> String {
    let nerd = detect_nerd_font();
    let prompt = if nerd {
        "\r\n\x1b[36m\x1b[0m Enter sudo password: "
    } else {
        "\r\nEnter sudo password: "
    };

    print!("{}", prompt);
    stdout().flush().unwrap();

    let mut pwd = String::new();
    if enable_raw_mode().is_err() {
        println!("\r\nFailed to enable secure password input mode.");
        exit(1);
    }

    loop {
        match read() {
            Ok(Event::Key(event)) if event.kind == KeyEventKind::Press => match event.code {
                KeyCode::Enter => {
                    print!("\r\n");
                    stdout().flush().unwrap();
                    break;
                }
                KeyCode::Char('c') if event.modifiers.contains(KeyModifiers::CONTROL) => {
                    disable_raw_mode().unwrap();
                    print!("\r\nProcess aborted.\r\n");
                    stdout().flush().unwrap();
                    pwd.zeroize();
                    exit(1);
                }
                KeyCode::Backspace => {
                    if !pwd.is_empty() {
                        pwd.pop();
                        print!("\x08 \x08");
                        stdout().flush().unwrap();
                    }
                }
                KeyCode::Char(c) => {
                    pwd.push(c);
                    print!("*");
                    stdout().flush().unwrap();
                }
                _ => {}
            },
            Ok(_) => {} // Ignore mouse events, focus changes, release events, etc.
            Err(_) => {
                // Prevent infinite busy loop if terminal is flooded with OS read errors
                std::thread::sleep(std::time::Duration::from_millis(10));
            }
        }
    }

    disable_raw_mode().unwrap();
    pwd
}

fn build_iso() {
    let mut pwd = prompt_password();

    println!("Verifying dependencies...");
    check_deps(&pwd);
    println!("Building ISO...");
    build(&pwd);
    println!("ISO complete.");

    pwd.zeroize();
}

#[cfg(target_os = "windows")]
fn run_sudo_bash(pwd: &str, cmd: &str) {
    // Embed the password into the bash command itself so sudo
    // reads it from the pipe immediately without WSL startup delay.
    let full_cmd = format!("printf '%s\\n' '{}' | {}", pwd.replace('\'', "'\\''"), cmd);
    let status = Command::new("wsl")
        .arg("-e")
        .arg("bash")
        .arg("-c")
        .arg(&full_cmd)
        .status()
        .unwrap();
    if !status.success() {
        println!("Command failed");
        exit(1);
    }
}

#[cfg(target_os = "linux")]
fn run_sudo_bash(pwd: &str, cmd: &str) {
    let full_cmd = format!("printf '%s\\n' '{}' | {}", pwd.replace('\'', "'\\''"), cmd);
    let status = Command::new("bash")
        .arg("-c")
        .arg(&full_cmd)
        .status()
        .unwrap();
    if !status.success() {
        println!("Command failed");
        exit(1);
    }
}

fn check_deps(pwd: &str) {
    #[cfg(target_os = "windows")]
    {
        println!("Windows detected. Checking WSL status...");
        let wsl_check = Command::new("wsl").arg("--status").status();
        if wsl_check.is_err() {
            println!("WSL missing. Installing WSL...");
            Command::new("wsl").arg("--install").status().unwrap();
            println!("WSL installation complete. Reboot required.");
            exit(0);
        }
        println!("Installing Linux dependencies inside WSL...");
    }
    #[cfg(target_os = "linux")]
    {
        println!("Linux detected. Installing dependencies...");
    }

    run_sudo_bash(
        pwd,
        "sudo -p '' -S bash -c 'apt-get update && apt-get install -y live-build squashfs-tools xorriso debootstrap ca-certificates'",
    );
}

#[cfg(target_os = "windows")]
fn build(pwd: &str) {
    let cmd = format!(
        "ORIG_DIR=\"$(pwd)\" && mkdir -p {0} && cd {0} && \
         sudo -p '' -S bash -c \"lb clean --purge && rm -rf config/ && \
         lb config --distribution noble --archive-areas 'main restricted universe multiverse' && \
         env PATH=/usr/local/sbin:/usr/local/bin:/usr/sbin:/usr/bin:/sbin:/bin lb build\" && \
         cp *.iso \"$ORIG_DIR/\" 2>/dev/null || true",
        "~/.sylva_build_workspace"
    );
    run_sudo_bash(pwd, &cmd);
}

#[cfg(target_os = "linux")]
fn build(pwd: &str) {
    run_sudo_bash(
        pwd,
        concat!(
            "sudo -p '' -S bash -c \"",
            "lb clean --purge && ",
            "rm -rf config/ && ",
            "lb config --distribution noble --archive-areas 'main restricted universe multiverse' && ",
            "env PATH=/usr/local/sbin:/usr/local/bin:/usr/sbin:/usr/bin:/sbin:/bin lb build",
            "\""
        ),
    );
}
