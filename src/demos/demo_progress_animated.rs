use dioxus::prelude::*;

use crate::ui::button::Button;
use crate::ui::progress::Progress;

#[component]
pub fn DemoProgressAnimated() -> Element {
    let mut value = use_signal(|| 0.0_f64);

    rsx! {
        div { class: "flex flex-col gap-4 w-full max-w-sm",
            Progress { value: value() }
            div { class: "flex gap-2",
                Button {
                    onclick: move |_| {
                        let v = (value() + 10.0).min(100.0);
                        value.set(v);
                    },
                    "+10%"
                }
                Button {
                    onclick: move |_| value.set(0.0),
                    "Reset"
                }
            }
        }
    }
}
