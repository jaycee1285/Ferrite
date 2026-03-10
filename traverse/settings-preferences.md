---
id: settings-preferences
kind: persistence-boundary
authority:
  - serialized-settings
mutates:
  - in-memory-settings-state
observes: []
persists_to:
  - serialized-settings
depends_on: []
staleness_risks:
  - older-settings-files
entrypoints:
  - src/config/settings.rs
  - src/ui/settings.rs
  - src/ui/welcome.rs
---

# Settings Preferences

## Purpose
Defines the persisted schema for user-configurable Ferrite behavior, including font family, font size, UI language, and CJK regional glyph preference. The settings UI and startup path both route through this model before runtime font setup occurs.

## Scope of Touch
Safe to edit when changing:
- default appearance values
- enum labels and descriptions shown in settings surfaces
- validation and sanitization for persisted preference values

Risky to edit when changing:
- serialized field names or enum variants
- assumptions used by startup font preload logic
- language-to-required-CJK-font mapping

## Authority Notes
This node is the contract for persisted preference shape. Runtime font code reads from it but should not redefine the meaning of CJK preference values independently.

## Links
- [Font Runtime Pipeline](font-runtime-pipeline.md)

