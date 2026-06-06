use dioxus::prelude::*;
use registry::ui::avatar::{Avatar, AvatarFallback};
use registry::ui::card::{Card, CardContent, CardDescription, CardHeader, CardTitle};
use registry::ui::select_native::SelectNative;

const ROLES: [&str; 2] = ["Owner", "Member"];

#[component]
pub fn CardTeamMembers() -> Element {
    rsx! {
        Card {
            CardHeader {
                CardTitle { "Team Members" }
                CardDescription { "Invite your team members to collaborate." }
            }
            CardContent { class: "grid gap-6",
                div { class: "flex justify-between items-center space-x-4",
                    div { class: "flex items-center space-x-4",
                        Avatar {
                            AvatarFallback { "OM" }
                        }
                        div {
                            p { class: "text-sm font-medium leading-none", "Sofia Davis" }
                            p { class: "text-sm text-muted-foreground", "m@example.com" }
                        }
                    }
                    SelectNative { class: "ml-auto w-[100px]",
                        {ROLES.iter().map(|role| rsx! {
                            option { key: "{role}", value: "{role}", selected: *role == ROLES[0], "{role}" }
                        })}
                    }
                }
                div { class: "flex justify-between items-center space-x-4",
                    div { class: "flex items-center space-x-4",
                        Avatar {
                            AvatarFallback { "JL" }
                        }
                        div {
                            p { class: "text-sm font-medium leading-none", "Jackson Lee" }
                            p { class: "text-sm text-muted-foreground", "p@example.com" }
                        }
                    }
                    SelectNative { class: "ml-auto w-[100px]",
                        {ROLES.iter().map(|role| rsx! {
                            option { key: "{role}", value: "{role}", selected: *role == ROLES[1], "{role}" }
                        })}
                    }
                }
                div { class: "flex justify-between items-center space-x-4",
                    div { class: "flex items-center space-x-4",
                        Avatar {
                            AvatarFallback { "IN" }
                        }
                        div {
                            p { class: "text-sm font-medium leading-none", "Isabella Nguyen" }
                            p { class: "text-sm text-muted-foreground", "i@example.com" }
                        }
                    }
                    SelectNative { class: "ml-auto w-[100px]",
                        {ROLES.iter().map(|role| rsx! {
                            option { key: "{role}", value: "{role}", selected: *role == ROLES[1], "{role}" }
                        })}
                    }
                }
            }
        }
    }
}
