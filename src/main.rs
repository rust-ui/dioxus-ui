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
        document::Link { rel: "stylesheet", href: TAILWIND_CSS }
        document::Script { src: LOCK_SCROLL_JS }
        Router::<Route> {}
    }
}
