use dioxus::prelude::*;

use crate::Route;
use crate::components::navigation::header_docs::HeaderDocs;
use crate::domain::charts::components::charts_hero::ChartsHero;

const APEXCHARTS_JS: Asset = asset!("/public/cdn/apexcharts.5.3.6.min.js");

#[component]
pub fn ChartsLayout() -> Element {
    rsx! {
        document::Script { src: APEXCHARTS_JS }

        HeaderDocs {}

        div { "data-name": "__ChartsLayout", class: "container flex flex-col gap-10",
            ChartsHero {}

            div { class: "page__fade",
                Outlet::<Route> {}
            }
        }
    }
}
