use dioxus::prelude::*;
use registry::ui::separator::{Separator, SeparatorOrientation};
use registry::ui::theme_toggle::ThemeToggle;

use crate::Route;
use crate::components::command_bar::CommandBarTrigger;
use crate::components::github_stars::GithubStars;

#[component]
pub fn Navbar() -> Element {
    rsx! {
        header { class: "sticky top-0 z-50 w-full border-b border-border/40 backdrop-blur supports-[backdrop-filter]:bg-background/60",
            div { class: "flex h-14 items-center justify-between px-6",
                div { class: "flex items-center gap-6",
                    Link {
                        class: "text-sm font-semibold hover:text-foreground/80 transition-colors",
                        to: Route::Home {},
                        "Dioxus UI"
                    }
                    Link {
                        class: "text-sm text-muted-foreground hover:text-foreground transition-colors",
                        active_class: "text-foreground font-medium",
                        to: Route::ComponentPage { name: "button".to_string() },
                        "Components"
                    }
                }
                div { class: "flex items-center gap-2",
                    CommandBarTrigger {}
                    Separator {
                        orientation: SeparatorOrientation::Vertical,
                        class: "hidden h-4 lg:block ml-2"
                    }
                    GithubStars {}
                    Separator {
                        orientation: SeparatorOrientation::Vertical,
                        class: "hidden h-4 lg:block"
                    }
                    ThemeToggle {}
                }
            }
        }
    }
}
