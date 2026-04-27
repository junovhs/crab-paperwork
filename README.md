# Crab Paperwork

A tiny native Markdown live preview desktop app built with Rust, Dioxus, and CSS.

## Principles

- Rust and CSS only
- No JavaScript
- No Node
- No npm
- No web build pipeline
- No editor-platform sprawl

## Scope

Crab Paperwork is intentionally small:

- Markdown input pane
- Rendered preview pane
- Open Markdown from disk
- Save Markdown back to disk
- Reset
- Copy
- Export sanitized preview HTML as the no-JavaScript PDF fallback
- Sync scroll setting
- Light/dark theme
- Local session persistence

## Text Layout

The editor uses a Rust-native, Pretext-inspired layout helper: text is prepared
once into grapheme-aware measured segments, then re-laid out cheaply for width
and line-height calculations. It follows Pretext's prepare/layout shape while
staying inside this app's Rust-only, no-JavaScript, no-npm constraints.

## Run

```bash
cargo run
```

## Check

```bash
neti check
```

## Build a Desktop App

Windows first:

```bash
dx bundle --desktop --release --package-types nsis --out-dir dist
```

The launchable app/installer is written under `dist`. If NSIS is unavailable
on the machine, build an unpacked desktop release with:

```bash
dx build --desktop --release
```
