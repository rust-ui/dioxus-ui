use dioxus::prelude::*;
use tw_merge::tw_merge;

#[component]
pub fn Kbd(#[props(into, default)] class: Option<String>, children: Element) -> Element {
    let class = tw_merge!(
        "inline-flex items-center gap-1 rounded border border-border bg-muted px-1.5 py-0.5 font-mono text-xs text-muted-foreground",
        class.as_deref().unwrap_or("")
    );
    rsx! {
        kbd { class: "{class}", {children} }
    }
}

#[component]
pub fn KbdGroup(#[props(into, default)] class: Option<String>, children: Element) -> Element {
    let class = tw_merge!(
        "inline-flex items-center gap-1",
        class.as_deref().unwrap_or("")
    );
    rsx! {
        kbd { class: "{class}", {children} }
    }
}
