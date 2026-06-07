use dioxus::prelude::*;

use crate::Route;

#[component]
pub fn NavDesktop() -> Element {
    rsx! {
        div { class: "hidden gap-0 items-center md:flex",
            Link {
                class: "inline-flex items-center py-1.5 px-2.5 text-sm rounded-md hover:bg-accent",
                to: Route::Home {},
                "Dioxus/UI"
            }
            Link {
                class: "inline-flex items-center py-1.5 px-2.5 text-sm rounded-md hover:bg-accent",
                active_class: "bg-accent",
                to: Route::ComponentPage { name: "introduction".to_string() },
                "Docs"
            }
            Link {
                class: "inline-flex items-center py-1.5 px-2.5 text-sm rounded-md hover:bg-accent",
                active_class: "bg-accent",
                to: Route::ComponentPage { name: "button".to_string() },
                "Components"
            }
            Link {
                class: "inline-flex items-center py-1.5 px-2.5 text-sm rounded-md hover:bg-accent",
                active_class: "bg-accent",
                to: Route::HookPage { name: "use-copy-clipboard".to_string() },
                "Hooks"
            }
            Link {
                class: "inline-flex items-center py-1.5 px-2.5 text-sm rounded-md hover:bg-accent",
                active_class: "bg-accent",
                to: Route::PageIcons {},
                "Icons"
            }
            a {
                class: "inline-flex items-center py-1.5 px-2.5 text-sm rounded-md hover:bg-accent",
                href: "/blocks",
                "Blocks"
            }
            Link {
                class: "inline-flex items-center py-1.5 px-2.5 text-sm rounded-md hover:bg-accent",
                active_class: "bg-accent",
                to: Route::AreaChartPage {},
                "Charts"
            }
            a {
                class: "inline-flex items-center py-1.5 px-2.5 text-sm rounded-md hover:bg-accent",
                href: "/create",
                "Create"
            }
        }
    }
}
