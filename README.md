<h1 align="center">A simple CLI metronome written in Rust. 👋</h1>
<p>
  <img alt="Version" src="https://img.shields.io/badge/version-1.0-blue.svg?cacheSeconds=2592000" />
  <a href="#" target="_blank">
    <img alt="License: MIT" src="https://img.shields.io/badge/License-MIT-yellow.svg" />
  </a>
</p>

A simple command-line metronome application written in Rust that allows users to set and adjust the beats per minute (BPM) in real-time.

### 🏠 [Homepage](https://github.com/nkatshiba/cli-metronome-rust)
## Features

- Set BPM via command-line input
- Increase or decrease BPM with keyboard controls
- Visual beat indication in the terminal
- Audible beat using sound files
## Prerequisites

Before you begin, ensure you have met the following requirements:

- Rust programming language installed
- Crossterm crate for terminal manipulation
- Rodio crate for audio playback
## Install

Clone the repository and build the project using Cargo:

```sh
git clone https://github.com/your-username/metronome-cli.git
cd metronome-cli
cargo build --release
```

## Usage
Run the compiled binary from the terminal:
```sh
./target/release/metronome-cli
```
Follow the on-screen instructions to set the initial BPM and use the following controls:
- `+` or `Arrow Up`: Increase BPM
- `-` or `Arrow Down`: Decrease BPM
- `q`: Quit the application

## Author

👤 **nkatshiba**

* Website: https://www.nkat.se/ | https://www.vimkat.com/
* Github: [@nkatshiba](https://github.com/nkatshiba)
