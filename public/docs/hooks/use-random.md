+++
title = "Use Random"
description = "Utility helpers for generating unique IDs and transition names for DOM elements and CSS animations."
tags = ["utils", "animation", "css"]
is_new = false
image = "/images/thumbnails/_placeholder.webp"
image_dark = "/images/thumbnails/_placeholder-dark.webp"
+++

<StaticUseRandom />

## Installation

<StaticInstallUseRandom />

## Usage

```rust
use registry::hooks::use_random::{use_random_id, use_random_id_for, use_random_transition_name};
```

```rust
let id = use_random_id();
let checkbox_id = use_random_id_for("checkbox");
let transition_name = use_random_transition_name();
```

## API

- `use_random_id() -> String`: returns a unique generated id
- `use_random_id_for(element) -> String`: prefixes the id with an element name
- `use_random_transition_name() -> String`: returns an inline CSS transition-name style

## Examples

### Random ID Generation

<StaticUseRandom />

## See Also

- [Use Copy Clipboard](/hooks/use-copy-clipboard)
