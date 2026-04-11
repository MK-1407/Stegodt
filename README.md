# Stegodt

A fast, command-line steganalysis tool that detects hidden data in images using statistical analysis techniques.

## Overview

Stegodt analyzes digital images to identify statistical anomalies commonly introduced by LSB (Least Significant Bit) steganography tools. It uses three complementary detection methods:

- **Chi-Squared Analysis** - Statistical histogram analysis
- **Odd-Even Analysis** - LSB parity pattern detection
- **Pair Analysis** - Adjacent value frequency detection

## Installation

```bash
cargo build --release
```

The binary will be at `target/release/stegodt.exe` (Windows) or `target/release/stegodt` (Unix).

## Usage

```bash
stegodt <command> <image_path>
```

### Commands

| Command | Description |
|---------|-------------|
| `odd` | Run odd-even LSB analysis |
| `pair` | Run pair value analysis |
| `chi` | Run chi-squared analysis |
| `complete` | Run all analyses with weighted score |

### Examples

```bash
# Run all detection methods
stegodt complete suspicious.png

# Run individual analysis
stegodt chi image.jpg
stegodt odd image.png
stegodt pair image.bmp
```

## Output Interpretation

When running `stegodt complete`, the tool outputs a suspicion score between 0.0 and 1.0:

| Score Range | Result |
|-------------|--------|
| > 0.75 | **HIGH** confidence steganography detected |
| > 0.50 | Moderate suspicion |
| > 0.30 | Weak signal |
| < 0.30 | Likely natural image |

## Supported Formats

- PNG
- JPEG
- BMP
- TIFF
- GIF
- WebP
- EXR
- AVIF

## How It Works

### Chi-Squared Analysis
Computes chi-square statistics on even/odd pixel value pairs. Steganographic images typically show very low chi-square values due to pair equalization.

### Odd-Even Analysis
Natural images have uneven LSB distributions. Steganography tools often produce near-perfect 50/50 balance, which is statistically suspicious.

### Pair Analysis
Measures frequency of adjacent pixel value pairs (2n, 2n+1). Hidden data tends to equalize pair frequencies.

## Technical Details

- **Language:** Rust
- **Edition:** 2026
- **Dependencies:** `image`, `colored`, `figlet-rs`

## License

MIT
