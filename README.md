# Framio

[![Rust Stable](https://img.shields.io/badge/rust-stable-blue.svg)](https://www.rust-lang.org/)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](LICENSE)
![Windows Only](https://img.shields.io/badge/platform-Windows-blue?logo=windows)

---

Framio is a frameless Windows shell built in Rust.  
Think of it as a starting point for modern apps without the clunky default title bars or outdated UI.  
You get full control of the window — the top bar, menus, branding, resizing, and theming — all built with [eframe/egui](https://github.com/emilk/egui).

---

## Dependencies

Framio is built on the latest stable versions.

```toml
[dependencies]
eframe = "0.32.1"
egui   = "0.32.1"
image  = "0.25.6"
```

## 📸 Preview

![Framio Preview](Framio.webp)

## 🚀 Getting Started

### Prerequisites

You need Rust (latest stable toolchain), Windows 10 or 11, and Cargo (bundled with Rust — check with `cargo --version`).

### Clone and Run

```bash
git clone https://github.com/SoloKingxRobert/Framio.git
cd Framio
cargo run
```

Or run the specific binary:

```bash
cargo run --bin framio
```


## 🧩 Project Structure

```
src
 ├─ Assets/
 │   └─ Logo/
 │       └─ Framio.png
 ├─ core/
 │   └─ window/
 │       ├─ layout/
 │       │   ├─ mod.rs
 │       │   └─ chrome.rs
 │       ├─ drag.rs
 │       ├─ menu.rs
 │       ├─ resize.rs
 │       └─ shell.rs
 ├─ ui/
 │   └─ theme/
 │       ├─ dark/
 │       │   └─ mod.rs
 │       ├─ colors.rs
 │       ├─ spacing.rs
 │       └─ typography.rs
 ├─ main.rs
target/
.gitignore
Cargo.lock
Cargo.toml
Framio.webp
README.md
```

---

## 🛠 Configuration

- **Frameless behaviour** – OS chrome disabled; you control title area, buttons, and hit-testing.
- **Top bar** – Reserved space for logo, menus, and actions (e.g. Support).
- **Window actions** – Click targets for minimise / maximise / close and a drag zone mapped to viewport commands.
- **Theme** – Palette constants defined once and applied across widgets for consistent brand visuals.

---

## 💡 Ideas & Experiments

Some concepts I’ve explored or may revisit in the future:
- Branded top bar kit
- Edge resize polish
- Palette presets
- Component library
- Cross‑platform pass

---

## ❓ FAQ

**Q: Why frameless?**  
A: Control. Own your brand, layout, and interactions instead of inheriting OS chrome constraints.

**Q: Can I use this in my own app?**  
A: Absolutely. Framio is designed as a reusable scaffold. Fork, extend, or integrate modules as needed.

---

## 📝 License
Copyright © 2025 Robert Whaite  
Licensed under the [MIT Licence](LICENSE).

---

## 🙏 Acknowledgements

- [Rust](https://www.rust-lang.org/) — Safe & blazing fast systems programming.
- [egui](https://github.com/emilk/egui) — Immediate mode GUI in Rust.
- [eframe](https://github.com/emilk/egui/tree/master/crates/eframe) — The Rust framework powering Framio.

---

## 🔗 Links

- [Rust](https://www.rust-lang.org)
- [eframe/egui](https://github.com/emilk/egui)
- [Cargo](https://doc.rust-lang.org/cargo/)  
