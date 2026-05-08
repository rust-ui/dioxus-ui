use dioxus::prelude::*;

const LOGO_SRC: &str = "/icons/logo.png";

#[component]
pub fn HomePage() -> Element {
    rsx! {
        div { class: "flex flex-col justify-center items-center px-6 h-full text-center",
            img {
                src: LOGO_SRC,
                alt: "Logo",
                class: "mb-6 rounded-2xl size-20",
            }
            h1 { class: "text-2xl font-bold tracking-tight mb-3", "Dioxus Fullstack" }
            p { class: "text-muted-foreground text-sm max-w-xs",
                "Cross-platform starter — web, mobile, desktop from one codebase. Built with "
                a {
                    href: "https://rust-ui.com",
                    target: "_blank",
                    class: "underline underline-offset-2 hover:text-foreground transition-colors",
                    "rust-ui.com"
                }
                "."
            }
        }
    }
}
