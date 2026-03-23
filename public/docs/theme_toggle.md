---
title: Theme Toggle
description: A button that toggles between light and dark mode, persisting the preference to localStorage.
tags: []
---

<DemoThemeToggle />

## Installation

Add `dioxus_ui` to your `Cargo.toml`:

```toml
dioxus_ui = "0.1"
```

## Usage

```rust
use dioxus_ui::theme_toggle::ThemeToggle;
```

```rust
rsx! {
    ThemeToggle {}
}
```

## How it works

On mount it reads `localStorage.getItem('darkmode')`, falling back to the system `prefers-color-scheme` preference. On toggle it adds/removes the `.dark` class on `<html>` and persists the choice.

## See Also

- [Button](/components/button)
