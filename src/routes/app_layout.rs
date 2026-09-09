use dioxus::prelude::*;

use crate::Route;
use crate::components::command_bar::{CommandBarDialog, use_command_bar_provider};
use crate::utils::page_transition::ScrollToTop;

#[component]
pub fn AppLayout() -> Element {
    use_command_bar_provider();

    rsx! {
        div { class: "flex flex-col h-full",
            ScrollToTop {}
            CommandBarDialog {}
            main { id: "data-scroll-target", class: "overflow-y-auto flex-1 overflow-x-clip",
                Outlet::<Route> {}
            }
        }
    }
}
