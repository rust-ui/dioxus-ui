use dioxus::prelude::*;

use crate::ui::command_bar::{CommandBarDialog, use_command_bar_provider};
use crate::ui::navbar::Navbar;
use crate::Route;

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
