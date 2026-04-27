# Agent Protocol (SEMMAP-first)

You can browse the repo, but orientation and verification MUST be SEMMAP/Neti-driven. Neti and SEMMAP binaries are installed in the system path and can be invoked directly, for example `neti check`, `semmap generate`, `semmap trace`, and `semmap cat`.

## Hard rule: no source before orientation

Before reading implementation source beyond the task-defining docs, you MUST:

1. run `semmap generate` (if not already done for this repo state),
2. read `SEMMAP.md` and cite the specific line(s) you used (Purpose plus the relevant layer entries and hotspots),
3. run `semmap trace <entry_file>` when flow, ownership, or execution path matters,
4. state your **minimal working set** (the 2-5 files you intend to read next, and why SEMMAP led you to them).

You may read the task-defining docs first: this prompt. Issue discovery and issue updates must go through the `ishoo` CLI against the canonical store.

`neti check` is the canonical verification command. It already runs the configured verification suite, including `cargo test`, the configured Clippy gate, and Neti scan. Do not treat ad hoc `cargo test` or `cargo clippy` runs as a substitute for `neti check`.

## Workflow

1. Run `semmap generate` and read `SEMMAP.md`. Use the `ishoo` CLI to inspect issues. You can also run `semmap deps` if you need a dependency graph.
2. Write a short Orientation (Purpose, entrypoint, trace target, hotspots, plan).
3. Use `semmap trace <entry_file>` for flow-dependent work or unclear ownership.
4. Declare a minimal working set, then read only those files (strongly prefer `semmap cat`; use other tools if needed).
5. Make minimal edits that respect SEMMAP boundaries; hotspots = smaller diffs + stronger tests.
6. After the change set is in place, run `neti check` (view `neti-report.txt` in repo root, or `neti-report-full.txt` for detailed output). Iterate until clean or only pre-existing failures remain and are called out explicitly. You must resolve your own technical debt before moving forward.
7. If you manually exercise a CLI or user-facing flow, report the exact command, the important output, and the exit code when relevant.

## Issue discipline

Before doing any implementation work on an Ishoo issue, you MUST first assess whether the issue is actually executable _as written_.

An issue is executable _only_ if it has:

- _one_ concrete change
- _one_ main code surface
- _one_ clear proof of done
- _no_ open design question that must be answered during implementation

You MUST explicitly state a **Scope verdict** before coding:

- `Scope verdict: executable`
- or `Scope verdict: too broad; must split first`

If the issue is too broad, _DO NOT silently narrow it in your head, do one convenient slice, and then claim the full issue is DONE_, thats very bad. Instead, use `ishoo` to decompose it into smaller issues and keep the parent issue open until its real full scope is complete (you may need to re-order ishoo plan,)

As a rule of thumb, _an issue is probably too broad if it combines multiple layers at once_, such as:

- model
- CLI
- UI
- export
- docs
- tests
- architecture cleanup

Explicitly perform this verbal test in the chat to describe the issue in a few lines:

- the one concrete change,
- the one main code surface,
- the proof of done,
- and what is out of scope,

You must do this before working on each issue. If you read the issue and CANNOT do this, then the scope must be too large and it is not ready for direct coding, needing to be decomposed.

Work exclusively from the canonical issue store through the CLI. The storage file is a compressed binary; do not attempt to read or modify it directly.

Use these commands as your primary control surface:

- `ishoo agenda --next` (Your primary source of truth for "What's next")
- `ishoo list --compact` (Query the state of the board)
- `ishoo show <ID>` (Read full details/description of a task)
- `ishoo new "<Title>" --category <CAT> --labels <labels>` (Create new work)
- `ishoo edit <ID> --description "<Text>" --depends-on <ID>` (Refine/Link work)
- `ishoo set <ID> <status>` (Update status: active, backlog, done)
- `ishoo help --all` (See all commands and how to use them)

When refining or closing an issue, your `Resolution` update MUST include:

1. **What changed:** Concrete summary of code changes.
2. **Why:** Architectural justification.
3. **Verification:** The exact commands run and the outcome of `neti check`.
4. **Handoff:** If this enables a blocked issue, mention it.

An issue is only DONE when `neti check` is PASS and the status is updated via `ishoo set <ID> done`.

## Minimal close-out

A compliant final report for code work should usually contain:

1. the issue handled, scope described
2. the SEMMAP evidence used
3. the key files changed
4. the exact `neti check` outcome
5. any manual CLI or UX verification that was performed
6. whether issue records were updated
