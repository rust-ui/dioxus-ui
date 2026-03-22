use dioxus::prelude::*;

use crate::demos::demo_button::DemoButton;
use crate::demos::demo_button_variants::DemoButtonVariants;
use crate::markdown::converter::{convert_md, MdComponents, MdNodeProps};
use crate::registry::{self, REGISTRY};

fn components_for(slug: &str) -> MdComponents {
    let mut c = MdComponents::new();
    match slug {
        "button" => {
            c.add("demo-button", |_: MdNodeProps| rsx! { DemoButton {} });
            c.add("demo-button-variants", |_: MdNodeProps| rsx! { DemoButtonVariants {} });
        }
        _ => {}
    }
    c
}

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
                        {convert_md(e.body_md(), &components_for(e.slug))}
                    },
                }
            }
        }
    }
}
