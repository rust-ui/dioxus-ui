use dioxus::prelude::*;
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
                    svg {
                        xmlns: "http://www.w3.org/2000/svg",
                        width: "12",
                        height: "12",
                        view_box: "0 0 24 24",
                        fill: "none",
                        stroke: "currentColor",
                        stroke_width: "3",
                        stroke_linecap: "round",
                        stroke_linejoin: "round",
                        polyline { points: "20 6 9 17 4 12" }
                    }
                }
            }
        }
    }
}
