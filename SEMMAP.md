# crab-paperwork -- Semantic Map

**Purpose:** Native Rust Dioxus desktop Markdown live preview app modeled after markdownlivepreview.com

## Legend

`[ENTRY]` Application entry point

`[CORE]` Core business logic

`[TYPE]` Data structures and types

`[UTIL]` Utility functions

`[HOTSPOT]` High fan-in file imported by 4+ others - request this file early in any task

`[GLOBAL-UTIL]` High fan-in utility imported from 3+ distinct domains

`[DOMAIN-CONTRACT]` Shared contract imported mostly by one subsystem

`[ROLE:model]` Primary domain model or state-holding data structure.

`[ROLE:controller]` Coordinates commands, events, or request handling.

`[ROLE:rendering]` Produces visual output or drawing behavior.

`[ROLE:view]` Represents a reusable UI view or presentation component.

`[ROLE:dialog]` Implements dialog-oriented interaction flow.

`[ROLE:config]` Defines configuration loading or configuration schema behavior.

`[ROLE:os-integration]` Bridges the application to OS-specific APIs or services.

`[ROLE:utility]` Provides cross-cutting helper logic without owning core flow.

`[ROLE:bootstrap]` Initializes the application or wires subsystem startup.

`[ROLE:build-only]` Supports the build toolchain rather than runtime behavior.

`[COUPLING:pure]` Logic stays within the language/runtime without external surface coupling.

`[COUPLING:mixed]` Blends pure logic with side effects or boundary interactions.

`[COUPLING:ui-coupled]` Depends directly on UI framework, rendering, or windowing APIs.

`[COUPLING:os-coupled]` Depends directly on operating-system services or platform APIs.

`[COUPLING:build-only]` Only relevant during build, generation, or compilation steps.

`[BEHAVIOR:owns-state]` Maintains durable in-memory state for a subsystem.

`[BEHAVIOR:mutates]` Changes application or model state in response to work.

`[BEHAVIOR:renders]` Produces rendered output, drawing commands, or visual layout.

`[BEHAVIOR:dispatches]` Routes commands, events, or control flow to other units.

`[BEHAVIOR:observes]` Listens to callbacks, notifications, or external signals.

`[BEHAVIOR:persists]` Reads from or writes to durable storage.

`[BEHAVIOR:spawns-worker]` Creates background workers, threads, or async jobs.

`[BEHAVIOR:sync-primitives]` Coordinates execution with locks, channels, or wait primitives.

`[SURFACE:filesystem]` Touches filesystem paths, files, or directory traversal.

`[SURFACE:ntfs]` Uses NTFS-specific filesystem semantics or metadata.

`[SURFACE:win32]` Touches Win32 platform APIs or Windows-native handles.

`[SURFACE:shell]` Integrates with shell commands, shell UX, or command launch surfaces.

`[SURFACE:clipboard]` Reads from or writes to the system clipboard.

`[SURFACE:gdi]` Uses GDI drawing primitives or related graphics APIs.

`[SURFACE:control]` Represents or manipulates widget/control surfaces.

`[SURFACE:view]` Represents a view-level presentation surface.

`[SURFACE:dialog]` Represents a dialog/window interaction surface.

`[SURFACE:document]` Represents document-oriented editing or display surfaces.

`[SURFACE:frame]` Represents application frame/window chrome surfaces.

`[BEHAVIOR:async]` Uses async/await patterns for concurrent execution.

`[BEHAVIOR:panics-on-error]` Contains unwrap/expect/panic patterns that abort on failure.

`[BEHAVIOR:logs-and-continues]` Logs errors and continues without propagating or aborting.

`[BEHAVIOR:returns-nil-on-error]` Returns nil/null/None on error instead of propagating.

`[BEHAVIOR:swallows-errors]` Catches errors without re-raising or propagating them.

`[BEHAVIOR:propagates-errors]` Propagates errors to callers via Result, throw, or raise.

`[SURFACE:http-handler]` Implements HTTP request handling or web endpoint logic.

`[SURFACE:database]` Interacts with database services or ORMs.

`[SURFACE:external-api]` Makes outbound calls to external HTTP APIs or services.

`[SURFACE:template]` Uses template engines for rendering output.

`[QUALITY:undocumented]` Has public symbols without documentation.

`[QUALITY:complex-flow]` Contains functions with high cognitive complexity.

`[QUALITY:error-boundary]` Concentrated error handling — many panic, swallow, or propagation sites.

`[QUALITY:concurrency-heavy]` Uses multiple concurrency primitives (async, locks, spawn).

`[QUALITY:syntax-degraded]` Parse errors detected — semantic analysis may be incomplete.

## Layer 0 -- Config

`Cargo.toml`
Workspace configuration.

`Dioxus.toml`
Configuration for Dioxus.

