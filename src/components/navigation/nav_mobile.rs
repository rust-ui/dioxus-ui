use dioxus::prelude::*;

use crate::Route;

#[component]
pub fn NavMobile() -> Element {
    let mut open = use_signal(|| false);

    rsx! {
        div { class: "md:hidden",
            button {
                class: "inline-flex items-center justify-center p-2 rounded-md text-muted-foreground hover:bg-accent",
                "aria-label": "Open Menu",
                onclick: move |_| open.set(!open()),
                svg {
                    stroke_width: "1.5",
                    view_box: "0 0 24 24",
                    fill: "none",
                    xmlns: "http://www.w3.org/2000/svg",
                    class: "size-5",
                    path { d: "M3 5H11", stroke: "currentColor", stroke_width: "1.5", stroke_linecap: "round", stroke_linejoin: "round" }
                    path { d: "M3 12H16", stroke: "currentColor", stroke_width: "1.5", stroke_linecap: "round", stroke_linejoin: "round" }
                    path { d: "M3 19H21", stroke: "currentColor", stroke_width: "1.5", stroke_linecap: "round", stroke_linejoin: "round" }
                }
            }

            if open() {
                div { class: "absolute top-14 left-0 right-0 z-50 bg-background border-b border-border p-4 flex flex-col gap-1",
                    Link {
                        class: "py-2 px-3 text-sm rounded-md hover:bg-accent",
                        to: Route::ComponentPage { name: "button".to_string() },
                        onclick: move |_| open.set(false),
                        "Components"
                    }
                    Link {
                        class: "py-2 px-3 text-sm rounded-md hover:bg-accent",
                        to: Route::HookPage { name: "use-copy-clipboard".to_string() },
                        onclick: move |_| open.set(false),
                        "Hooks"
                    }
                    Link {
                        class: "py-2 px-3 text-sm rounded-md hover:bg-accent",
                        to: Route::PageIcons {},
                        onclick: move |_| open.set(false),
                        "Icons"
                    }
                    a {
                        class: "py-2 px-3 text-sm rounded-md hover:bg-accent",
                        href: "/blocks",
                        onclick: move |_| open.set(false),
                        "Blocks"
                    }
                    Link {
                        class: "py-2 px-3 text-sm rounded-md hover:bg-accent",
                        to: Route::AreaChartPage {},
                        onclick: move |_| open.set(false),
                        "Charts"
                    }
                    Link {
                        class: "py-2 px-3 text-sm rounded-md hover:bg-accent",
                        to: Route::WorkflowsPage {},
                        onclick: move |_| open.set(false),
                        "Workflows"
                    }
                }
            }
        }
    }
}
