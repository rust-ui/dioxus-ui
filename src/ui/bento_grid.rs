use dioxus::prelude::*;
use tw_merge::tw_merge;

#[component]
pub fn BentoGrid(#[props(into, optional)] class: Option<String>, children: Element) -> Element {
    let c = tw_merge!("grid gap-2 md:grid-cols-4", class.as_deref().unwrap_or(""));
    rsx! { div { "data-name": "BentoGrid", class: "{c}", {children} } }
}

#[component]
pub fn BentoGrid6(#[props(into, optional)] class: Option<String>, children: Element) -> Element {
    let c = tw_merge!("grid gap-2 sm:grid-cols-2 md:grid-cols-4", class.as_deref().unwrap_or(""));
    rsx! { div { "data-name": "BentoGrid6", class: "{c}", {children} } }
}

#[component]
pub fn BentoRow(#[props(into, optional)] class: Option<String>, children: Element) -> Element {
    let c = tw_merge!("p-1 min-h-32 rounded-lg", class.as_deref().unwrap_or(""));
    rsx! { div { "data-name": "BentoRow", class: "{c}", {children} } }
}

#[component]
pub fn BentoCell(#[props(into, optional)] class: Option<String>, children: Element) -> Element {
    let c = tw_merge!(
        "text-xl rounded-lg size-full center bg-zinc-200 dark:bg-zinc-700",
        class.as_deref().unwrap_or("")
    );
    rsx! { div { "data-name": "BentoCell", class: "{c}", {children} } }
}
