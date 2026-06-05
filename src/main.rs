use dioxus::prelude::*;

pub mod __registry__;
mod demos;
pub mod markdown;
pub mod registry;
mod routes;
mod ui;

use routes::app_layout::AppLayout;
use routes::component_page::ComponentPage;
use routes::docs_layout::DocsLayout;
use routes::home_page::Home;

const FAVICON: Asset = asset!("/public/favicon.ico");
const FAVICON_16: Asset = asset!("/public/icons/favicon-16x16.png");
const FAVICON_32: Asset = asset!("/public/icons/favicon-32x32.png");
const APPLE_TOUCH_ICON: Asset = asset!("/public/icons/apple-touch-icon.png");
const MANIFEST: Asset = asset!("/public/manifest.json");
const TAILWIND_CSS: Asset = asset!("/public/tailwind.css");
const LOCK_SCROLL_JS: Asset = asset!("/public/hooks/lock_scroll.js");

#[derive(Debug, Clone, Routable, PartialEq)]
#[rustfmt::skip]
enum Route {
    #[layout(AppLayout)]
        #[layout(DocsLayout)]
            #[route("/components/:name")]
            ComponentPage { name: String },
        #[end_layout]
        #[route("/")]
        Home {},
}

fn main() {
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    rsx! {
        document::Link { rel: "icon", href: FAVICON }
        document::Link { rel: "icon", r#type: "image/png", sizes: "16x16", href: FAVICON_16 }
        document::Link { rel: "icon", r#type: "image/png", sizes: "32x32", href: FAVICON_32 }
        document::Link { rel: "apple-touch-icon", href: APPLE_TOUCH_ICON }
        document::Link { rel: "manifest", href: MANIFEST }
        document::Link { rel: "stylesheet", href: TAILWIND_CSS }
        document::Script { src: LOCK_SCROLL_JS }
        Router::<Route> {}
    }
}
