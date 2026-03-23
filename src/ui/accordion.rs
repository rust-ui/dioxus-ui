use dioxus::prelude::*;
use dioxus::prelude::Signal;
use tw_merge::tw_merge;

#[component]
pub fn Accordion(
    #[props(into, optional)] class: Option<String>,
    children: Element,
) -> Element {
    let merged = tw_merge!("divide-y divide-border w-full", class.as_deref().unwrap_or(""));
    rsx! { div { class: "{merged}", {children} } }
}

#[component]
pub fn AccordionItem(
    #[props(into, optional)] class: Option<String>,
    children: Element,
) -> Element {
    let merged = tw_merge!("w-full", class.as_deref().unwrap_or(""));
    rsx! { div { class: "{merged}", {children} } }
}

#[component]
pub fn AccordionTrigger(
    #[props(into, optional)] class: Option<String>,
    open: Signal<bool>,
    onclick: EventHandler<MouseEvent>,
    children: Element,
) -> Element {
    let merged = tw_merge!(
        "flex w-full justify-between items-center p-3 text-sm font-medium cursor-pointer hover:bg-muted/50 transition-colors",
        class.as_deref().unwrap_or("")
    );
    let rotate = if open() { "rotate-180" } else { "" };
    rsx! {
        button {
            class: "{merged}",
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
pub fn AccordionContent(
    #[props(into, optional)] class: Option<String>,
    open: Signal<bool>,
    children: Element,
) -> Element {
    let merged = tw_merge!("px-3 pb-3 text-sm text-muted-foreground", class.as_deref().unwrap_or(""));
    if open() {
        rsx! { div { class: "{merged}", {children} } }
    } else {
        rsx! {}
    }
}
