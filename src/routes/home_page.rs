use app_config::SeoMeta;
use app_domain::themes::components::theme_selector::ThemeSelector;
use dioxus::prelude::*;
use registry::ui::button::Button;

use crate::Route;

#[component]
pub fn Home() -> Element {
    rsx! {
        SeoMeta {
            title: "Rust UI · Component Library for Dioxus".to_string(),
            description: "Beautiful UI components built with Dioxus and Tailwind CSS. A cross-platform component library for modern Rust web apps.".to_string(),
        }
        div { class: "flex flex-col items-center gap-12 px-4 py-16 mx-auto w-full max-w-[1200px]",
            div { class: "flex flex-col items-center gap-6 text-center",
                h1 { class: "text-4xl font-bold tracking-tight", "Dioxus UI" }
                p { class: "text-muted-foreground text-lg max-w-md",
                    "A component library built with Dioxus and Tailwind CSS."
                }
                Link { to: Route::ComponentPage { name: "button".to_string() },
                    Button { "Browse Components" }
                }
            }
            ThemeSelector {}
        }
    }
}
