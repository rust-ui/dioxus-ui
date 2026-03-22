use dioxus::prelude::*;
use tw_merge::tw_merge;

#[component]
pub fn Switch(
    #[props(default = false)] checked: bool,
    #[props(default)] on_change: Option<EventHandler<bool>>,
    #[props(into, default)] class: Option<String>,
) -> Element {
    let mut state = use_signal(|| checked);
    let class = tw_merge!(
        "peer inline-flex h-6 w-11 shrink-0 cursor-pointer items-center rounded-full border-2 border-transparent transition-colors focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring focus-visible:ring-offset-2 focus-visible:ring-offset-background disabled:cursor-not-allowed disabled:opacity-50",
        class.as_deref().unwrap_or("")
    );
    let bg = if state() { "bg-primary" } else { "bg-input" };
    let data_state = if state() { "checked" } else { "unchecked" };
    let thumb_translate = if state() { "translate-x-5" } else { "translate-x-0" };
    rsx! {
        button {
            role: "switch",
            aria_checked: "{state()}",
            "data-state": data_state,
            class: "{class} {bg}",
            onclick: move |_| {
                let new_val = !state();
                state.set(new_val);
                if let Some(handler) = &on_change {
                    handler.call(new_val);
                }
            },
            span {
                class: "pointer-events-none block h-5 w-5 rounded-full bg-background shadow-lg ring-0 transition-transform {thumb_translate}",
                "data-state": data_state,
            }
        }
    }
}
