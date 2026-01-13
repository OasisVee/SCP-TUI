# SCiPIndex-TUI

A terminal-based user interface (TUI) for reading SCP Foundation entries. Written in Rust for speed and comfort.

## Features

- **Read SCP Entries:** Fetch and read any SCP entry by number.
- **Random SCP:** Discover new entries with a random picker.
- **Offline Storage:** Entries are saved locally (`~/.local/share/scipindex-tui/saved_entries` on Linux/macOS) for instant access next time.
- **Vim-like Navigation:** Scroll with `j` and `k`, jump to top/bottom with `g`/`G`.
- **Search:** Quickly jump to specific entries.

## Installation

### Cargo

```bash
cargo install SCiPIndex-TUI
```

### From Source

Ensure you have [Rust and Cargo installed](https://rustup.rs/).

1. Clone the repository (if you haven't already):

   ```bash
   git clone https://github.com/OasisVee/SCiPIndex-TUI.git
   cd SCiPIndex-TUI
   ```

2. Build the release binary:

   ```bash
   cargo build --release
   ```

3. Move the binary to your PATH (optional):

   ```bash
   sudo cp target/release/scipindex /usr/local/bin/
   ```

## Usage

Run the application:

```bash
scipindex
```

### Keybindings

| Key          | Action                                                            |
| :----------- | :---------------------------------------------------------------- |
| `i` or `/`   | **Input Mode:** Type an SCP number (e.g., `173`) and press Enter. |
| `r`          | **Random:** Load a random SCP entry.                              |
| `j` / `Down` | Scroll down one line.                                             |
| `k` / `Up`   | Scroll up one line.                                               |
| `Ctrl+d`     | Scroll down 10 lines.                                             |
| `Ctrl+u`     | Scroll up 10 lines.                                               |
| `g`          | Jump to the top of the entry.                                     |
| `G`          | Jump to the bottom of the entry.                                  |
| `q`          | Quit the application.                                             |

## License

This software is released under the MIT License. See the [LICENSE](LICENSE) file for details.

## SCP Content Attribution

All SCP Foundation content displayed by this application is sourced from the [SCP Wiki](https://scp-wiki.wikidot.com/) and is licensed under the [Creative Commons Attribution-ShareAlike 3.0 Unported License (CC BY-SA 3.0)](https://creativecommons.org/licenses/by-sa/3.0/).

This application is a reader tool and claims no ownership over SCP Foundation articles. All credit for SCP content goes to the original authors on the SCP Wiki.

**Attribution:** Content from the SCP Foundation, including all SCP articles, is © their respective authors and is used under CC BY-SA 3.0.
