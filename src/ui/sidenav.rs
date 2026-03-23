use dioxus::prelude::*;

use crate::registry::REGISTRY;
use crate::Route;

#[component]
pub fn Sidenav() -> Element {
    rsx! {
        div { class: "hidden fixed top-14 z-30 md:flex md:sticky md:top-14 md:ml-2 w-[205px] h-[calc(100vh-3.5rem)] shrink-0",
            aside { class: "flex overflow-hidden flex-col flex-1 group/scrollbar-on-hover",
                div { class: "flex overflow-hidden overflow-y-auto overscroll-y-contain flex-col gap-4 pb-4 w-full h-full scrollbar__on_hover",
                    h4 { class: "my-1 text-sm font-semibold", "Components" }
                    ul { class: "ml-1 list-none",
                        for entry in REGISTRY {
                            li {
                                Link {
                                    class: "text-sm text-muted-foreground hover:underline",
                                    active_class: "font-bold",
                                    to: Route::ComponentPage { name: entry.slug.to_string() },
                                    {entry.title()}
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}
