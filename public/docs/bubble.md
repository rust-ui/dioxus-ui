+++
title = "Bubble"
description = "Displays a chat message bubble with multiple style variants and an optional reactions overlay."
+++

<DemoBubble />

## Installation

Coming soon.

## Usage

```rust
use crate::ui::bubble::{Bubble, BubbleContent, BubbleGroup, BubbleReactions, BubbleVariant};
```

```rust
rsx! {
    Bubble {
        BubbleContent { "I checked the registry output and removed the stale route." }
        BubbleReactions {
            span { "👍" }
        }
    }
}
```

## Composition

```
Bubble
├── BubbleContent          (renders as <div>, <a>, or <button>)
└── BubbleReactions        (absolute overlay — emoji / reaction buttons)

BubbleGroup                (stacks multiple bubbles with gap)
```

## Examples

### Variants

Use `variant` to change the visual treatment of the bubble.

<DemoBubbleVariants />

### Alignment

Use `align` on `Bubble` to align the bubble to the start or end of the conversation.

<DemoBubbleAlignment />

### Bubble Group

Use `BubbleGroup` to group consecutive bubbles from the same sender.

<DemoBubbleGroup />

### Links and Buttons

Pass `onclick` to `BubbleContent` to render it as a `<button>`, or `href` to render it as an `<a>`.

<DemoBubbleLinkButton />

### Reactions

Use `BubbleReactions` for bubble reactions. Use `side` (`Top` / `Bottom`) and `align` (`Start` / `End`) to position the overlay.

<DemoBubbleReactions />

### Show More / Collapsible

Compose `BubbleContent` with `Collapsible` for show-more / show-less long content.

<DemoBubbleCollapsible />

### Tooltip

Wrap a bubble in a `Tooltip` to reveal metadata on hover.

<DemoBubbleTooltip />

### Popover

Pair a bubble with a `Popover` to surface more information on demand.

<DemoBubblePopover />

## See Also

- [Message](/components/message)
- [Attachment](/components/attachment)
