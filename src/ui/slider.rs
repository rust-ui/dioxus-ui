use dioxus::prelude::*;
use tw_merge::tw_merge;

#[component]
pub fn Slider(
    #[props(into, optional)] class: Option<String>,
    #[props(default = 0.0)] min: f64,
    #[props(default = 100.0)] max: f64,
    #[props(default = 1.0)] step: f64,
    #[props(default = 50.0)] value: f64,
    #[props(optional)] disabled: bool,
    #[props(optional)] oninput: Option<EventHandler<FormEvent>>,
) -> Element {
    let merged = tw_merge!(
        "w-full h-2 appearance-none rounded-full bg-secondary cursor-pointer accent-primary disabled:opacity-50 disabled:cursor-not-allowed",
        class.as_deref().unwrap_or("")
    );
    rsx! {
        input {
            r#type: "range",
            class: "{merged}",
            min: "{min}",
            max: "{max}",
            step: "{step}",
            value: "{value}",
            disabled: disabled,
            oninput: move |e| {
                if let Some(handler) = &oninput {
                    handler.call(e);
                }
            },
        }
    }
}
