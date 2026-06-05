use dioxus::prelude::*;

use crate::Route;
use crate::components::command_bar::{CommandBarDialog, use_command_bar_provider};
use crate::components::navbar::Navbar;

#[component]
pub fn AppLayout() -> Element {
    use_command_bar_provider();

    rsx! {
        div { class: "min-h-screen bg-background",
            Navbar {}
            CommandBarDialog {}
            Outlet::<Route> {}
        }
    }
}
