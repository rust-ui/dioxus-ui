use app_config::SeoMeta;
use dioxus::prelude::*;
use icons::{ChevronLeft, ChevronRight};

use crate::__registry__::static_md_registry::{MyMd, docs_component_prev_next, find_docs_component_entry};
use crate::components::doc_header::DocHeader;
use crate::components::footer_layout::FooterLayout;
use crate::components::newsletter_signup::NewsletterSignup;
use crate::registry::types::RegistryEntry;
use crate::routes::page_not_found::PageNotFound;

#[component]
pub fn ComponentPage(name: String) -> Element {
    let entry = find_docs_component_entry(&name);
    let (prev, next) = docs_component_prev_next(&name);


    rsx! {
        div { class: "flex flex-col pt-4 mx-auto w-full min-h-screen px-3 md:px-4 max-w-[730px]",
            match entry {
                None => rsx! {
                    PageNotFound { segments: vec!["docs".into(), "components".into(), name.clone()] }
                },
                Some(e) => rsx! {
                    SeoMeta {
                        title: format!("{} · Rust UI", e.title()),
                        description: e.description(),
                        canonical_url: format!("https://dioxus-ui.com/docs/components/{}", e.slug),
                        og_type: "article".to_string(),
                    }
                    DocHeader {
                        title: e.title(),
                        description: e.description(),
                        tags: e.tags.to_vec(),
                        raw: e.raw,
                        slug: e.slug,
                        section_label: "Components".to_string(),
                        base_path: "/docs/components".to_string(),
                        prev,
                        next,
                    }
                    MyMd { raw: e.raw }
                    div { class: "mt-14 mb-6",
                        NewsletterSignup {}
                    }
                    DocBottomNav { prev, next }
                    FooterLayout {}
                },
            }
        }
    }
}

#[component]
fn DocBottomNav(prev: Option<&'static RegistryEntry>, next: Option<&'static RegistryEntry>) -> Element {
    rsx! {
        div { class: "flex justify-between items-center mt-8",
            if let Some(p) = prev {
                a {
                    href: "/docs/components/{p.slug}",
                    class: "py-0 px-2 h-8 inline-flex justify-center items-center text-sm font-medium whitespace-nowrap rounded-md transition-colors w-fit focus-visible:outline-hidden focus-visible:ring-1 focus-visible:ring-ring border bg-background border-input hover:bg-accent hover:text-accent-foreground z-50",
                    ChevronLeft {}
                    span { "{p.title()}" }
                }
            } else {
                div {}
            }
            if let Some(n) = next {
                a {
                    href: "/docs/components/{n.slug}",
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
