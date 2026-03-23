use dioxus::prelude::*;
use tw_merge::tw_merge;

#[component]
pub fn ToggleGroup(
    #[props(into, optional)] class: Option<String>,
    children: Element,
) -> Element {
    let merged = tw_merge!(
        "inline-flex items-center rounded-md border bg-muted p-1 text-muted-foreground gap-0",
        class.as_deref().unwrap_or("")
    );
    rsx! { div { class: "{merged}", role: "group", {children} } }
}

#[component]
pub fn ToggleGroupItem(
    #[props(into, optional)] class: Option<String>,
    #[props(into)] value: String,
    #[props(default = false)] pressed: bool,
    #[props(optional)] onclick: Option<EventHandler<MouseEvent>>,
    children: Element,
) -> Element {
    let state = if pressed { "on" } else { "off" };
    let merged = tw_merge!(
        "inline-flex flex-1 items-center justify-center gap-2 px-3 py-1.5 text-sm font-medium rounded-sm transition-colors whitespace-nowrap cursor-pointer bg-transparent hover:bg-background/50 data-[state=on]:bg-background data-[state=on]:text-foreground data-[state=on]:shadow-sm",
        class.as_deref().unwrap_or("")
    );
    rsx! {
        button {
            class: "{merged}",
            "data-state": "{state}",
            "aria-pressed": "{pressed}",
            role: "radio",
            onclick: move |e| {
                if let Some(handler) = &onclick {
                    handler.call(e);
                }
            },
            {children}
        }
    }
}
