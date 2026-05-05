use dioxus::prelude::*;

#[derive(Clone, PartialEq)]
pub struct TocItem {
    pub id: String,
    pub text: String,
    pub depth: u8, // 2 = h2, 3 = h3
}

/// Slugify heading text to a valid HTML id: lowercase, spaces → dashes, strip non-alphanumeric
pub fn slugify(text: &str) -> String {
    text.chars()
        .map(|c| {
            if c.is_alphanumeric() {
                c.to_ascii_lowercase()
            } else {
                '-'
            }
        })
        .collect::<String>()
        .split('-')
        .filter(|s| !s.is_empty())
        .collect::<Vec<_>>()
        .join("-")
}

#[component]
pub fn TableOfContents(items: Vec<TocItem>) -> Element {
    if items.is_empty() {
        return rsx! {};
    }

    rsx! {
        aside { class: "hidden sticky lg:block top-16 h-[calc(100vh-3.5rem)] w-[250px] shrink-0",
            div { class: "flex flex-col pl-4 h-full",
                div {
                    h4 { class: "mb-4 text-sm font-medium", "On This Page" }
                    nav {
                        "aria-label": "Table of contents",
                        class: "overflow-y-auto max-h-[calc(100vh-16rem)] scroll-smooth",
                        ul { class: "pb-4 space-y-1.5",
                            for item in items {
                                li {
                                    a {
                                        href: "#{item.id}",
                                        class: if item.depth == 2 {
                                            "block text-sm text-muted-foreground hover:text-foreground transition-colors"
                                        } else {
                                            "block text-sm text-muted-foreground hover:text-foreground transition-colors pl-4"
                                        },
                                        "{item.text}"
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}
