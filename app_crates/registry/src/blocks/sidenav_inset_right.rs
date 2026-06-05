/* TODO: requires sidenav UI component */
use dioxus::prelude::*;

#[component]
pub fn SidenavInsetRight() -> Element {
    rsx! {
        div { class: "flex flex-col min-h-svh",
            header { class: "flex items-center h-14 border-b px-6",
                span { class: "font-semibold", "Sidenav Inset Right" }
            }
            main { class: "flex flex-1",
                div { class: "flex-1 p-6",
                    "Main content area"
                }
            }
        }
    }
}
