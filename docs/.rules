# Zed Lite Documentation Rules

These rules apply to files under `docs/`.

## Scope

- Document **Zed Lite**, not upstream Zed.
- Keep docs limited to retained functionality: editor, Git/Git UI, terminal,
  settings, keymaps, themes, language extensions, diagnostics, LSP, search, and
  Markdown/CSV/SVG/image previews.
- Do not document removed AI, agents, edit prediction, login, cloud,
  collaboration, remote development, debugger/DAP, REPL, notebooks, auto-update,
  or multi-channel release behavior.
- Do not mention dev/nightly/preview/stable channels as product variants.

## Voice

- Be direct and practical.
- Start with what the reader can do.
- State limitations plainly.
- Avoid marketing language and superlatives.
- Address the reader as "you".
- Use present tense.

## Formatting

- Use `sh` code blocks for shell commands.
- Use backticks for settings, paths, commands, keybindings, and action names.
- Show complete JSON examples, not fragments.
- Use tables for mappings and comparisons.
- Use blockquotes with bold labels:

```md
> **Tip:** Useful detail.
```

## Keybindings and Actions

- Prefer docs preprocessor syntax when available:
  - `{#kb scope::Action}`
  - `{#action scope::Action}`
- Do not hardcode keybindings when an action reference exists.
- Keep namespaces tied to retained modules such as `zed`, `editor`, `git`,
  `workspace`, `terminal`, `vim`, and `settings`.

## Links and Assets

- Use relative links within `docs/src`.
- Do not add binary image or video assets to the repository.
- Avoid links to removed upstream hosted workflows or channel-specific docs.

## Before Commit

Run formatting when docs content changes:

```sh
cd docs
pnpm dlx prettier@3.5.0 . --write
```
