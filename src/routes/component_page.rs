use dioxus::prelude::*;

use crate::markdown::converter::{convert_md, extract_toc};
use crate::registry::{self, prev_next};
use crate::ui::doc_header::DocHeader;
use crate::ui::toc::TocItem;

#[component]
pub fn ComponentPage(name: String) -> Element {
    let entry = registry::find(&name);
    let (prev, next) = prev_next(&name);

    // Write headings into the DocsLayout-provided signal so the TOC sidebar updates
    let mut toc: Signal<Vec<TocItem>> = use_context();
    let toc_items: Vec<TocItem> = entry.map(|e| extract_toc(e.body_md())).unwrap_or_default();
    use_effect(move || toc.set(toc_items.clone()));

    rsx! {
        div { "data-name": "Preview", class: "flex flex-col pt-4 mx-auto w-full min-h-screen px-4 max-w-[730px]",
            match entry {
                None => rsx! {
                    p { class: "text-muted-foreground", "Component not found: {name}" }
                },
                Some(e) => rsx! {
                    DocHeader {
                        title: e.title(),
                        description: e.description(),
                        tags: e.tags.to_vec(),
                        prev,
                        next,
                    }
                    {convert_md(e.body_md(), &(e.components)())}
                },
            }
        }
    }
}
