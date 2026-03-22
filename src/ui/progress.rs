use dioxus::prelude::*;
use tw_merge::tw_merge;

#[component]
pub fn Progress(
    /// Value between 0.0 and 100.0
    #[props(default = 0.0)]
    value: f64,
    #[props(into, default)] class: Option<String>,
) -> Element {
    let class = tw_merge!(
        "relative h-2 w-full overflow-hidden rounded-full bg-secondary",
        class.as_deref().unwrap_or("")
    );
    let translate = format!("-{}%", 100.0 - value.clamp(0.0, 100.0));
    rsx! {
        div {
            role: "progressbar",
            aria_valuemin: "0",
            aria_valuemax: "100",
            aria_valuenow: "{value}",
            class: "{class}",
            div {
                class: "h-full w-full flex-1 bg-primary transition-all",
                style: "transform: translateX({translate})",
            }
        }
    }
}
