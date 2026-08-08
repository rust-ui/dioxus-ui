use app_config::SeoMeta;
use dioxus::prelude::*;
use icons::{ChevronLeft, ChevronRight};

use crate::components::doc_header::DocHeader;
use crate::components::footer_layout::FooterLayout;
use crate::components::newsletter_signup::NewsletterSignup;
use crate::components::toc::TocItem;
use crate::markdown::converter::{convert_md, extract_toc};
use crate::registry::changelog::CHANGELOG;
use crate::registry::figma::FIGMA;
use crate::registry::installation::INSTALLATION;
use crate::registry::introduction::INTRODUCTION;
use crate::registry::rtl::RTL;
use crate::registry::types::RegistryEntry;

static GET_STARTED: &[&RegistryEntry] = &[&INTRODUCTION, &INSTALLATION, &CHANGELOG, &FIGMA, &RTL];

fn find(slug: &str) -> Option<&'static RegistryEntry> {
    GET_STARTED.iter().copied().find(|e| e.slug == slug)
}

fn prev_next(slug: &str) -> (Option<&'static RegistryEntry>, Option<&'static RegistryEntry>) {
    let pos = GET_STARTED.iter().position(|e| e.slug == slug);
    match pos {
        None => (None, None),
        Some(i) => (
            if i > 0 { Some(GET_STARTED[i - 1]) } else { None },
            if i + 1 < GET_STARTED.len() { Some(GET_STARTED[i + 1]) } else { None },
        ),
    }
}

#[component]
pub fn DocsPage(name: String) -> Element {
    let entry = find(&name);
    let (prev, next) = prev_next(&name);

    let mut toc: Signal<Vec<TocItem>> = use_context();
    let toc_items: Vec<TocItem> = entry.map(|e| extract_toc(e.body_md())).unwrap_or_default();
    use_effect(move || toc.set(toc_items.clone()));

    rsx! {
        div { class: "flex flex-col pt-4 mx-auto w-full min-h-screen px-3 md:px-4 max-w-[730px]",
            match entry {
                None => rsx! {
                    p { class: "text-muted-foreground", "Page not found: {name}" }
                },
                Some(e) => rsx! {
                    SeoMeta {
                        title: format!("{} · Rust UI", e.title()),
                        description: e.description(),
                        canonical_url: format!("https://dioxus-ui.com/docs/{}", e.slug),
                        og_type: "article".to_string(),
                    }
                    DocHeader {
                        title: e.title(),
                        description: e.description(),
                        tags: e.tags.to_vec(),
                        raw: e.raw,
                        slug: e.slug,
                        section: "docs".to_string(),
                        prev,
                        next,
                    }
                    {convert_md(e.body_md(), &(e.components)())}
                    div { class: "mt-14 mb-6",
                        NewsletterSignup {}
                    }
                    DocsBottomNav { prev, next }
                    FooterLayout {}
                },
            }
        }
    }
}

#[component]
fn DocsBottomNav(prev: Option<&'static RegistryEntry>, next: Option<&'static RegistryEntry>) -> Element {
    rsx! {
        div { class: "flex justify-between items-center mt-8",
            if let Some(p) = prev {
                a {
                    href: "/docs/{p.slug}",
                    class: "py-0 px-2 h-8 inline-flex justify-center items-center text-sm font-medium whitespace-nowrap rounded-md transition-colors w-fit focus-visible:outline-hidden focus-visible:ring-1 focus-visible:ring-ring border bg-background border-input hover:bg-accent hover:text-accent-foreground z-50",
                    ChevronLeft {}
                    span { "{p.title()}" }
                }
            } else {
                div {}
            }
            if let Some(n) = next {
                a {
                    href: "/docs/{n.slug}",
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
