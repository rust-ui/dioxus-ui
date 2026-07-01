+++
title = "Message"
description = "Displays a message in a conversation, with optional avatar, header, footer, and alignment."
+++

<DemoMessage />

## Installation

Coming soon.

## Usage

```rust
use crate::ui::message::{Message, MessageAvatar, MessageContent, MessageHeader, MessageFooter, MessageGroup};
```

```rust
rsx! {
    Message {
        MessageAvatar {
            Avatar {
                AvatarImage { src: "/avatars/02.png", alt: "@avatar" }
                AvatarFallback { "CN" }
            }
        }
        MessageContent {
            Bubble {
                BubbleContent { "How can I help you today?" }
            }
        }
    }
}
```

## Composition

```
Message
├── MessageAvatar
└── MessageContent
    ├── MessageHeader
    ├── Bubble
    └── MessageFooter
```

Use `MessageGroup` to stack consecutive messages from the same sender:

```
MessageGroup
├── Message
└── Message
```

## Examples

### Avatar

Use `MessageAvatar` to render an avatar next to the message. Set `align: MessageAlign::End` on the message to align the avatar to the end.

<DemoMessageAvatar />

### Group

Use `MessageGroup` to stack consecutive messages from the same sender. Render an empty `MessageAvatar` on the earlier messages to keep them aligned.

<DemoMessageGroup />

### Header and Footer

Use `MessageHeader` for a sender name and `MessageFooter` for metadata such as a delivery or read status.

<DemoMessageHeaderFooter />

### Actions

Place message-level actions in `MessageFooter`, such as copy, retry, or feedback buttons.

<DemoMessageActions />

### Attachment

<DemoMessageAttachment />

## See Also

- [Bubble](/components/bubble)
- [Avatar](/components/avatar)
