use ::registry::hooks::use_theme_mode::ThemeMode;
use dioxus::prelude::*;

pub mod __registry__;
pub mod components;
mod domain;
pub mod markdown;
pub mod registry;
mod routes;

use domain::blocks::routing::blocks_layout::BlocksLayout;
use domain::blocks::routing::blocks_pages::{
    FaqBlocks, FootersBlocks, HeadersBlocks, IntegrationsBlocks, LoginBlocks, SidenavBlocks,
};
use domain::charts::routing::charts_layout::ChartsLayout;
use domain::charts::routing::charts_pages::{
    AreaChartPage, BarChartPage, LineChartPage, PieChartPage, RadarChartPage, RadialChartPage,
};
use domain::create::page_create::PageCreate;
use domain::test::routing::test_layout::TestLayout;
use domain::test::routing::test_pages::TestPage;
use domain::workflows::routing::workflows_layout::WorkflowsLayout;
use domain::workflows::routing::workflows_pages::WorkflowsPage;
use routes::app_layout::AppLayout;
use routes::component_page::ComponentPage;
use routes::docs_layout::DocsLayout;
use routes::home_layout::HomeLayout;
use routes::home_page::Home;
use routes::hook_page::HookPage;
use routes::page_icons::PageIcons;

const FAVICON: Asset = asset!("/public/favicon.ico");
const FAVICON_16: Asset = asset!("/public/icons/favicon-16x16.png");
const FAVICON_32: Asset = asset!("/public/icons/favicon-32x32.png");
const APPLE_TOUCH_ICON: Asset = asset!("/public/icons/apple-touch-icon.png");
const MANIFEST: Asset = asset!("/public/manifest.json");
const TAILWIND_CSS: Asset = asset!("/assets/tailwind.css");
const LOCK_SCROLL_JS: Asset = asset!("/public/hooks/lock_scroll.js");
const CHART_INIT_JS: Asset = asset!("/public/app_components/chart_init.js");

#[derive(Debug, Clone, Routable, PartialEq)]
#[rustfmt::skip]
enum Route {
    #[layout(AppLayout)]
        #[layout(HomeLayout)]
            #[route("/")]
            Home {},
        #[end_layout]
        #[layout(DocsLayout)]
            #[route("/components/:name")]
            ComponentPage { name: String },
            #[route("/hooks/:name")]
            HookPage { name: String },
        #[end_layout]
        #[layout(BlocksLayout)]
            #[redirect("/blocks", || Route::LoginBlocks {})]
            #[route("/blocks/login")]
            LoginBlocks {},
            #[route("/blocks/sidenav")]
            SidenavBlocks {},
            #[route("/blocks/headers")]
            HeadersBlocks {},
            #[route("/blocks/footers")]
            FootersBlocks {},
            #[route("/blocks/faq")]
            FaqBlocks {},
            #[route("/blocks/integrations")]
            IntegrationsBlocks {},
        #[end_layout]
        #[layout(ChartsLayout)]
            #[redirect("/charts", || Route::AreaChartPage {})]
            #[route("/charts/area-chart")]
            AreaChartPage {},
            #[route("/charts/bar-chart")]
            BarChartPage {},
            #[route("/charts/line-chart")]
            LineChartPage {},
            #[route("/charts/pie-chart")]
            PieChartPage {},
            #[route("/charts/radar-chart")]
            RadarChartPage {},
            #[route("/charts/radial-chart")]
            RadialChartPage {},
        #[end_layout]
        #[layout(WorkflowsLayout)]
            #[route("/workflows")]
            WorkflowsPage {},
        #[end_layout]
        #[layout(TestLayout)]
            #[route("/test-page")]
            TestPage {},
        #[end_layout]
        #[route("/icons")]
        PageIcons {},
        #[route("/create")]
        PageCreate {},
}

fn main() {
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    let theme_mode = ThemeMode::init();

    // TODO: replace with <Html class=...> once Dioxus supports reactive html-element attributes (like leptos_meta `<Html {..} class=...>`)
    use_effect(move || {
        let is_dark = theme_mode.is_dark();
        spawn(async move {
            let js = if is_dark {
                "document.documentElement.classList.add('dark');"
            } else {
                "document.documentElement.classList.remove('dark');"
            };
            dioxus::document::eval(js).await.ok();
        });
    });

    rsx! {
        document::Title { "Rust/UI" }
        // TODO: Dioxus injects a <div id="main"> between <body> and the App component.
        // Leptos doesn't have this wrapper, so the h-full chain works natively:
        //   html(100dvh) → body(h-full) → AppWrapper div(h-full) → main(flex-1 overflow-y-auto)
        // In Dioxus the chain is broken because #main has no height, so flex-1 on <main> collapses.
        // Ideal fix: find a way to pass h-full to the Dioxus mount div without a style tag.
        // For now we inject it inline so it wins over any stylesheet ordering issues.
        document::Style { "#main {{ height: 100%; }}" }
        document::Link { rel: "icon", r#type: "image/png", sizes: "32x32", href: FAVICON_32 }
        document::Link { rel: "icon", r#type: "image/png", sizes: "16x16", href: FAVICON_16 }
        document::Link { rel: "icon", href: FAVICON }
        document::Link { rel: "apple-touch-icon", href: APPLE_TOUCH_ICON }
        document::Link { rel: "manifest", href: MANIFEST }
        document::Stylesheet { href: TAILWIND_CSS }
        document::Script { src: LOCK_SCROLL_JS }
        document::Script { src: CHART_INIT_JS }
        Router::<Route> {}
    }
}
