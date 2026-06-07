use dioxus::prelude::*;

use crate::Route;
use crate::components::navigation::header_docs::HeaderDocs;
use crate::components::sidenav::Sidenav;
use crate::components::toc::{TableOfContents, TocItem};

#[component]
pub fn DocsLayout() -> Element {
    let toc = use_context_provider(|| Signal::new(Vec::<TocItem>::new()));

    rsx! {
        HeaderDocs {}
        div { class: "flex-1",
            div { class: "container mx-auto flex items-start",
                Sidenav {}
                Outlet::<Route> {}
                TableOfContents { items: toc() }
            }
        }
    }
}
