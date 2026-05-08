use dioxus::prelude::*;

use crate::app::Route;
use crate::components::ui::button::{Button, ButtonVariant};
use crate::components::ui::card::{Card, CardContent, CardDescription, CardHeader, CardTitle};

#[component]
pub fn HomePage() -> Element {
    let navigator = use_navigator();

    rsx! {
        div { class: "flex flex-col gap-6 p-6 pt-[calc(env(safe-area-inset-top)+4rem)] sm:pt-6",
            div { class: "flex flex-col gap-2",
                h1 { class: "text-3xl font-bold tracking-tight", "Dioxus Fullstack" }
                p { class: "text-muted-foreground",
                    "Cross-platform starter — web, mobile, desktop from one codebase."
                }
            }

            div { class: "grid gap-4 sm:grid-cols-2",
                Card {
                    CardHeader {
                        CardTitle { "Items" }
                        CardDescription { "Browse and manage your items." }
                    }
                    CardContent {
                        Button {
                            onclick: move |_| { navigator.push(Route::ItemList {}); },
                            "View Items"
                        }
                    }
                }
                Card {
                    CardHeader {
                        CardTitle { "Settings" }
                        CardDescription { "Configure your preferences." }
                    }
                    CardContent {
                        Button {
                            variant: ButtonVariant::Outline,
                            onclick: move |_| { navigator.push(Route::Settings {}); },
                            "Open Settings"
                        }
                    }
                }
            }
        }
    }
}
