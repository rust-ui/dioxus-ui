use dioxus::prelude::*;

use crate::ui::avatar::{Avatar, AvatarFallback, AvatarImage};
use crate::ui::chat::{
    ChatBody, ChatCard, ChatFooter, ChatHeader, ChatMessageAvatar, ChatMessageBubble,
    ChatMessageContent, ChatMessageList, ChatMessageReceived, ChatMessageSent, ChatMessageTime,
};
use crate::ui::input::Input;

#[component]
pub fn DemoChat() -> Element {
    rsx! {
        ChatCard { class: "max-w-sm h-[400px]",
            ChatHeader { class: "p-4 gap-3",
                Avatar { class: "size-8",
                    AvatarImage { src: "https://github.com/shadcn.png" }
                    AvatarFallback { "AI" }
                }
                div {
                    p { class: "text-sm font-medium", "Assistant" }
                    p { class: "text-xs text-muted-foreground", "Online" }
                }
            }
            ChatBody { class: "p-4",
                ChatMessageList {
                    ChatMessageReceived {
                        ChatMessageAvatar { class: "mr-2 size-8",
                            Avatar { class: "size-8",
                                AvatarImage { src: "https://github.com/shadcn.png" }
                                AvatarFallback { "AI" }
                            }
                        }
                        div {
                            ChatMessageBubble { class: "bg-muted",
                                ChatMessageContent { "Hey! How can I help you today?" }
                            }
                            ChatMessageTime { class: "text-muted-foreground", "10:30 AM" }
                        }
                    }
                    ChatMessageSent {
                        div {
                            ChatMessageBubble { class: "bg-primary text-primary-foreground",
                                ChatMessageContent { "I'd like to learn about Dioxus!" }
                            }
                            ChatMessageTime { class: "text-muted-foreground", "10:31 AM" }
                        }
                    }
                    ChatMessageReceived {
                        ChatMessageAvatar { class: "mr-2 size-8",
                            Avatar { class: "size-8",
                                AvatarImage { src: "https://github.com/shadcn.png" }
                                AvatarFallback { "AI" }
                            }
                        }
                        div {
                            ChatMessageBubble { class: "bg-muted",
                                ChatMessageContent {
                                    "Dioxus is a Rust UI framework for building cross-platform apps!"
                                }
                            }
                            ChatMessageTime { class: "text-muted-foreground", "10:31 AM" }
                        }
                    }
                }
            }
            ChatFooter { class: "p-4 gap-2",
                Input { placeholder: "Type a message..." }
            }
        }
    }
}
