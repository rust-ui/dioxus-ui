use dioxus::prelude::*;

use crate::ui::badge::{Badge, BadgeVariant};
use crate::ui::button::{Button, ButtonVariant};
use crate::Route;

#[component]
pub fn DocHeader(
    title: String,
    description: String,
    tags: Vec<&'static str>,
    prev: Option<&'static str>,
    next: Option<&'static str>,
) -> Element {
    rsx! {
        div { class: "mb-8",
            // 1. Breadcrumb
            nav { class: "flex items-center gap-1 text-sm text-muted-foreground mb-4",
                Link { to: Route::Home {}, "Home" }
                span { "/" }
                span { "Components" }
                span { "/" }
                span { class: "text-foreground font-medium", "{title}" }
            }

            // 2. Title row + prev/next
            div { class: "flex items-start justify-between gap-4 mt-2",
                h1 { class: "text-4xl font-bold tracking-tight", "{title}" }
                div { class: "flex gap-2 shrink-0 pt-1",
                    if let Some(p) = prev {
                        Link { to: Route::ComponentPage { name: p.to_string() },
                            Button { variant: ButtonVariant::Ghost, "← {p}" }
                        }
                    }
                    if let Some(n) = next {
                        Link { to: Route::ComponentPage { name: n.to_string() },
                            Button { variant: ButtonVariant::Ghost, "{n} →" }
                        }
                    }
                }
            }

            // 3. Description
            p { class: "mt-2 text-muted-foreground", "{description}" }

            // 4. Tags
            if !tags.is_empty() {
                div { class: "flex gap-2 items-center mt-3 mb-2",
                    for tag in tags.iter() {
                        Badge { variant: BadgeVariant::Outline, "{tag}" }
                    }
                }
            }
        }
    }
}
