# Architecture

Crab Paperwork is a small native Markdown live preview app.

The app deliberately avoids becoming a full editor platform. It is organized around one simple loop:

```text
Markdown text -> Markdown renderer -> sanitizer -> preview pane
```

## Boundaries

### `app`

Owns top-level state, app actions, defaults, and shell composition.

### `markdown`

Owns Markdown rendering and HTML sanitization.

### `ui`

Owns Dioxus components only.

### `storage`

Owns config and session persistence.

### `platform`

Owns OS integrations such as clipboard and file dialogs.

### `export`

Owns HTML/PDF export boundaries.

### `theme`

Owns theme names and CSS class mapping.

## Non-goals

- No JavaScript
- No Node
- No Monaco
- No terminal
- No Git integration
- No workspace explorer
- No LSP
- No custom editor engine

````

### `docs/PHASES.md`

```markdown
# Phases

## Phase 1

Create the Rust and CSS scaffold.

## Phase 2

Implement Markdown rendering, sanitization, and state.

## Phase 3

Implement UI components.

## Phase 4

Wire clipboard, persistence, reset, theme, and export actions.

## Phase 5

Polish layout, keyboard shortcuts, scroll behavior, and packaging.
````
