+++
title = "Use Locks"
description = "Context hook for locking design params against randomization so each param can be toggled individually."
tags = ["utils"]
is_new = false
image = "/images/thumbnails/_placeholder.webp"
image_dark = "/images/thumbnails/_placeholder-dark.webp"
+++

<StaticUseLocks />

## Installation

<StaticInstallUseLocks />

## API

- **`UseLocks::init()`**: initializes and provides the lock context at the page root
- **`use_locks()`**: accesses the lock context from child components
- **`LockableParam`**: enum of all lockable design params

## Usage

```rust
use registry::hooks::use_locks::{LockableParam, UseLocks, use_locks};
```

```rust
#[component]
fn Page() -> Element {
    let _locks_provider = UseLocks::init();
    let mut locks = use_locks();

    rsx! {
        button {
            onclick: move |_| locks.toggle_lock(LockableParam::Font),
            "Toggle font lock"
        }
    }
}
```

## Methods

| Method | Returns | Description |
|--------|---------|-------------|
| `is_locked(param)` | `bool` | `true` when the param is locked |
| `toggle_lock(param)` | `()` | Toggle lock on/off |
| `lock(param)` | `()` | Lock explicitly |
| `unlock(param)` | `()` | Unlock explicitly |
| `can_randomize(param)` | `bool` | `true` when the param is not locked |
| `locked_params()` | `HashSet<LockableParam>` | All currently locked params |

## Examples

### Default

<StaticUseLocks />

## See Also

- [Use History](/hooks/use-history)
