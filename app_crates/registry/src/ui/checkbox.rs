use dioxus::prelude::*;
use icons::Check;
use tw_merge::tw_merge;

#[component]
pub fn Checkbox(
    #[props(default = false)] checked: bool,
    #[props(default = false)] disabled: bool,
    #[props(default)] on_change: Option<EventHandler<bool>>,
    #[props(into, default)] class: Option<String>,
) -> Element {
    let mut state = use_signal(|| checked);
    let class = tw_merge!(
        "peer size-4 shrink-0 rounded-sm border border-primary ring-offset-background focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring focus-visible:ring-offset-2 disabled:cursor-not-allowed disabled:opacity-50",
        class.as_deref().unwrap_or("")
    );
    let bg = if state() { "bg-primary text-primary-foreground" } else { "bg-background" };
    let data_state = if state() { "checked" } else { "unchecked" };
    rsx! {
        button {
            role: "checkbox",
            aria_checked: "{state()}",
            "data-state": data_state,
            disabled,
            class: "{class} {bg}",
            onclick: move |_| {
                let new_val = !state();
                state.set(new_val);
                if let Some(handler) = &on_change {
                    handler.call(new_val);
                }
            },
            if state() {
                span { class: "flex items-center justify-center text-current",
                    Check { class: "size-3.5" }
                }
            }
        }
    }
}
