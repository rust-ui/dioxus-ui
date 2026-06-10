use dioxus::prelude::*;

use crate::Route;
use crate::components::navigation::header_docs::HeaderDocs;
use crate::domain::workflows::workflows_hero::WorkflowsHero;

#[component]
pub fn WorkflowsLayout() -> Element {
    rsx! {
        HeaderDocs {}

        div { "data-name": "__WorkflowsLayout", class: "container flex flex-col gap-20 pb-14",
            WorkflowsHero {}

            div { class: "flex flex-col gap-20 page__fade",
                Outlet::<Route> {}
            }
        }
    }
}
