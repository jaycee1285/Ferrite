---
id: feature-index
kind: index
authority: []
mutates: []
observes:
  - font-runtime-pipeline
  - settings-preferences
  - embedded-font-assets
persists_to: []
depends_on:
  - font-runtime-pipeline
  - settings-preferences
  - embedded-font-assets
staleness_risks: []
entrypoints:
  - traverse/font-runtime-pipeline.md
  - traverse/settings-preferences.md
  - traverse/embedded-font-assets.md
---

# Feature Index

## Font And Preference Neighborhood
- [Font Runtime Pipeline](font-runtime-pipeline.md): runtime font definition assembly, lazy script loading, and atlas invalidation.
- [Settings Preferences](settings-preferences.md): persisted appearance and CJK preference contract consumed at startup and in settings surfaces.
- [Embedded Font Assets](embedded-font-assets.md): built-in shipped font binaries compiled into the app.

