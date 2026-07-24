# 🍅 Pomotui

> ⚠️ **Learning Project Disclaimer**  
> This project was built primarily for **learning Rust** and exploring Terminal User Interface (TUI) development. It is an experimental and educational project to practice Rust concepts, memory safety, pattern matching, state management, and async/event loops in terminal applications.

---

## 📖 About The Project

**Pomotui** is a lightweight, customizable Pomodoro technique timer application that runs directly in your terminal. Built with [Ratatui](https://github.com/ratatui/ratatui) and [Crossterm](https://github.com/crossterm-rs/crossterm), it provides an interactive terminal dashboard to help track focus sessions and boost productivity.

---

## ✨ Features

- 🍅 **Interactive Pomodoro Timer**: Visual countdown display powered by `tui-big-text`.
- 📊 **Statistics Dashboard**: Track completed sessions and time spent focusing.
- ⚙️ **Customizable Settings**: Adjust focus and break intervals to fit your workflow.
- 🦀 **Rust Educational Notes**: Includes a dedicated [`rust_learnings.md`](./rust_learnings.md) log documenting language concepts encountered during development.

---

## 🧰 Tech Stack & Dependencies

- **Language:** [Rust](https://www.rust-lang.org/) (2024 Edition)
- **TUI Framework:** [`ratatui`](https://crates.io/crates/ratatui)
- **Terminal Backend:** [`crossterm`](https://crates.io/crates/crossterm)
- **UI Enhancements:** [`tui-big-text`](https://crates.io/crates/tui-big-text)
- **Date & Time:** [`chrono`](https://crates.io/crates/chrono)

---

## 🚀 Getting Started

### Prerequisites

Make sure you have the Rust toolchain (cargo, rustc) installed:

### Installation & Execution

1. **Clone the repository:**
   ```bash
   git clone https://github.com/your-username/pomotui.git
   cd pomotui
   ```

2. **Run in development mode:**
   ```bash
   cargo run
   ```

3. **Build optimized release binary:**
   ```bash
   cargo build --release
   ```

---

## 📚 Rust Learnings & Takeaways

As a project created for learning Rust, key language concepts explored throughout development include:

- **Ownership & Borrowing:** Managing app state and rendering frames without unnecessary cloning.
- **Closures & HOFs:** Functional patterns used in Ratatui drawing routines (`terminal.draw(|frame| ...)`).
- **Error Handling:** Idiomatic error propagation with `Result<T, E>` and the `?` operator.
- **Enums & Pattern Matching:** Representing UI tabs (`Timer`, `Stats`, `Settings`) and timer states (`Paused`, `Running`, `Finished`).

---

## 📄 License

This project is open-source and intended for educational and personal learning purposes.
