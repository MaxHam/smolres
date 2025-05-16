# smolres

**Turn your high-resolution images into pixelated, low-res magic.**

smolres is a Rust CLI tool that pixelates images by reducing their resolution and enlarging blocks of pixels. Whether you want to turn your precious high-res images into blocky pixel art or just reduce image sizes for some retro flair, smolres makes it easy and fun.

This projects purpose is to learn Rust for me.

## How to install

1. Build the project

```shell
cargo install --path .
```

2. Install it globally. If you want to use smolres from anywhere:

```shell
cargo install --path .
```

3. Check it out

```shell
smolres --help
```

## Roadmap

### Core functionality

-[x] Basic CLI setup with clap for argument parsing.
-[] Decode PNG/JPEG to raw RGB (using png or jpeg-decoder).
-[] Implement pixelation algorithm (downscale to a virtual grid, then upscale).
-[] Implement output saving (as PNG, or optionally PPM).
-[] Implement grayscale conversion for retro look (optional flag).
-[] Implement bit-depth reduction (2-bit, 4-bit color quantization).
-[] Handle basic error cases (invalid files, missing arguments).

### Advanced features

-[] Add batch processing (process an entire folder of images).
-[] Add progress bar (using indicatif).
-[] Improve color palette management (for 2-bit/4-bit output).
-[] Add a logging system for detailed outputs (tracing or log crate).
-[] Allow custom aspect ratio pixelation (e.g., 16x9 instead of square grids).
