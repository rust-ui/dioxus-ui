use dioxus::prelude::*;

use crate::Route;
use crate::components::navigation::header_docs::HeaderDocs;
use crate::domain::blocks::components::blocks_hero::BlocksHero;

#[component]
pub fn BlocksLayout() -> Element {
    rsx! {
        HeaderDocs {}

        div { "data-name": "__BlockLayout", class: "container flex flex-col gap-20 pb-14",
            BlocksHero {}

            div { class: "flex flex-col gap-20 page__fade",
                Outlet::<Route> {}
            }
        }
    }
}