`README.md`
Project overview and usage guide.

`SEMMAP.md`
Generated semantic map.

`neti.toml`
Configuration for neti.

`rustfmt.toml`
Configuration for rustfmt.

`src/storage/config.rs`
Provides shared config used across multiple domains. [HOTSPOT] [COUPLING:pure] [QUALITY:undocumented]
Exports: AppConfig.normalized_split_ratio, AppConfig, AppConfig.default
Semantic: pure computation

## Layer 1 -- Domain (Engine)

`src/app.rs`
Implements App functionality. [COUPLING:mixed] [BEHAVIOR:owns-state]
Exports: actions, defaults, state
Semantic: side-effecting stateful module

`src/app/actions.rs`
Implements app action. [COUPLING:pure] [QUALITY:undocumented]
Exports: AppAction, initial_state, reduce
Semantic: pure computation

`src/app/defaults.rs`
Implements default markdown. [COUPLING:mixed] [BEHAVIOR:owns-state]
Exports: DEFAULT_MARKDOWN
Semantic: side-effecting stateful module

`src/app/state.rs`
Implements app state.default. [COUPLING:pure] [QUALITY:undocumented]
Exports: ThemeMode.from_config_value, ThemeMode, ThemeMode.as_str, ThemeMode.css_class
Semantic: pure computation

`src/markdown.rs`
Re-exports the public API surface.
Exports: render, sanitize

`src/markdown/sanitize.rs`
Implements sanitize html. [COUPLING:mixed] [BEHAVIOR:panics-on-error]
Exports: sanitize_html
Semantic: side-effecting that panics on error

`src/platform.rs`
Re-exports the public API surface.
Exports: clipboard, dialogs, files

`src/platform/clipboard.rs`
Implements copy text. [COUPLING:pure] [BEHAVIOR:propagates-errors]
Exports: copy_text
Semantic: pure computation that propagates errors

`src/platform/dialogs.rs`
Implements pick export html save path. [COUPLING:mixed] [BEHAVIOR:persists] [QUALITY:undocumented]
Exports: pick_export_html_save_path, pick_markdown_open_path, pick_markdown_save_path
Semantic: side-effecting adapter

`src/platform/files.rs`
controller for files via file I/O. [COUPLING:mixed] [BEHAVIOR:persists]
Exports: read_markdown, write_markdown
Semantic: side-effecting adapter

`src/pretext.rs`
Implements prepare options.default. [HOTSPOT] [COUPLING:mixed] [BEHAVIOR:panics-on-error] [QUALITY:undocumented,complex-flow]
Exports: measure_line_stats, PreparedText, WhiteSpace, WordBreak
Semantic: side-effecting that panics on error

`src/storage.rs`
config for storage via file I/O. [HOTSPOT] [COUPLING:mixed] [BEHAVIOR:persists,propagates-errors] [QUALITY:undocumented,error-boundary,syntax-degraded]
Exports: load_config, load_session, save_config, save_session
Semantic: side-effecting adapter that propagates errors

`src/storage/paths.rs`
config for paths via file I/O. [COUPLING:mixed] [BEHAVIOR:owns-state,persists,propagates-errors] [QUALITY:undocumented,error-boundary]
Exports: project_dirs, data_dir, session_file, config_dir
Semantic: side-effecting stateful adapter that propagates errors

`src/storage/session.rs`
Implements session state. [HOTSPOT]
Exports: SessionState

`src/ui.rs`
Re-exports the public API surface. [HOTSPOT]
Exports: editor, preview, split, status

`src/ui/editor.rs`
Implements editor props. [COUPLING:pure]
Exports: EditorProps, Editor
Semantic: pure computation

`src/ui/preview.rs`
Implements preview props. [COUPLING:pure]
Exports: PreviewProps, Preview
Semantic: pure computation

`src/ui/split.rs`
Implements split props. [COUPLING:mixed] [BEHAVIOR:panics-on-error] [QUALITY:error-boundary]
Exports: SplitProps, Split
Semantic: side-effecting that panics on error

`src/ui/status.rs`
Implements status props. [COUPLING:pure]
Exports: StatusProps, Status
Semantic: pure computation

`src/ui/toolbar.rs`
Implements toolbar props. [COUPLING:pure]
Exports: ToolbarProps, Toolbar
Semantic: pure computation

## Layer 2 -- Adapters / Infra

`src/export.rs`
Re-exports the public API surface.
Exports: html, pdf

`src/export/html.rs`
utility for html via file I/O. [COUPLING:mixed] [BEHAVIOR:persists]
Exports: save_html
Semantic: side-effecting adapter

`src/export/pdf.rs`
Sets the pdf. [COUPLING:pure]
Exports: save_pdf
Semantic: pure computation

