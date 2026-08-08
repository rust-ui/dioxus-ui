use dioxus::prelude::*;

use crate::__registry__::sidenav::SIDENAV_ITEMS;
use crate::__registry__::sidenav_get_started::SIDENAV_GET_STARTED_ITEMS;
use crate::__registry__::sidenav_hooks::SIDENAV_HOOKS_ITEMS;
use crate::Route;

#[component]
pub fn Sidenav() -> Element {
    let route = use_route::<Route>();
    let is_hooks = matches!(route, Route::HookPage { .. });
    let is_docs = matches!(route, Route::DocsPage { .. });

    rsx! {
        div { class: "hidden fixed top-14 z-30 md:flex md:sticky md:top-14 md:ml-2 w-[205px] h-[calc(100vh-3.5rem)] shrink-0",
            aside { class: "flex overflow-hidden flex-col flex-1 group/scrollbar-on-hover",
                div { class: "flex overflow-hidden overflow-y-auto overscroll-y-contain flex-col gap-4 pb-4 w-full h-full scrollbar__on_hover",
                    if is_docs {
                        h4 { class: "my-1 text-sm font-semibold", "Get Started" }
                        ul { class: "ml-1 list-none",
                            for item in SIDENAV_GET_STARTED_ITEMS {
                                li {
                                    Link {
                                        class: "text-sm text-muted-foreground hover:underline",
                                        active_class: "font-bold",
                                        to: Route::DocsPage { name: item.slug.to_string() },
                                        "{item.label}"
                                    }
                                }
                            }
                        }
                    } else if is_hooks {
                        h4 { class: "my-1 text-sm font-semibold", "Hooks" }
                        ul { class: "ml-1 list-none",
                            for item in SIDENAV_HOOKS_ITEMS {
                                li {
                                    Link {
                                        class: "text-sm text-muted-foreground hover:underline",
                                        active_class: "font-bold",
                                        to: Route::HookPage { name: item.slug.to_string() },
                                        "{item.label}"
                                    }
                                }
                            }
                        }
                    } else {
                        h4 { class: "my-1 text-sm font-semibold", "Components" }
                        ul { class: "ml-1 list-none",
                            for item in SIDENAV_ITEMS {
                                li {
                                    Link {
                                        class: "text-sm text-muted-foreground hover:underline",
                                        active_class: "font-bold",
                                        to: Route::ComponentPage { name: item.slug.to_string() },
                                        "{item.label}"
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}
