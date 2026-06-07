use dioxus::prelude::*;
use registry::ui::separator::{Separator, SeparatorOrientation};
use registry::ui::theme_toggle::ThemeToggle;

use crate::components::command_bar::CommandBarTrigger;
use crate::components::github_stars::GithubStars;
use crate::components::navigation::nav_desktop::NavDesktop;
use crate::components::navigation::nav_mobile::NavMobile;

#[component]
pub fn HeaderDocs() -> Element {
    rsx! {
        header { class: "sticky top-0 z-50 w-full border-b border-border/40 backdrop-blur supports-[backdrop-filter]:bg-background/60",
            div { class: "container flex justify-between items-center px-6 h-14",
                nav { class: "flex flex-1 justify-between items-center",
                    div {
                        NavMobile {}
                        NavDesktop {}
                    }
                    div { class: "flex gap-2 items-center min-w-0",
                        CommandBarTrigger {}
                        Separator { orientation: SeparatorOrientation::Vertical, class: "hidden ml-2 h-4 lg:block" }
                        GithubStars {}
                        Separator { orientation: SeparatorOrientation::Vertical, class: "hidden h-4 lg:block" }
                        ThemeToggle {}
                    }
                }
            }
        }
    }
}