`src/markdown/render.rs`
Formats markdown for output. [HOTSPOT] [COUPLING:mixed] [BEHAVIOR:panics-on-error]
Exports: render_markdown
Semantic: side-effecting that panics on error

## Layer 3 -- App / Entrypoints

`assets/github_markdown_dark.css`
Implements github markdown dark. styles.

`assets/github_markdown_light.css`
Implements github markdown light. styles.

`assets/main.css`
Implements main functionality. styles.

`src/lib.rs`
Re-exports the public API surface. [HOTSPOT]
Exports: export, markdown, platform, pretext

`src/main.rs`
Application entry point. [COUPLING:pure]
Semantic: pure computation

## Layer 4 -- Tests

`tests/markdown_render.rs`
Tests for crab_paperwork. [COUPLING:mixed]
Semantic: side-effecting

`tests/pretext_layout.rs`
Tests for crab_paperwork. [COUPLING:mixed]
Semantic: side-effecting

`tests/storage.rs`
Tests for crab_paperwork. [COUPLING:mixed]
Semantic: side-effecting


## DependencyGraph

```yaml
DependencyGraph:
  # --- Entrypoints ---
  github_markdown_dark.css, github_markdown_light.css, main.css, main.rs:
    Imports: []
    ImportedBy: []
  # --- High Fan-In Hotspots ---
  config.rs:
    Imports: []
    ImportedBy: [actions.rs, app.rs, editor.rs, pretext.rs, pretext_layout.rs, sanitize.rs, src/storage.rs, tests/storage.rs]
  lib.rs:
    Imports: [app.rs, export.rs, markdown.rs, platform.rs, pretext.rs, src/storage.rs, ui.rs]
    ImportedBy: [editor.rs, files.rs, markdown_render.rs, render.rs]
  render.rs:
    Imports: [lib.rs, markdown.rs, sanitize.rs]
    ImportedBy: [actions.rs, app.rs, markdown.rs, markdown_render.rs]
  ui.rs:
    Imports: [editor.rs, preview.rs, split.rs, status.rs, toolbar.rs]
    ImportedBy: [app.rs, lib.rs, pretext.rs, split.rs]
  # --- Layer 0 -- Config ---
  Cargo.toml, Dioxus.toml, README.md, SEMMAP.md, neti.toml, rustfmt.toml:
    Imports: []
    ImportedBy: []
  # --- Layer 1 -- Domain (Engine) ---
  actions.rs:
    Imports: [app.rs, config.rs, markdown.rs, render.rs]
    ImportedBy: [app.rs]
  app.rs:
    Imports: [actions.rs, config.rs, defaults.rs, render.rs, session.rs, src/storage.rs, state.rs, ui.rs]
    ImportedBy: [actions.rs, lib.rs]
  clipboard.rs, dialogs.rs:
    Imports: []
    ImportedBy: [platform.rs]
  defaults.rs, state.rs:
    Imports: []
    ImportedBy: [app.rs]
  editor.rs:
    Imports: [config.rs, lib.rs, pretext.rs]
    ImportedBy: [ui.rs]
  files.rs:
    Imports: [lib.rs]
    ImportedBy: [platform.rs]
  markdown.rs:
    Imports: [render.rs, sanitize.rs]
    ImportedBy: [actions.rs, lib.rs, render.rs]
  paths.rs:
    Imports: []
    ImportedBy: [src/storage.rs]
  platform.rs:
    Imports: [clipboard.rs, dialogs.rs, files.rs]
    ImportedBy: [lib.rs]
  pretext.rs:
    Imports: [config.rs, ui.rs]
    ImportedBy: [editor.rs, lib.rs, pretext_layout.rs]
  preview.rs, status.rs, toolbar.rs:
    Imports: []
    ImportedBy: [ui.rs]
  sanitize.rs:
    Imports: [config.rs, export.rs]
    ImportedBy: [markdown.rs, render.rs]
  session.rs:
    Imports: []
    ImportedBy: [app.rs, src/storage.rs]
  split.rs:
    Imports: [ui.rs]
    ImportedBy: [ui.rs]
  src/storage.rs:
    Imports: [config.rs, paths.rs, session.rs]
    ImportedBy: [app.rs, lib.rs]
  # --- Layer 2 -- Adapters / Infra ---
  export.rs:
    Imports: [html.rs, pdf.rs]
    ImportedBy: [lib.rs, sanitize.rs]
  html.rs, pdf.rs:
    Imports: []
    ImportedBy: [export.rs]
  # --- Tests ---
  markdown_render.rs:
    Imports: [lib.rs, render.rs]
    ImportedBy: []
  pretext_layout.rs:
    Imports: [config.rs, pretext.rs]
    ImportedBy: []
  tests/storage.rs:
    Imports: [config.rs]
    ImportedBy: []
```
