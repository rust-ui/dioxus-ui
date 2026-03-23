use dioxus::prelude::*;
use tw_merge::tw_merge;

#[component]
pub fn Collapsible(
    #[props(into, optional)] class: Option<String>,
    children: Element,
) -> Element {
    let merged = tw_merge!("w-full", class.as_deref().unwrap_or(""));
    rsx! { div { class: "{merged}", {children} } }
}

#[component]
pub fn CollapsibleTrigger(
    #[props(into, optional)] class: Option<String>,
    open: Signal<bool>,
    onclick: EventHandler<MouseEvent>,
    children: Element,
) -> Element {
    let merged = tw_merge!(
        "flex w-full items-center justify-between rounded-md px-3 py-2 text-sm font-medium hover:bg-muted/50 transition-colors cursor-pointer",
        class.as_deref().unwrap_or("")
    );
    let rotate = if open() { "rotate-180" } else { "" };
    rsx! {
        button {
            class: "{merged}",
            "aria-expanded": "{open()}",
            onclick: move |e| onclick.call(e),
            {children}
            svg {
                class: "size-4 shrink-0 transition-transform duration-200 {rotate}",
                xmlns: "http://www.w3.org/2000/svg",
                view_box: "0 0 24 24",
                fill: "none",
                stroke: "currentColor",
                stroke_width: "2",
                stroke_linecap: "round",
                stroke_linejoin: "round",
                path { d: "m6 9 6 6 6-6" }
            }
        }
    }
}

#[component]
pub fn CollapsibleContent(
    #[props(into, optional)] class: Option<String>,
    open: Signal<bool>,
    children: Element,
) -> Element {
    let merged = tw_merge!("mt-1", class.as_deref().unwrap_or(""));
    if open() {
        rsx! { div { class: "{merged}", {children} } }
    } else {
        rsx! {}
    }
}
