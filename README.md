# Dioxus UI

The goal: bring the shadcn/ui component registry model to Dioxus — Tailwind CSS, copy-paste ready components for Rust web apps.

> **Early stage, experimental** — expect breaking changes. Open to contributors.

This project will eventually be integrated into [rust-ui.com](https://www.rust-ui.com), alongside the existing Leptos component library.

Live demo: **[dioxus-ui.wasmer.app](https://dioxus-ui.wasmer.app)**

## Stack

- [Dioxus](https://dioxuslabs.com/) 0.7
- [Tailwind CSS](https://tailwindcss.com/) v4
- [tw_merge](https://crates.io/crates/tw_merge) — class merging
- [icons](https://crates.io/crates/icons) — Lucide icons for Dioxus
- Deployed on [Wasmer Edge](https://wasmer.io/)

## Run locally

```bash
dx serve
```

## License

MIT — see [LICENSE](./LICENSE)
