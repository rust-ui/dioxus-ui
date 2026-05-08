use dioxus::prelude::*;
use tw_merge::tw_merge;

#[component]
pub fn Faq(#[props(into, optional)] class: Option<String>, children: Element) -> Element {
    let c = tw_merge!("flex flex-col gap-3 w-full max-w-screen-md", class.as_deref().unwrap_or(""));
    rsx! { div { "data-name": "Faq", class: "{c}", {children} } }
}

#[component]
pub fn FaqTitle(#[props(into, optional)] class: Option<String>, children: Element) -> Element {
    let c = tw_merge!("text-lg text-primary", class.as_deref().unwrap_or(""));
    rsx! { span { "data-name": "FaqTitle", class: "{c}", {children} } }
}

#[component]
pub fn FaqDescription(
    #[props(into, optional)] class: Option<String>,
    children: Element,
) -> Element {
    let c = tw_merge!("pr-6 mb-2 text-muted-foreground", class.as_deref().unwrap_or(""));
    rsx! { p { "data-name": "FaqDescription", class: "{c}", {children} } }
}

#[component]
pub fn FaqSection(#[props(into, optional)] class: Option<String>, children: Element) -> Element {
    let c = tw_merge!(
        "w-full rounded bg-accent/30 hover:bg-accent flex flex-col",
        class.as_deref().unwrap_or("")
    );
    rsx! { div { "data-name": "FaqSection", class: "{c}", {children} } }
}

#[component]
pub fn FaqContent(children: Element) -> Element {
    rsx! {
        div { class: "grid overflow-hidden mt-2 transition-all duration-500 grid-rows-[0fr] peer-checked:grid-rows-[1fr]",
            div { class: "px-4 min-h-[0]", {children} }
        }
    }
}

#[component]
pub fn FaqLabel(
    #[props(into, optional)] class: Option<String>,
    #[props(into)] html_for: String,
    children: Element,
) -> Element {
    let c = tw_merge!(
        "flex justify-between items-center py-2 px-4 mt-2 w-full cursor-pointer",
        class.as_deref().unwrap_or("")
    );
    rsx! {
        label { class: "{c}", r#for: "{html_for}", {children} }
    }
}

#[component]
pub fn FaqInput(#[props(into)] id: String) -> Element {
    rsx! {
        input { id: "{id}", r#type: "checkbox", class: "ml-auto peer" }
    }
}
