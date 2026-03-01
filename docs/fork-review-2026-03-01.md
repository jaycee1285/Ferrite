# Ferrite Review and Fork Brief

## Scope

This document captures:

- a feature inventory for the current repository
- the highest-impact codebase issues
- a practical fork direction for a leaner variant

Date: 2026-03-01

## Feature Inventory

| Area | Feature | Included | Notes |
|------|---------|----------|-------|
| Editing | Raw text editor | Yes | Custom rope-based Ferrite editor for large files |
| Editing | WYSIWYG Markdown editing | Yes | Editable rendered markdown with click-to-edit behavior |
| Editing | Split view | Yes | Raw and rendered side-by-side |
| Editing | Zen mode | Yes | Centered distraction-free writing mode |
| Editing | Multi-cursor editing | Yes | Custom editor supports multiple selections/cursors |
| Editing | Undo/redo | Yes | Tab-level state plus editor-level history |
| Editing | Find/replace | Yes | Includes regex support and batch replace |
| Editing | Go to line | Yes | Integrated command/panel support |
| Editing | Line duplicate/move/delete | Yes | Keyboard and command handling present |
| Editing | Code folding | Yes | Heading/list/code-block oriented folding |
| Editing | Bracket matching | Yes | Windowed search around cursor |
| Editing | Auto-close brackets/quotes | Yes | Disabled in some large-file paths |
| Editing | Syntax highlighting | Yes | Syntect + extra grammars |
| Editing | Minimap | Yes | Semantic and pixel modes |
| Editing | Vim mode | Yes | Modal editing support exists in custom editor |
| Markdown | Table of contents generation | Yes | TOC insert/update support |
| Markdown | Smart paste for links | Yes | URL paste wraps selection |
| Markdown | Drag/drop image insertion | Yes | Saves into local `assets/` folder |
| Markdown | Wikilinks | Yes | Includes backlinks panel support |
| Markdown | GitHub-style callouts | Yes | Documented and implemented |
| Markdown | Mermaid rendering | Yes | Large native implementation with many diagram types |
| Structured data | JSON/YAML/TOML tree view | Yes | Inline value editing, path copy, expand/collapse |
| Structured data | Structured document validation/formatting | Yes | JSON/YAML/TOML formatting flows exist |
| Tabular data | CSV/TSV table viewer | Yes | Delimiter detection and header detection included |
| Workspace | Folder/workspace mode | Yes | File tree, quick switcher, search-in-files |
| Workspace | Git status in tree | Yes | `git2`-based local repo integration |
| Workspace | Session persistence | Yes | Tabs, scroll positions, recovery paths |
| Panels | Outline panel | Yes | Heading navigation and sync |
| Panels | Backlinks panel | Yes | Markdown wikilink-focused |
| Panels | Settings/about/welcome panels | Yes | Includes first-run welcome flow |
| Terminal | Integrated terminal | Yes | PTY-backed multi-pane terminal workspace |
| Terminal | Terminal layout persistence | Yes | Saved layouts and theming |
| Automation | Live pipeline shell panel | Yes | JSON/YAML shell piping feature |
| Export | HTML export/copy-as-HTML | Yes | Themed HTML export path exists |
| Platform | Single-instance behavior | Yes | Local TCP forwarding for open-file requests |
| Platform | Custom window/title bar | Yes | Borderless window, custom resize handling |
| Platform | Update checker | Yes | Manual GitHub release check |
| i18n | Localized UI | Yes | Multiple YAML locale files |

## Major Codebase Issues

### 1. Dual source of truth for document state

Severity: High

The codebase still stores full document content in `Tab.content: String` while also maintaining a rope inside `FerriteEditor`.

Relevant references:

