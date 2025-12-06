# Advent of Code 2025 — Rust

[![Rust](https://img.shields.io/badge/Rust-orange?logo=rust&logoColor=white)](https://www.rust-lang.org)
[![Stars](https://img.shields.io/badge/AoC%20Stars-★%2012/24-green)](https://adventofcode.com/2025)
[![License](https://img.shields.io/badge/License-MIT-blue.svg)](LICENSE)

A collection of my solutions to **Advent of Code 2025**, written in **Rust**.  
This project serves as a practical way for me to **learn Rust deeply** by solving progressively harder problems using increasingly idiomatic patterns.

Each day follows a shared interface (`Day` trait) and a consistent file structure.

---

## 🚀 Running

Run a day:

```bash
cargo run -- <day>
```

Run a specific part:

```bash
cargo run -- <day> <part>
```

Run all tests:

```bash
cargo test
```

---

## 📁 Project Structure

```tree
aoc-2025/
├── inputs/
│   ├── day01.txt
│   ├── day02.txt
│   └── ...
├── src/
│   ├── main.rs
│   ├── lib.rs        # Day trait + generic runner
│   └── days/
│       ├── mod.rs
│       ├── day01.rs
│       ├── day02.rs
│       └── ...
```

- `lib.rs`: defines the `Day` trait and shared utilities  
- `main.rs`: command-line entrypoint  
- `days/dayXX.rs`: each day's solution  

---

## 🎯 Goals

- Learn **idiomatic Rust**
- Practice:
  - ownership & borrowing  
  - lifetimes (when needed)  
  - iterators & combinators  
  - pattern matching  
  - clean module structures  
  - error handling  
  - testing & benchmarks  
- Solve the puzzles in a clean, readable way  
- Refactor code as Rust knowledge improves  

---

## ⭐ Progress (12/24)

| Day | Part 1 | Part 2 | Notes |
|-----|--------|--------|--------|
| 01  | ⭐      | ⭐      |        |
| 02  | ⭐      | ⭐      |        |
| 03  | ⭐      | ⭐      |        |
| 04  | ⭐      | ⭐      |        |
| 05  | ⭐      | ⭐      |        |
| 06  | ⭐      | ⭐      |        |
| 07  | ❌      | ❌      |        |
| 08  | ❌      | ❌      |        |
| 09  | ❌      | ❌      |        |
| 10  | ❌      | ❌      |        |
| 11  | ❌      | ❌      |        |
| 12  | ❌      | ❌      |        |


---

## 📝 Disclaimer

This repository is part of my Rust learning journey.  
Expect refactoring, experiments, and increasingly idiomatic solutions as I progress.
