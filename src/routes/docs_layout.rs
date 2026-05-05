use dioxus::prelude::*;

use crate::Route;
use crate::ui::footer::Footer;
use crate::ui::sidenav::Sidenav;
use crate::ui::toc::{TableOfContents, TocItem};

#[component]
pub fn DocsLayout() -> Element {
    // Provide a writable signal — child pages write their headings into it
    let toc = use_context_provider(|| Signal::new(Vec::<TocItem>::new()));

    rsx! {
        div { class: "flex flex-col min-h-screen bg-background",
            div { class: "flex-1",
                div { class: "container mx-auto flex items-start",
                    Sidenav {}
                    Outlet::<Route> {}
                    TableOfContents { items: toc() }
                }
            }
            Footer {}
        }
    }
}
