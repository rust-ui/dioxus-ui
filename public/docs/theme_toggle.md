+++
title = "Theme Toggle"
+++

<DemoThemeToggle />

## Installation

Coming soon.

## Usage

```rust
use crate::ui::theme_toggle::ThemeToggle;
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
