
# moonsong-rs 🎸

A Rust-powered MIDI reader specifically designed for YARG (Yet Another Rhythm Game) and Clone Hero chart schemas.

## Overview

`moonsong-rs` is a specialized MIDI parser that understands and processes rhythm game chart formats, particularly those used in YARG and Clone Hero. It can parse various track types including guitar, bass, drums, and vocals across multiple difficulty levels.

## Features

- 🎵 Parses MIDI files for rhythm game charts
- 🎸 Supports multiple instrument tracks:
  - Guitar (including GHL and Pro variants)
  - Bass
  - Drums
  - Vocals and Harmonies
  - Keys
  - Rhythm
- 🔄 Handles tempo changes and timing events
- 📊 Multiple difficulty support (Easy, Medium, Hard, Expert)
- 🕒 Accurate song length calculation
- 📝 Event tracking and parsing

## Supported Track Types

- [x] Standard Instruments
  - [x] Guitar
  - [x] Bass
  - [x] Drums
  - [x] Keys
  - [x] Rhythm
  - [x] Guitar Co-op

- [ ] Pro Instruments
  - [ ] Pro Guitar (17 & 22 Fret)
  - [ ] Pro Bass (17 & 22 Fret)
  - [ ] Pro Keys

- [ ] Guitar Hero Live
  - [ ] GHL Guitar
  - [ ] GHL Bass
  - [ ] GHL Rhythm
  - [ ] GHL Guitar Co-op

- [ ] Vocals
  - [ ] Lead Vocals
  - [ ] Harmony 1-3

## Usage

```rust
use moonsong::Moonsong;

fn main() {
    // TODO: Add usage instructions
}
```

## Installation

TODO: Add installation instructions


## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.


## Acknowledgments

- YARG Project
- Clone Hero Community
- Nathanator with his amazing documentation [here](https://github.com/TheNathannator/GuitarGame_ChartFormats)
