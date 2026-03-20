use dioxus::prelude::*;

use crate::Route;

const COMPONENTS: &[(&str, &str)] = &[
    ("button", "Button"),
    ("card", "Card"),
    ("input", "Input"),
    ("badge", "Badge"),
    ("separator", "Separator"),
    ("skeleton", "Skeleton"),
    ("spinner", "Spinner"),
];

#[component]
pub fn Sidenav() -> Element {
    rsx! {
        nav { class: "w-56 shrink-0 min-h-screen border-r bg-background flex flex-col p-4 gap-1",
            p { class: "text-xs font-medium text-muted-foreground uppercase tracking-wider px-2 mb-1",
                "Components"
            }
            for (slug, label) in COMPONENTS {
                Link {
                    class: "flex items-center rounded-md px-2 py-1.5 text-sm text-muted-foreground hover:bg-accent hover:text-accent-foreground transition-colors",
                    active_class: "bg-accent text-accent-foreground font-medium",
                    to: Route::ComponentPage { name: slug.to_string() },
                    "{label}"
                }
            }
        }
    }
}
