use dioxus::prelude::*;

use crate::Route;
use crate::components::command_bar::{CommandBarDialog, use_command_bar_provider};
#[component]
pub fn AppLayout() -> Element {
    use_command_bar_provider();

    rsx! {
        div { class: "flex flex-col h-full",
            CommandBarDialog {}
            main { id: "data-scroll-target", class: "overflow-y-auto flex-1 overflow-x-clip",
                Outlet::<Route> {}
            }
        }
    }
}
