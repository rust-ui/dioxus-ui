+++
title = "Installation"
description = "Get started with Rust/UI by installing the CLI tool and setting up your project with minimal dependencies and framework support."
tags = []
is_new = false
image = "/images/thumbnails/_placeholder.webp"
image_dark = "/images/thumbnails/_placeholder-dark.webp"
order = 2
+++

## Minimal Dependencies

Rust/UI is designed to be **as minimal as possible**. We don't rely on third party libraries like Radix UI, instead building components from the ground up to give you full control over your UI.

This approach means:

- **No External Dependencies:** Your components don't carry the weight of large third-party libraries.
- **Full Control:** Every line of code in your components is under your direct control and can be customized.
- **Better Performance:** Minimal overhead means faster load times and better runtime performance.
- **Security:** Fewer dependencies mean fewer potential security vulnerabilities in your application.

_Unlike other UI libraries that bundle heavyweight dependencies, Rust/UI components are built specifically for the Rust ecosystem with performance and simplicity in mind._

## Framework Support

**This is the Dioxus edition of Rust/UI** — built on [Dioxus](https://dioxuslabs.com/), a Rust framework with a React-like component model that renders to web, desktop, and mobile from a single codebase.

Our Dioxus integration offers:

- **Cross-Platform:** The same components work on web, desktop, and mobile targets.
- **Signals:** Built-in support for Dioxus signals and reactive primitives.
- **Server Functions:** Full fullstack support via `dioxus::fullstack` for SSR and server functions.
- **Type Safety:** Full Rust type safety throughout your component tree.

**Also Available For Leptos:**

Rust/UI is also available for [Leptos](https://leptos.dev/), another modern, reactive Rust web framework with excellent SSR and hydration support.

_Our goal is to provide the same high-quality component experience across multiple Rust web frameworks, giving you the flexibility to choose the right tool for your project._

## Installation

Install the UI CLI tool to get started:

```bash
cargo install ui-cli --force
```

See more details on **CLI section**.
