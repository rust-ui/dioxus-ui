use app_config::SeoMeta;
use dioxus::prelude::*;
use icons::{ChevronLeft, ChevronRight};

use crate::components::doc_header::DocHeader;
use crate::components::footer_layout::FooterLayout;
use crate::components::newsletter_signup::NewsletterSignup;
use crate::components::toc::TocItem;
use crate::markdown::converter::{convert_md, extract_toc};
use crate::registry::hooks::{self, prev_next};
use crate::registry::types::RegistryEntry;

#[component]
pub fn HookPage(name: String) -> Element {
    let entry = hooks::find(&name);
    let (prev, next) = prev_next(&name);

    let mut toc: Signal<Vec<TocItem>> = use_context();
    let toc_items: Vec<TocItem> = entry.map(|e| extract_toc(e.body_md())).unwrap_or_default();
    use_effect(move || toc.set(toc_items.clone()));

    rsx! {
        div { class: "flex flex-col pt-4 mx-auto w-full min-h-screen px-3 md:px-4 max-w-[730px]",
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
                        raw: e.raw,
                        slug: e.slug,
                        section: "hooks".to_string(),
                        prev,
                        next,
                    }
                    {convert_md(e.body_md(), &(e.components)())}
                    div { class: "mt-14 mb-6",
                        NewsletterSignup {}
                    }
                    HookBottomNav { prev, next }
                    FooterLayout {}
                },
            }
        }
    }
}

#[component]
fn HookBottomNav(prev: Option<&'static RegistryEntry>, next: Option<&'static RegistryEntry>) -> Element {
    rsx! {
        div { class: "flex justify-between items-center mt-8",
            if let Some(p) = prev {
                a {
                    href: "/hooks/{p.slug}",
                    class: "py-0 px-2 h-8 inline-flex justify-center items-center text-sm font-medium whitespace-nowrap rounded-md transition-colors w-fit focus-visible:outline-hidden focus-visible:ring-1 focus-visible:ring-ring border bg-background border-input hover:bg-accent hover:text-accent-foreground z-50",
                    ChevronLeft {}
                    span { "{p.title()}" }
                }
            } else {
                div {}
            }
            if let Some(n) = next {
                a {
                    href: "/hooks/{n.slug}",
                    class: "py-0 px-2 h-8 inline-flex justify-center items-center text-sm font-medium whitespace-nowrap rounded-md transition-colors w-fit focus-visible:outline-hidden focus-visible:ring-1 focus-visible:ring-ring border bg-background border-input hover:bg-accent hover:text-accent-foreground z-50",
                    span { "{n.title()}" }
                    ChevronRight {}
                }
            } else {
                div {}
            }
        }
    }
}
