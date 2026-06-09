# Zed Lite Docs

This directory contains documentation sources inherited from Zed and trimmed for
Zed Lite.

The docs should describe the lite product only. Do not document removed AI,
collaboration, cloud, remote, debugger, notebook, auto-update, or multi-channel
release behavior unless that functionality is restored.

## Preview Locally

Install mdBook, then serve the book:

```sh
cargo install mdbook@0.4.40
mdbook serve docs
```

Some pages may still reference the custom docs preprocessor inherited from
upstream Zed. If a page uses action or keybinding templates, generate metadata
first:

```sh
script/generate-action-metadata
mdbook serve docs
```

## Formatting

Format docs before committing:

```sh
cd docs
pnpm dlx prettier@3.5.0 . --write
```

## Notes

- Prefer direct user-facing docs for the lite build.
- Avoid references to `zed.dev` deployment, preview docs, nightly docs, or
  release channels.
- Use external image/video hosting instead of adding binary assets to the repo.
