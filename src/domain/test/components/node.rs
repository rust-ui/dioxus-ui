use dioxus::prelude::*;
use tw_merge::tw_merge;

#[component]
pub fn Node(
    #[props(optional)] target: bool,
    #[props(optional)] source: bool,
    #[props(into, optional)] class: Option<String>,
    children: Element,
) -> Element {
    let merged = tw_merge!(
        "relative w-full rounded-md border bg-card shadow-sm text-card-foreground",
        class.as_deref().unwrap_or("")
    );
    rsx! {
        div { class: "{merged}",
            if target {
                div {
                    class: "absolute left-0 top-1/2 size-3 rounded-full border-2 border-primary bg-background pointer-events-none",
                    style: "transform: translate(-50%, -50%);",
                }
            }
            if source {
                div {
                    class: "absolute right-0 top-1/2 size-3 rounded-full border-2 border-primary bg-background pointer-events-none",
                    style: "transform: translate(50%, -50%);",
                }
            }
            {children}
        }
    }
}

#[component]
pub fn NodeHeader(
    #[props(into, optional)] class: Option<String>,
    children: Element,
) -> Element {
    let merged = tw_merge!(
        "flex items-center gap-2 px-3 py-2.5 border-b bg-secondary/50 rounded-t-md",
        class.as_deref().unwrap_or("")
    );
    rsx! { div { class: "{merged}", {children} } }
}

#[component]
pub fn NodeTitle(
    #[props(into, optional)] class: Option<String>,
    children: Element,
) -> Element {
    let merged = tw_merge!("text-xs font-medium", class.as_deref().unwrap_or(""));
    rsx! { span { class: "{merged}", {children} } }
}

#[component]
pub fn NodeDescription(
    #[props(into, optional)] class: Option<String>,
    children: Element,
) -> Element {
    let merged = tw_merge!(
        "text-[10px] text-muted-foreground",
        class.as_deref().unwrap_or("")
    );
    rsx! { p { class: "{merged}", {children} } }
}

#[component]
pub fn NodeContent(
    #[props(into, optional)] class: Option<String>,
    children: Element,
) -> Element {
    let merged = tw_merge!("px-3 py-2", class.as_deref().unwrap_or(""));
    rsx! { div { class: "{merged}", {children} } }
}

#[component]
pub fn NodeFooter(
    #[props(into, optional)] class: Option<String>,
    children: Element,
) -> Element {
    let merged = tw_merge!(
        "flex items-center gap-2 px-3 py-2.5 border-t bg-secondary/50 rounded-b-md",
        class.as_deref().unwrap_or("")
    );
    rsx! { div { class: "{merged}", {children} } }
}
