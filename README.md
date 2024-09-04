
## Viewing the SQLite Database

This project uses an SQLite database. To view and manage the database, we recommend using the SQLite Viewer for VSCode. 

### How to Use:
1. Open this project in [Visual Studio Code](https://code.visualstudio.com/).
2. Install the recommended extensions when prompted (or manually install the SQLite Viewer from the Extensions Marketplace).
3. After installation, open the database file (`.db`) from the Explorer sidebar, and it will be displayed in the SQLite Viewer tab.

# Chess Neural Network with Rust and PyTorch

This project uses the `tch` crate to implement a neural network for chess move prediction. The model is built using PyTorch (LibTorch) in Rust.

## Prerequisites

- [Rust](https://www.rust-lang.org/) (with `cargo`)
- No need to manually install `LibTorch`, it will be automatically downloaded during the build process.
