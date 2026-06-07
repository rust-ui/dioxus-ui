use dioxus::prelude::*;
use icons::{
    AlignHorizontalSpaceAround, BlocksAnimate, CalendarDaysAnimate, CompassAnimate, FrameAnimate, LogIn, Menu,
    PanelLeftOpenAnimate, SearchAnimate, WindAnimate, X,
};
use registry::ui::header::{
    Header, IconWrapper, InsetCard, NavMenu, NavMenuContent, NavMenuContentInset, NavMenuFixed, NavMenuHomeLink,
    NavMenuItem, NavMenuLink, NavMenuLinkDescription, NavMenuLinkGrid, NavMenuLinkTitle, NavMenuList, NavMenuMiddle,
    NavMenuTitle, NavMenuTrigger, NavMenuWrapper,
};
use registry::ui::theme_toggle::ThemeToggle;

use crate::components::command_bar::CommandBarTrigger;
use crate::components::github_stars::GithubStars;

#[component]
pub fn HeaderHome() -> Element {
    let is_mobile_menu_open = use_signal(|| false);
    let data_state = move || if is_mobile_menu_open() { "active" } else { "inactive" };

    rsx! {
        Header { data_state: data_state(),
            NavMenuFixed {
                NavMenuWrapper {
                    div { class: "flex relative flex-wrap justify-between items-center lg:py-3",
                        div { class: "flex gap-8 justify-between items-center max-md:in-data-[state=active]:border-b max-md:h-14 max-md:w-full",
                            NavMenuHomeLink { href: "/", aria_label: "home",
                                span { class: "text-lg font-semibold", "Dioxus/UI" }
                            }
                            MobileMenuTrigger { is_open: is_mobile_menu_open }
                        }

                        NavMenuMiddle {
                            NavMenu {
                                NavMenuList {
                                    FirstNavMenu {}
                                    SecondNavMenu {}

                                    NavMenuItem {
                                        NavMenuLink { href: "/icons", "Icons" }
                                    }
                                    NavMenuItem {
                                        NavMenuLink { href: "/create", "Create" }
                                    }
                                }
                            }
                        }

                        NavMenuRight {}
                    }

                    MobileMenu { is_open: is_mobile_menu_open }
                }
            }
        }
    }
}

/* ========================================================== */
/*                     ✨ FUNCTIONS ✨                        */
/* ========================================================== */

