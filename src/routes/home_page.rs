use dioxus::prelude::*;

use crate::ui::button::Button;
use crate::ui::seo_meta::SeoMeta;
use crate::Route;

#[component]
pub fn Home() -> Element {
    rsx! {
        SeoMeta {
            title: "Dioxus UI · Component Library for Dioxus".to_string(),
            description: "Beautiful UI components built with Dioxus and Tailwind CSS. A cross-platform component library for modern Rust web apps.".to_string(),
        }
        div { class: "flex flex-col items-center justify-center min-h-[calc(100vh-3.5rem)] gap-6 text-center px-4",
            h1 { class: "text-4xl font-bold tracking-tight", "Dioxus UI" }
            p { class: "text-muted-foreground text-lg max-w-md",
                "A component library built with Dioxus and Tailwind CSS."
            }
            Link { to: Route::ComponentPage { name: "button".to_string() },
                Button { "Browse Components" }
            }
        }
    }
}
