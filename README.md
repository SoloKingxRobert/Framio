# Framio

[![Rust Stable](https://img.shields.io/badge/rust-stable-blue.svg)](https://www.rust-lang.org/)
[![Crates.io](https://img.shields.io/crates/v/eframe.svg?label=eframe)](https://crates.io/crates/eframe)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](LICENSE)
[![Build](https://github.com/yourusername/framio/actions/workflows/build.yml/badge.svg)](https://github.com/yourusername/framio/actions)
![Windows Only](https://img.shields.io/badge/platform-Windows-blue?style=for-the-badge&logo=windows)

> A **frameless, Rust-powered Windows shell** for building modern, reusable apps — without the OS’s outdated chrome.

Framio gives developers **full control over window chrome, branding, and workflow integration**.  
Built with [Rust](https://www.rust-lang.org/) and [eframe/egui](https://github.com/emilk/egui), it’s the foundation for creating modern, reusable, and visually consistent desktop applications — free from default title bars and legacy UI patterns.

---

## ✨ Features

- **Frameless Window Chrome** – Replace the OS title bar with your own branded top bar, menus, and controls.
- **Custom Navigation Bar** – Pixel-perfect, branded menu buttons with hover states, dropdowns, and logo integration.
- **Resizable & Draggable** – Full edge-resize support and custom drag zones for moving the window.
- **Theming & Branding** – Centralised dark/light palette and brand accents applied consistently across the UI.
- **Reusable Components** – Modular `egui` widgets that drop cleanly into other Rust projects.
- **Performance-First** – Rust speed, safety, and maintainability with minimal dependencies.

---

## 📦 Dependencies

Framio is built on the latest stable versions:

```toml
[dependencies]
eframe = "0.32.1"
egui   = "0.32.1"
image  = "0.25.6"
```

---

## 📸 Preview

![Framio Preview](Framio.webp)

👉 Click the image to watch the demo video.

---

## 🚀 Getting Started

### Prerequisites
- [Rust](https://www.rust-lang.org/tools/install) (latest stable toolchain)
- Windows 10 or 11 (primary focus)
- Cargo (bundled with Rust; verify with `cargo --version`)

### Clone and Run

```bash
git clone https://github.com/yourusername/framio.git
cd framio
cargo run
```

If your binary name is capitalised in `Cargo.toml`:

```bash
cargo run --bin Framio
```

---

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
- **Top bar** – Reserved space for logo, menus, and actions (e.g., Support).
- **Window actions** – Click targets for minimise / maximise / close and a drag zone mapped to viewport commands.
- **Theme** – Palette constants defined once and applied across widgets for consistent brand visuals.

---

## ☕ Support Framio

If Framio helps you ship faster or inspires your own shell, you can say thanks here:

[![Donate via PayPal](https://img.shields.io/badge/💸-Donate%20via%20PayPal-blue?style=for-the-badge)](https://paypal.me/RobertWhaite)


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
Licensed under the [MIT License](LICENSE).

---

## 🤝 Contributing

We welcome contributions!

1. Fork the repo and create your branch (`git checkout -b feature/amazing-feature`)
2. Commit your changes (`git commit -m 'Add some amazing feature'`)
3. Push to the branch (`git push origin feature/amazing-feature`)
4. Open a Pull Request

**Guidelines:**
- Issues & ideas: open with a focused description and screenshots if visual.
- PR style: small, composable changes with clear commit messages.
- Design alignment: keep components DRY, brand-aware, and consistent with the palette and spacing system.

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
