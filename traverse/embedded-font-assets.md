---
id: embedded-font-assets
kind: persistence-boundary
authority:
  - repo-root-font-binaries
mutates: []
observes: []
persists_to: null
depends_on: []
staleness_risks:
  - include-bytes-paths
  - mismatched-regular-bold-italic-variants
entrypoints:
  - spline-sans.ttf
  - spline-sans-bold.ttf
  - spline-sans-mono.ttf
  - spline-sans-mono-bold.ttf
  - spline-sans-mono-italic.ttf
  - spline-sans-mono-bold-italic.ttf
---

# Embedded Font Assets

## Purpose
Provides the built-in proportional and monospace font binaries that `src/fonts.rs` embeds with `include_bytes!`. These files determine the default UI/code typography shipped in the application binary.

## Scope of Touch
Safe to edit when changing:
- which concrete font binaries Ferrite ships by default
- file replacement of an existing weight or style variant

Risky to edit when changing:
- filenames referenced by `include_bytes!`
- assumptions that italic slots reuse regular or bold files
- binary size and packaging impact of added font assets

## Authority Notes
These font files are the authoritative source for Ferrite's built-in Latin UI and monospace faces. `src/fonts.rs` only maps them into egui families; it does not own the actual glyph data.

## Links
- [Font Runtime Pipeline](font-runtime-pipeline.md)

