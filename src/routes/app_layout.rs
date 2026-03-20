use dioxus::prelude::*;

use crate::ui::navbar::Navbar;
use crate::Route;

#[component]
pub fn AppLayout() -> Element {
    rsx! {
        div { class: "min-h-screen bg-background",
            Navbar {}
            Outlet::<Route> {}
        }
    }
}
