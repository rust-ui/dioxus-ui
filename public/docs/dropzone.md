+++
title = "Dropzone"
description = "File drop area with drag-and-drop, click-to-browse, list/grid toggle, and file validation."
tags = []
is_new = false
image = "/images/thumbnails/_placeholder.webp"
image_dark = "/images/thumbnails/_placeholder-dark.webp"
+++

<DemoDropzoneToggle />

## Installation

Copy `ui/dropzone.rs` into your project and add `pub mod dropzone;` to your `ui/mod.rs`.

Requires: `tw_merge`, `icons`, `web-sys` (wasm32), `wasm-bindgen`.

## Usage

```rust
use crate::ui::dropzone::{
    Dropzone, DropzoneArea, DropzoneFileList, DropzoneHint,
    DropzoneIcon, DropzoneLabel, DropzoneViewToggle,
    DropzoneCtx, ViewMode,
};

rsx! {
    Dropzone {
        DropzoneArea {
            DropzoneIcon { Upload { class: "size-7" } }
            DropzoneLabel { "Drop files here" }
            DropzoneHint { "Up to 8 MB each" }
        }
        DropzoneFileList {}
    }
}
```

## Examples

### List view

<DemoDropzone />

### Grid view

<DemoDropzoneGrid />

### List / grid toggle

<DemoDropzoneToggle />

## Props

### `Dropzone`

| Prop | Type | Default | Description |
|---|---|---|---|
| `max_files` | `Option<usize>` | unlimited | Max number of files |
| `max_size_mb` | `Option<f64>` | unlimited | Max size per file in MB |
| `accept` | `Option<Vec<String>>` | all | Allowed MIME type prefixes |

## See Also

- [Input](/components/input)
- [Drag and Drop](/components/drag-and-drop)
