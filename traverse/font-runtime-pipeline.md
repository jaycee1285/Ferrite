---
id: font-runtime-pipeline
kind: module
authority:
  - settings-preferences
  - embedded-font-assets
  - system-locale
mutates:
  - egui-font-definitions
  - font-generation-counter
  - script-font-load-flags
observes:
  - settings-preferences
  - embedded-font-assets
  - system-locale
  - document-text-script-detection
persists_to: []
depends_on:
  - settings-preferences
staleness_risks:
  - editor-galley-cache
  - partially-warmed-font-atlas
entrypoints:
  - src/fonts.rs
  - src/app/mod.rs
  - src/app/central_panel.rs
  - src/editor/ferrite/editor.rs
---

# Font Runtime Pipeline

## Purpose
Builds egui font definitions from Ferrite's embedded Spline Sans families, optional custom font selection, and lazily loaded script-specific fallbacks. It also tracks font generation so editor caches can invalidate when the atlas changes.

## Scope of Touch
Safe to edit when changing:
- font preload heuristics
- CJK preference ordering
- font atlas invalidation and prewarm behavior

Risky to edit when changing:
- the set of embedded font files expected by `include_bytes!`
- cache invalidation behavior relied on by the editor
- startup and settings flows that rebuild fonts

## Authority Notes
This module is not the source of truth for user preference values; it consumes `Settings::font_family`, `Settings::cjk_font_preference`, and inferred locale/script signals. The authoritative built-in font bytes live in repo-root `.ttf` assets that are compiled into the binary.

## Links
- [Settings Preferences](settings-preferences.md)
- [Embedded Font Assets](embedded-font-assets.md)

