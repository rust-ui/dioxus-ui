use dioxus::prelude::*;

use crate::Route;
use crate::components::sidenav::Sidenav;
use crate::components::toc::{TableOfContents, TocItem};

#[component]
pub fn DocsLayout() -> Element {
    // Provide a writable signal — child pages write their headings into it
    let toc = use_context_provider(|| Signal::new(Vec::<TocItem>::new()));

    rsx! {
        div { class: "flex-1",
            div { class: "container mx-auto flex items-start",
                Sidenav {}
                Outlet::<Route> {}
                TableOfContents { items: toc() }
            }
        }
    }
}
