use dioxus::prelude::*;

#[component]
pub fn Workflow01() -> Element {
    rsx! {
        div { class: "flex justify-center items-center w-full h-full text-muted-foreground text-sm",
            "Coming soon"
        }
    }
}
