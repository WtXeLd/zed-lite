# Zed Lite

Zed Lite is a trimmed build of Zed focused on local editing, Git workflows,
terminal usage, language/theme extension support, and lightweight previews.

This repository intentionally removes the cloud, AI, collaboration, remote
development, debugger, onboarding, update-channel, and multi-product release
paths from upstream Zed. The product identity is fixed as **Zed Lite**.

## Scope

Kept:

- Core editor, buffers, panes, search, command palette, settings, keymaps, Vim
  mode, snippets, diagnostics, LSP, and language tooling.
- Git integration and Git UI.
- Integrated terminal.
- Language and theme extension runtime.
- Preview UI for Markdown, CSV, SVG, and images.
- Multi-platform packaging for Linux, macOS, and Windows.

Removed:

- AI assistants, model providers, edit prediction, agents, and web search.
- Login, cloud API, collaboration, calls, channels, and sharing.
- Remote development, SSH/WSL/dev containers, remote server, and REPL/notebooks.
- Debug adapter UI and debugger support.
- Auto-update, multi-channel release logic, and old release workflows.

## Build

Use the lite feature set explicitly:

```sh
cargo check -p zed --no-default-features --features zed/lite
cargo build -p zed --no-default-features --features zed/lite
```

The CLI has a matching lite feature:

```sh
cargo build --no-default-features --features zed/lite,cli/lite --package zed --package cli
```

## Package

Packaging scripts always build **Zed Lite**. There are no dev, nightly, preview,
or stable variants.

```sh
./script/bundle-linux
./script/bundle-mac aarch64-apple-darwin
./script/bundle-mac x86_64-apple-darwin
pwsh script/bundle-windows.ps1 -Architecture x86_64
```

Generated artifacts use lite names:

- `target/release/zed-lite-linux-x86_64.tar.gz`
- `target/aarch64-apple-darwin/release/Zed-Lite-aarch64.dmg`
- `target/x86_64-apple-darwin/release/Zed-Lite-x86_64.dmg`
- `target/zed-lite-windows-x86_64.zip`

## Repository Notes

- `AGENTS.md`, `CLAUDE.md`, and `GEMINI.md` point to `.rules`.
- Root rules should describe the lite fork, not upstream Zed release process.
- Do not reintroduce removed cloud, AI, collaboration, remote, debugger, or
  multi-channel release modules unless explicitly requested.

## License

Zed Lite keeps the upstream licensing model: GPL-3.0-or-later for the main
application, with Apache-2.0 components where marked. Third-party dependency
license data is generated through the existing license scripts.