#[component]
fn FirstNavMenu() -> Element {
    rsx! {
        NavMenuItem {
            NavMenuTrigger { href: "/components/button", "Registry" }
            NavMenuContent { class: "min-w-lg",
                NavMenuContentInset { class: "grid gap-2 grid-cols-[auto_1fr]",
                    InsetCard {
                        NavMenuTitle { "Content" }
                        ul { class: "mt-1 space-y-2",
                            li {
                                NavMenuLinkGrid { href: "/components/button",
                                    IconWrapper { BlocksAnimate { class: "text-foreground" } }
                                    div { class: "space-y-0.5",
                                        NavMenuLinkTitle { "Components" }
                                        NavMenuLinkDescription { "Find any components" }
                                    }
                                }
                            }
                            li {
                                NavMenuLinkGrid { href: "/icons",
                                    IconWrapper { SearchAnimate { class: "text-foreground" } }
                                    div { class: "space-y-0.5",
                                        NavMenuLinkTitle { "Icons" }
                                        NavMenuLinkDescription { "More than 1600 icons" }
                                    }
                                }
                            }
                            li {
                                NavMenuLinkGrid { href: "/hooks/use-copy-clipboard",
                                    IconWrapper { CompassAnimate { class: "text-foreground" } }
                                    div { class: "space-y-0.5",
                                        NavMenuLinkTitle { "Hooks" }
                                        NavMenuLinkDescription { "Reusable logic functions" }
                                    }
                                }
                            }
                        }
                    }
                    div { class: "p-2",
                        NavMenuTitle { "Get Started" }
                        ul { class: "mt-1",
                            li {
                                NavMenuLinkGrid { class: "items-center", href: "/components/introduction",
                                    IconWrapper { FrameAnimate { class: "text-foreground" } }
                                    NavMenuLinkTitle { "Introduction" }
                                }
                            }
                            li {
                                NavMenuLinkGrid { class: "items-center", href: "/components/installation",
                                    IconWrapper { WindAnimate { class: "text-foreground" } }
                                    NavMenuLinkTitle { "Installation" }
                                }
                            }
                            li {
                                NavMenuLinkGrid { class: "items-center", href: "/components/changelog",
                                    IconWrapper { CalendarDaysAnimate { class: "text-foreground" } }
                                    NavMenuLinkTitle { "Changelog" }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

/* ========================================================== */
/*                     ✨ FUNCTIONS ✨                        */
/* ========================================================== */

#[component]
fn SecondNavMenu() -> Element {
    rsx! {
        NavMenuItem {
            NavMenuTrigger { href: "/blocks/login", "Blocks" }
            NavMenuContent { class: "w-[300px]",
                NavMenuContentInset {
                    InsetCard {
                        h3 { class: "px-2 mb-4 text-sm font-semibold text-foreground", "Demonstrations" }
                        ul { class: "space-y-2",
                            li {
                                NavMenuLinkGrid { href: "/blocks/login",
                                    IconWrapper { LogIn { class: "text-foreground" } }
                                    div { class: "space-y-0.5",
                                        NavMenuLinkTitle { "Login" }
                                        NavMenuLinkDescription { "Login Blocks" }
                                    }
                                }
                            }
                            li {
                                NavMenuLinkGrid { href: "/blocks/sidenav",
                                    IconWrapper { PanelLeftOpenAnimate { class: "text-foreground" } }
                                    div { class: "space-y-0.5",
                                        NavMenuLinkTitle { "Sidenav" }
                                        NavMenuLinkDescription { "Sidenav Blocks" }
                                    }
                                }
                            }
                            li {
                                NavMenuLinkGrid { href: "/blocks/footers",
                                    IconWrapper { AlignHorizontalSpaceAround { class: "text-foreground" } }
                                    div { class: "space-y-0.5",
                                        NavMenuLinkTitle { "Footers" }
                                        NavMenuLinkDescription { "Footers Blocks" }
                                    }
                                }
                            }
                        }
                    }
                    div { class: "p-2 w-full",
                        NavMenuLink {
                            class: "items-start font-normal underline text-primary",
                            href: "/blocks/login",
                            "Browse all Blocks"
                        }
                    }
                }
            }
        }
    }
}

/* ========================================================== */
/*                     ✨ FUNCTIONS ✨                        */
/* ========================================================== */

#[component]
fn NavMenuRight() -> Element {
    rsx! {
        div { class: "hidden md:flex md:gap-6 md:p-0 md:m-0 md:bg-transparent md:border-transparent md:shadow-none dark:shadow-none md:w-fit dark:md:bg-transparent",
            div { class: "flex flex-row gap-3 items-center",
                CommandBarTrigger {}
                GithubStars {}
                div { class: "flex justify-center items-center mr-1",
                    ThemeToggle {}
                }
            }
        }
    }
}

/* ========================================================== */
/*                     ✨ FUNCTIONS ✨                        */
/* ========================================================== */

#[component]
fn MobileMenuTrigger(is_open: Signal<bool>) -> Element {
    rsx! {
        button {
            "aria-label": "Open Menu",
            class: "block relative z-20 p-2.5 -m-2.5 -mr-3 cursor-pointer md:hidden",
            onclick: move |_| is_open.set(!is_open()),
            Menu { class: "m-auto duration-200 lucide lucide-menu in-data-[state=active]:rotate-180 in-data-[state=active]:scale-0 in-data-[state=active]:opacity-0 size-5" }
            X { class: "absolute inset-0 m-auto opacity-0 duration-200 scale-0 -rotate-180 lucide lucide-x in-data-[state=active]:rotate-0 in-data-[state=active]:scale-100 in-data-[state=active]:opacity-100 size-5" }
        }
    }
}

/* ========================================================== */
/*                     ✨ FUNCTIONS ✨                        */
/* ========================================================== */

#[component]
fn MobileMenu(is_open: Signal<bool>) -> Element {
    let display_class = move || if is_open() { "flex" } else { "hidden" };

    rsx! {
        div { class: "flex-col gap-4 mt-6 w-full md:hidden mb-4 {display_class()}",
            div { class: "flex flex-row gap-3 items-center w-full",
                CommandBarTrigger {}
                GithubStars {}
                div { class: "flex justify-center items-center mr-1",
                    ThemeToggle {}
                }
            }
        }
    }
}
