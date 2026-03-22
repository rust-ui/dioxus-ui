use dioxus::prelude::*;

use crate::Route;
use crate::ui::theme_toggle::ThemeToggle;

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
                    Link {
                        class: "text-sm text-muted-foreground hover:text-foreground transition-colors",
                        active_class: "text-foreground font-medium",
                        to: Route::MarkdownPage { slug: "button".to_string() },
                        "Markdown"
                    }
                }
                ThemeToggle {}
            }
        }
    }
}
