use app_config::SeoMeta;
use dioxus::prelude::*;

use crate::components::doc_header::DocHeader;
use crate::components::toc::TocItem;
use crate::markdown::converter::{convert_md, extract_toc};
use crate::registry::hooks::{self, prev_next};

#[component]
pub fn HookPage(name: String) -> Element {
    let entry = hooks::find(&name);
    let (prev, next) = prev_next(&name);

    let mut toc: Signal<Vec<TocItem>> = use_context();
    let toc_items: Vec<TocItem> = entry.map(|e| extract_toc(e.body_md())).unwrap_or_default();
    use_effect(move || toc.set(toc_items.clone()));

    rsx! {
        div { class: "flex flex-col pt-4 mx-auto w-full min-h-screen px-4 max-w-[730px]",
            match entry {
                None => rsx! {
                    p { class: "text-muted-foreground", "Hook not found: {name}" }
                },
                Some(e) => rsx! {
                    SeoMeta {
                        title: format!("{} · Dioxus UI", e.title()),
                        description: e.description(),
                        canonical_url: format!("https://dioxus-ui.com/hooks/{}", e.slug),
                        og_type: "article".to_string(),
                    }
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
