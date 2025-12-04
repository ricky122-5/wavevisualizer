# WaveViz

A real-time music visualizer for macOS that displays waveforms, album art, and track info right in your terminal.
<img width="1512" height="947" alt="Screenshot 2025-12-03 at 10 55 05 PM" src="https://github.com/user-attachments/assets/d84f9b1e-6dce-4b86-809e-9f0101ba3efc" />


## What is this?

MusicDash captures audio playing on your Mac and visualizes it with a live waveform display. It also pulls in what's currently playing from Apple Music/Music app and shows the album art, song info, and playback progress, all in your terminal with a clean, colorful interface.

## Features

- **Real-time waveform visualization** - See your music as it plays with smooth 60 FPS rendering
- **Album art display** - Full-color album covers rendered in the terminal
- **Track info & progress** - Current song, artist, and playback position
- **Dynamic colors** - Automatically extracts colors from album art for the UI theme
- **Low latency** - Sub-100ms delay from audio playback to visualization

## Requirements

- macOS (uses Apple Music integration and CoreAudio)
- [BlackHole](https://github.com/ExistentialAudio/BlackHole) - Virtual audio driver for system audio capture
- Rust toolchain (for building from source)

## Setup

### 1. Install BlackHole

BlackHole creates a virtual audio device that lets the visualizer capture system audio:

```bash
brew install blackhole-2ch
```

### 2. Configure Audio MIDI Setup

1. Open **Audio MIDI Setup** (in `/Applications/Utilities/`)
2. Click the **+** button and create a "Multi-Output Device"
3. Check both your normal speakers/headphones AND BlackHole 2ch
4. Set this Multi-Output Device as your system output

Now audio will play through your speakers and be captured by MusicDash simultaneously.

### 3. Build and Run

```bash
cargo build --release
cargo run --release
```

## Usage

Run with default red theme:
```bash
cargo run --release
```

Choose a specific color:
```bash
cargo run --release blue
cargo run --release green
cargo run --release purple
cargo run --release orange
```

Use album art colors automatically:
```bash
cargo run --release album
```

Press `q` to quit.

## How it works

MusicDash runs three threads in parallel:
1. **Audio capture** - Pulls audio from BlackHole at 48kHz
2. **Metadata polling** - Queries Apple Music every second via AppleScript
3. **UI rendering** - Draws the terminal interface at 60 FPS

Audio is processed through a 4096-point FFT with Hann windowing for smooth frequency analysis.

## Tech Stack

- **Rust** - Core language
- **CPAL** - Cross-platform audio I/O
- **Ratatui** - Terminal UI framework
- **spectrum-analyzer** - FFT and frequency analysis
- **image** - Album art processing
- **AppleScript** - macOS Music app integration

## License

Open source project - feel free to use and modify!

---

Built with Rust 🦀

