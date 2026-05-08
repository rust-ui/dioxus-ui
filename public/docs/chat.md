+++
title = "Chat"
description = "Chat UI layout components for building messaging interfaces."
+++

<DemoChat />

## Installation

Coming soon.

## Usage

```rust
use crate::ui::chat::{
    ChatCard, ChatHeader, ChatBody, ChatFooter,
    ChatMessageList, ChatMessageReceived, ChatMessageSent,
    ChatMessageAvatar, ChatMessageBubble, ChatMessageContent, ChatMessageTime,
};
```

```rust
rsx! {
    ChatCard {
        ChatHeader { class: "p-4", "Assistant" }
        ChatBody { class: "p-4",
            ChatMessageList {
                ChatMessageReceived {
                    ChatMessageBubble { class: "bg-muted",
                        ChatMessageContent { "Hello!" }
                    }
                }
                ChatMessageSent {
                    ChatMessageBubble { class: "bg-primary text-primary-foreground",
                        ChatMessageContent { "Hi there!" }
                    }
                }
            }
        }
        ChatFooter { class: "p-4", "Input here" }
    }
}
```

## See Also

- [Avatar](/components/avatar)
- [Input](/components/input)
- [Card](/components/card)
