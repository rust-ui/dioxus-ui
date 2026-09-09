use dioxus::prelude::*;

use crate::Route;
use crate::utils::page_transition::ScrollToTop;

#[component]
pub fn AppLayout() -> Element {
    rsx! {
        div { class: "flex flex-col h-full",
            ScrollToTop {}
            main { id: "data-scroll-target", class: "overflow-y-auto flex-1 overflow-x-clip",
                Outlet::<Route> {}
            }
        }
    }
}
