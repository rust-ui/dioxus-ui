+++
title = "Use History"
description = "Undo/redo history stack for URL-based state, with keyboard shortcuts (⌘Z / ⌘⇧Z)."
tags = ["utils"]
is_new = false
image = "/images/thumbnails/_placeholder.webp"
image_dark = "/images/thumbnails/_placeholder-dark.webp"
+++

<StaticUseHistory />

## Installation

<StaticInstallUseHistory />

## API

- **`UseHistory::init()`**: initializes and provides the history context at the page root
- **`use_history()`**: accesses the history context from child components

## Usage

```rust
use registry::hooks::use_history::{UseHistory, use_history};
```

```rust
#[component]
fn Page() -> Element {
    let _history_provider = UseHistory::init();
    let history = use_history();

    rsx! {
        button { onclick: move |_| history.go_back(), "Undo" }
        button { onclick: move |_| history.go_forward(), "Redo" }
    }
}
```

## Methods

| Method | Returns | Description |
|--------|---------|-------------|
| `push(url)` | `()` | Append a URL, truncating any forward history |
| `go_back()` | `()` | Navigate one step back |
| `go_forward()` | `()` | Navigate one step forward |
| `can_go_back()` | `bool` | `true` when undo is available |
| `can_go_forward()` | `bool` | `true` when redo is available |
| `position()` | `usize` | Current 1-based position in stack |
| `total()` | `usize` | Total number of states |

## Examples

### Default

<StaticUseHistory />

## See Also

- [Use Media Query](/hooks/use-media-query)
- [Use Is Mobile](/hooks/use-is-mobile)
