<div align="center">
<h1>JustBot Chess Engine</h1>
  
[![License: GPL-3.0](https://img.shields.io/github/license/HasanFakih21/JustBot?style=flat-square&color=blue)](https://www.gnu.org/licenses/gpl-3.0.en.html)
[![GitHub Release](https://img.shields.io/github/v/release/HasanFakih21/JustBot?include_prereleases&style=flat-square)](https://github.com/HasanFakih21/JustBot/releases)

</div>


JustBot is my first attempt at creating a UCI chess engine with magic bitboards without any agentic or LLM assisted coding.

## Building the project
To build the project, you need a working installation of Rust and Cargo, once the repository is cloned, you can run:

```bash
cargo build --release
```

## Features
- Basic UCI compatibility
- Transposition Tables with Hash Moves
- Alpha-Beta search
- Material and Piece Square evaluation
- Quiescence Search
- Iterative Deepening
- Time management
- Principal Variation Search

## Acknowledgments
- [Chess Programming Wiki](https://www.chessprogramming.org/Main_Page)
- [Maksim Korzh](https://www.youtube.com/watch?v=QUNP-UjujBM&list=PLmN0neTso3Jxh8ZIylk74JpwfiWNI76Cs) for helpful introductory videos, and where my magic numbers are from
- [Reckless](https://github.com/codedeliveryservice/Reckless) and [Stockfish](https://github.com/official-stockfish/stockfish)
