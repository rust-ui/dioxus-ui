+++
title = "Dropzone"
description = "Rust/UI component for drag-and-drop file uploads with list and grid views."
+++

<DemoDropzoneToggle />

## Installation

Coming soon.

## Usage

```rust
use crate::ui::dropzone::{Dropzone, DropzoneArea, DropzoneFileList, DropzoneHint, DropzoneIcon, DropzoneLabel};
```

```rust
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

Files appear as a row list with name, size, and remove button.

<DemoDropzone />

### Grid view

Files appear as a card grid with image and video previews on hover.

<DemoDropzoneGrid />

### List / grid toggle

Switch between list and grid view using the toggle buttons.

<DemoDropzoneToggle />

## See Also

- [Input](/components/input)
- [Drag and Drop](/components/drag-and-drop)
