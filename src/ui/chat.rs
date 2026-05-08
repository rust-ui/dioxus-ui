use dioxus::prelude::*;
use tw_merge::tw_merge;

#[component]
pub fn ChatCard(#[props(into, optional)] class: Option<String>, children: Element) -> Element {
    let c = tw_merge!("flex flex-col w-full rounded-lg border", class.as_deref().unwrap_or(""));
    rsx! { div { "data-name": "ChatCard", class: "{c}", {children} } }
}

#[component]
pub fn ChatHeader(#[props(into, optional)] class: Option<String>, children: Element) -> Element {
    let c = tw_merge!("flex items-center border-b", class.as_deref().unwrap_or(""));
    rsx! { header { "data-name": "ChatHeader", class: "{c}", {children} } }
}

#[component]
pub fn ChatBody(#[props(into, optional)] class: Option<String>, children: Element) -> Element {
    let c = tw_merge!("overflow-hidden flex-1", class.as_deref().unwrap_or(""));
    let mounted = use_hook(|| format!("document.querySelector('[data-name=\"ChatBody\"]')?.scrollTo({{top: 9999}})"));
    rsx! {
        div { "data-name": "ChatBody", class: "{c}", {children} }
        script { dangerous_inner_html: "{mounted}" }
    }
}

#[component]
pub fn ChatMessageList(
    #[props(into, optional)] class: Option<String>,
    children: Element,
) -> Element {
    let c = tw_merge!("space-y-4", class.as_deref().unwrap_or(""));
    rsx! { div { "data-name": "ChatMessageList", class: "{c}", {children} } }
}

#[component]
pub fn ChatMessageReceived(
    #[props(into, optional)] class: Option<String>,
    children: Element,
) -> Element {
    let c = tw_merge!("flex max-w-[85%]", class.as_deref().unwrap_or(""));
    rsx! { div { "data-name": "ChatMessageReceived", class: "{c}", {children} } }
}

#[component]
pub fn ChatMessageSent(
    #[props(into, optional)] class: Option<String>,
    children: Element,
) -> Element {
    let c = tw_merge!("flex ml-auto max-w-[85%]", class.as_deref().unwrap_or(""));
    rsx! { div { "data-name": "ChatMessageSent", class: "{c}", {children} } }
}

#[component]
pub fn ChatMessageAvatar(
    #[props(into, optional)] class: Option<String>,
    children: Element,
) -> Element {
    let c = tw_merge!(
        "flex shrink-0 overflow-hidden rounded-full",
        class.as_deref().unwrap_or("")
    );
    rsx! { span { "data-name": "ChatMessageAvatar", class: "{c}", {children} } }
}

#[component]
pub fn ChatMessageBubble(
    #[props(into, optional)] class: Option<String>,
    children: Element,
) -> Element {
    let c = tw_merge!("py-2 px-3 text-sm rounded-lg", class.as_deref().unwrap_or(""));
    rsx! { div { "data-name": "ChatMessageBubble", class: "{c}", {children} } }
}

#[component]
pub fn ChatMessageContent(
    #[props(into, optional)] class: Option<String>,
    children: Element,
) -> Element {
    let c = tw_merge!("leading-normal break-words", class.as_deref().unwrap_or(""));
    rsx! { p { "data-name": "ChatMessageContent", class: "{c}", {children} } }
}

#[component]
pub fn ChatMessageTime(
    #[props(into, optional)] class: Option<String>,
    children: Element,
) -> Element {
    let c = tw_merge!("mt-1 text-xs text-right", class.as_deref().unwrap_or(""));
    rsx! { p { "data-name": "ChatMessageTime", class: "{c}", {children} } }
}

#[component]
pub fn ChatFooter(#[props(into, optional)] class: Option<String>, children: Element) -> Element {
    let c = tw_merge!("flex items-center border-t", class.as_deref().unwrap_or(""));
    rsx! { footer { "data-name": "ChatFooter", class: "{c}", {children} } }
}
