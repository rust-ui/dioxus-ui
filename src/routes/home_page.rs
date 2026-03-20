use dioxus::prelude::*;

use crate::ui::button::Button;
use crate::Route;

#[component]
pub fn Home() -> Element {
    rsx! {
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
