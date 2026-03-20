use dioxus::prelude::*;

use crate::ui::sidenav::Sidenav;
use crate::Route;

#[component]
pub fn DocsLayout() -> Element {
    rsx! {
        div { class: "flex min-h-screen bg-background",
            Sidenav {}
            main { class: "flex-1 p-8 overflow-y-auto",
                Outlet::<Route> {}
            }
        }
    }
}
