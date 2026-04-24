# Sylva

Sylva is a modern Linux distribution desktop environment derived from Cinnamon. It aims to deliver a stable, reliable, and powerful user experience with a focus on modern Linux technologies like Wayland and declarative configuration.

## Mission

- **Simple for beginners.**
- **Powerful for experts.**
- **Never restrictive.**

---

## Technical Stack

Sylva is built using a multi-language architecture:

- **C++**: Core compositor integration (Muffin/Clutter), Shell Toolkit (ST), and performance-critical components.
- **JavaScript (GJS)**: User Interface logic, panel management, applets, and extension system.
- **Rust**: Declarative system configuration engine (`sylva-config`), `sylva-cli`, and modern services.
- **Python**: Automation tools and developer utilities.
- **Shell**: Session initialization and installation wrappers.
- **Batch**: Windows development helpers.

---

## Directory Structure

### Core Components

- **`src/`**: C++ source code. Contains the Shell Toolkit (ST), compositor plugins, and low-level managers.
- **`js/`**: JavaScript UI logic. The heart of the shell interface, including the panel, menu, and window management UI.
- **`data/`**: Configuration schemas (GSettings), theme data, and desktop entry files.

### Tools & Utilities

- **`tools/`**: Development and maintenance utilities.
  - `sylva-config/`: The Rust-based declarative configuration engine.
- **`utils/`**: Helper scripts for development and session management.
- **`files/`**: System-side configurations, portal definitions, and default scripts.

### Build & Packaging

- **`debian/`**: Debian/Ubuntu packaging metadata and control files.
- **`meson.build`**: Entry point for the Meson build system.
- **`meson_options.txt`**: Build-time configuration toggles (e.g., Wayland support).

### Documentation & Testing

- **`docs/`**: Technical documentation and architecture overviews.
- **`man/`**: Manual pages for Sylva components.
- **`tests/`** & **`test/`**: Automated test suites for core logic and UI components.

### Other

- **`po/`**: Translation files (Gettext).
- **`calendar-server/`**: Specialized server for calendar integration.
- **`python3/`**: Internal Python library modules for Sylva tools.

---

## Getting Started

### Build Requirements

- Meson 0.56+
- Ninja
- GJS (Gjs-1.0)
- Muffin (Mutter fork)
- Rust (for `sylva-config`)

### Building

```bash
meson setup builddir
ninja -C builddir install
```
