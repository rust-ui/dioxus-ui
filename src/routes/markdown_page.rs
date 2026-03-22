use dioxus::prelude::*;

use crate::registry::{self, REGISTRY};

#[component]
pub fn MarkdownPage(slug: String) -> Element {
    let entry = registry::find(&slug);

    rsx! {
        div { class: "flex gap-8 max-w-4xl",
            // Sidebar
            nav { class: "w-40 shrink-0 flex flex-col gap-1 pt-1",
                p { class: "text-xs font-semibold text-muted-foreground uppercase tracking-wider mb-2", "Docs" }
                for item in REGISTRY.iter() {
                    Link {
                        class: "text-sm px-2 py-1 rounded hover:bg-accent transition-colors",
                        active_class: "bg-accent font-medium",
                        to: crate::Route::MarkdownPage { slug: item.slug.to_string() },
                        "{item.slug}"
                    }
                }
            }

            // Content
            div { class: "flex-1 min-w-0",
                match entry {
                    None => rsx! {
                        p { class: "text-muted-foreground", "Doc not found: {slug}" }
                    },
                    Some(e) => rsx! {
                        h1 { class: "text-2xl font-bold mb-1", "{e.title()}" }
                        p { class: "text-muted-foreground text-sm mb-6", "{e.description()}" }
                        div {
                            class: "prose prose-sm dark:prose-invert max-w-none",
                            dangerous_inner_html: "{e.body_html()}",
                        }
                    },
                }
            }
        }
    }
}
