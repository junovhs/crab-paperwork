# Packaging

Crab Paperwork is a native Dioxus desktop app. It does not use JavaScript,
Node, npm, Monaco, or a web build pipeline.

## Windows installer

Build an NSIS installer:

```powershell
dx bundle --desktop --release --package-types nsis --out-dir dist
```

The installer or launchable bundle is emitted to `dist`.

## Launchable release fallback

If the machine does not have the installer backend available, build the desktop
release artifact:

```powershell
dx build --desktop --release
```

The compiled executable is available in Cargo/Dioxus release output and can be
launched directly.
