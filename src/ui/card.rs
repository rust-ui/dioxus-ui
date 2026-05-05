use dioxus::prelude::*;
use tw_merge::tw_merge;

#[component]
pub fn Card(#[props(into, optional)] class: Option<String>, children: Element) -> Element {
    let merged_class = tw_merge!(
        "bg-card text-card-foreground flex flex-col gap-4 rounded-xl border py-6 shadow-sm",
        class.as_deref().unwrap_or("")
    );

    rsx! { div { "data-name": "Card", class: "{merged_class}", {children} } }
}

#[component]
pub fn CardHeader(#[props(into, optional)] class: Option<String>, children: Element) -> Element {
    let merged_class = tw_merge!(
        "@container/card-header grid auto-rows-min grid-rows-[auto_auto] items-start gap-1.5 px-6 has-data-[slot=card-action]:grid-cols-[1fr_auto] [.border-b]:pb-6",
        class.as_deref().unwrap_or("")
    );

    rsx! { div { "data-name": "CardHeader", class: "{merged_class}", {children} } }
}

#[component]
pub fn CardTitle(#[props(into, optional)] class: Option<String>, children: Element) -> Element {
    let merged_class = tw_merge!("leading-none font-semibold", class.as_deref().unwrap_or(""));

    rsx! { h2 { "data-name": "CardTitle", class: "{merged_class}", {children} } }
}

#[component]
pub fn CardContent(#[props(into, optional)] class: Option<String>, children: Element) -> Element {
    let merged_class = tw_merge!("px-6", class.as_deref().unwrap_or(""));

    rsx! { div { "data-name": "CardContent", class: "{merged_class}", {children} } }
}

#[component]
pub fn CardDescription(
    #[props(into, optional)] class: Option<String>,
    children: Element,
) -> Element {
    let merged_class = tw_merge!(
        "text-muted-foreground text-sm",
        class.as_deref().unwrap_or("")
    );

    rsx! { p { "data-name": "CardDescription", class: "{merged_class}", {children} } }
}

#[component]
pub fn CardFooter(#[props(into, optional)] class: Option<String>, children: Element) -> Element {
    let merged_class = tw_merge!(
        "flex items-center gap-2 px-6 [.border-t]:pt-6",
        class.as_deref().unwrap_or("")
    );

    rsx! { footer { "data-name": "CardFooter", class: "{merged_class}", {children} } }
}

#[component]
pub fn CardAction(#[props(into, optional)] class: Option<String>, children: Element) -> Element {
    let merged_class = tw_merge!(
        "col-start-2 row-span-2 row-start-1 self-start justify-self-end",
        class.as_deref().unwrap_or("")
    );

    rsx! { div { "data-name": "CardAction", class: "{merged_class}", {children} } }
}