- [src/state.rs](/home/john/repos/Ferrite/src/state.rs#L1060)
- [src/editor/widget.rs](/home/john/repos/Ferrite/src/editor/widget.rs#L525)
- [src/editor/widget.rs](/home/john/repos/Ferrite/src/editor/widget.rs#L749)
- [docs/technical/editor/architecture.md](/home/john/repos/Ferrite/docs/technical/editor/architecture.md)

Why it matters:

- duplicates memory for every open document
- forces expensive `to_string()` bridges between app state and editor state
- keeps non-editor features coupled to legacy `String` content access
- makes correctness harder because selection/cursor/fold state also exists in parallel forms

This is the main architectural constraint on a better fork.

### 2. Full-buffer string materialization remains in hot UI coordination paths

Severity: High

Several UI paths still call `editor.buffer().to_string()` to derive selection ranges or synchronize editor state.

Relevant references:

- [src/app/mod.rs](/home/john/repos/Ferrite/src/app/mod.rs#L1443)
- [src/app/formatting.rs](/home/john/repos/Ferrite/src/app/formatting.rs)
- [src/app/find_replace.rs](/home/john/repos/Ferrite/src/app/find_replace.rs)
- [src/editor/widget.rs](/home/john/repos/Ferrite/src/editor/widget.rs#L527)

Why it matters:

- large files still pay for whole-buffer allocation during command flows
- performance fixes are scattered as special cases instead of solved structurally
- “large file support” depends on many guardrails rather than one clean data model

### 3. State synchronization is spread across app, tab, widget, and editor layers

Severity: High

The editor widget syncs content, cursor, scroll, fold state, and selection metadata between the app tab model and the internal editor state every frame.

Relevant references:

- [src/editor/widget.rs](/home/john/repos/Ferrite/src/editor/widget.rs#L525)
- [src/editor/widget.rs](/home/john/repos/Ferrite/src/editor/widget.rs#L658)
- [src/editor/widget.rs](/home/john/repos/Ferrite/src/editor/widget.rs#L732)
- [src/editor/widget.rs](/home/john/repos/Ferrite/src/editor/widget.rs#L749)

Why it matters:

- high regression risk when adding features
- behavior depends on ordering of UI/render/sync code
- stale state bugs are already documented in comments

### 4. Monolithic modules still dominate maintenance cost

Severity: Medium-High

The repo has already improved, but core files remain extremely large.

Relevant references:

- [docs/refactoring-assessment.md](/home/john/repos/Ferrite/docs/refactoring-assessment.md)
- [src/state.rs](/home/john/repos/Ferrite/src/state.rs)
- [src/markdown/editor.rs](/home/john/repos/Ferrite/src/markdown/editor.rs)
- [src/markdown/widgets.rs](/home/john/repos/Ferrite/src/markdown/widgets.rs)
- [src/editor/ferrite/editor.rs](/home/john/repos/Ferrite/src/editor/ferrite/editor.rs)

Why it matters:

- review and refactor cost is high
- cross-cutting changes land in 2k-5k line files
- the repo is hard to fork selectively without dragging along unrelated complexity

### 5. Product scope is broader than the core editor problem

Severity: Medium

The repo now combines a markdown editor, structured-data editor, CSV viewer, workspace manager, embedded terminal, shell pipeline runner, updater, custom windowing, and substantial Mermaid rendering.

Relevant references:

- [README.md](/home/john/repos/Ferrite/README.md)
- [src/terminal/mod.rs](/home/john/repos/Ferrite/src/terminal/mod.rs)
- [docs/technical/viewers/live-pipeline.md](/home/john/repos/Ferrite/docs/technical/viewers/live-pipeline.md)

Why it matters:

- core editor changes are harder to validate
- security and maintenance surface area is larger than necessary for a focused fork
- non-core features contribute significantly to code volume and platform edge cases

### 6. Performance strategy still leans on exceptions and feature disablement

Severity: Medium

Large-file handling disables or bypasses multiple features instead of relying on a consistent architecture.

Relevant references:

- [docs/technical/editor/large-file-performance.md](/home/john/repos/Ferrite/docs/technical/editor/large-file-performance.md)
- [docs/technical/planning/memory-optimization.md](/home/john/repos/Ferrite/docs/technical/planning/memory-optimization.md)

Why it matters:

- performance characteristics are harder to reason about
- user-visible behavior changes by file size threshold
- complexity grows every time a feature needs a large-file exception

## Recommended Fork Direction

### Working name

Ferrite Core

### Goal

Keep the parts that form a strong native editor:

- rope-based raw editor
- markdown preview/WYSIWYG path
- JSON/YAML/TOML tree view
- workspace tree + quick open + search
- Git status
- outline, backlinks, session persistence

Defer or remove for the fork:

- integrated terminal
- live pipeline shell execution
- update checker
- welcome flow
- productivity hub
- custom borderless windowing if it complicates platform polish

### Why this fork shape

This removes a large amount of non-core surface area while preserving the differentiators you likely care about: native egui UI, markdown focus, tree view, and workspace navigation.

## Fork Plan

### Phase 1: Stabilize the fork surface

- remove terminal and pipeline modules from UI navigation and settings
- remove updater wiring
- simplify default panels and startup flow
- keep file formats, workspace mode, tree viewer, markdown editor, and Git

### Phase 2: Make the editor the source of truth

- replace `Tab.content: String` as the authoritative runtime document model
- let `FerriteEditor` or a shared `DocumentBuffer` own document text
- generate strings only for save/export/clipboard/full-document operations
- convert app actions to operate on buffer ranges instead of whole strings

### Phase 3: Split core state

- break `state.rs` into `state/tab.rs`, `state/app_state.rs`, `state/folds.rs`, `state/history.rs`
- continue reducing `app/mod.rs` into orchestration-only code
- isolate markdown/tree/csv viewers behind a clearer document-view trait or enum

### Phase 4: Re-evaluate WYSIWYG scope

- if your main need is a robust markdown/code editor, consider keeping rendered preview but trimming editable widget complexity
- if your main need is visual markdown authoring, keep WYSIWYG and trim unrelated tooling first

## Suggested Decision Points

Before implementing the fork in earnest, decide which of these you want:

1. Writer-first: markdown + preview + wikilinks + backlinks, minimal dev tooling
2. Dev-notes-first: markdown + workspace tree + Git + JSON/YAML/TOML tree view
3. Structured-docs-first: markdown secondary, tree/csv viewers primary

Without that decision, it is easy to create a fork that is only “smaller” rather than better.

## Validation Notes

- `cargo check` and `cargo test` were started during review but had not completed at the time this document was written because dependency compilation was still in progress.
- The review above is based on repository structure, documentation, and direct code inspection of the main app/editor/state paths.
