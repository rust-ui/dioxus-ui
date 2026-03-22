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
        .map(|c| if c.is_alphanumeric() { c.to_ascii_lowercase() } else { '-' })
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
        div { class: "hidden xl:block w-52 shrink-0",
            div { class: "sticky top-16 pt-8 pb-4",
                p { class: "text-xs font-semibold text-foreground uppercase tracking-wider mb-3",
                    "On This Page"
                }
                nav { class: "flex flex-col gap-1",
                    for item in items {
                        a {
                            href: "#{item.id}",
                            class: if item.depth == 2 {
                                "text-sm text-muted-foreground hover:text-foreground transition-colors py-0.5"
                            } else {
                                "text-sm text-muted-foreground hover:text-foreground transition-colors py-0.5 pl-3"
                            },
                            "{item.text}"
                        }
                    }
                }
            }
        }
    }
}
