+++
title = "Use Copy Clipboard"
description = "A reactive hook that copies text to the clipboard and exposes a temporary copied state for feedback UI."
tags = ["utils"]
is_new = false
image = "/images/thumbnails/_placeholder.webp"
image_dark = "/images/thumbnails/_placeholder-dark.webp"
+++

<StaticUseCopyToClipboard />

## Installation

<StaticInstallUseCopyClipboard />

## Usage

```rust
use registry::hooks::use_copy_clipboard::use_copy_clipboard;
```

```rust
let (copy_to_clipboard, copied) = use_copy_clipboard(Some(2000));

copy_to_clipboard("https://rust-ui.com/docs/components/input");
```

## Return Value

- `copy_to_clipboard(&str)`: copies the provided text
- `copied() -> bool`: returns `true` for a short period after a successful copy

## Examples

### Default

<StaticUseCopyToClipboard />

## See Also

- [Use Random](/hooks/use-random)
- [Input](/components/input)
