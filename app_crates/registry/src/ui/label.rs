use dioxus::prelude::*;
use tw_merge::tw_merge;

#[component]
pub fn Label(
    #[props(into, default)] html_for: Option<String>,
    #[props(into, default)] class: Option<String>,
    children: Element,
) -> Element {
    let class = tw_merge!(
        "text-sm font-medium leading-none select-none peer-disabled:cursor-not-allowed peer-disabled:opacity-70",
        class.as_deref().unwrap_or("")
    );
    rsx! {
        label {
            r#for: html_for.as_deref().unwrap_or(""),
            class: "{class}",
            {children}
        }
    }
}
