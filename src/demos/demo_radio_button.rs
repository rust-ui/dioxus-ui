use dioxus::prelude::*;

use crate::ui::label::Label;
use crate::ui::radio_group::{RadioGroup, RadioItem};

const FREQUENCIES: &[&str] = &["Daily", "Weekly", "Monthly"];

#[component]
pub fn DemoRadioButton() -> Element {
    let mut selected = use_signal(|| "Weekly".to_string());

    rsx! {
        RadioGroup {
            for frequency in FREQUENCIES.iter() {
                div { class: "flex gap-3 items-center",
                    RadioItem {
                        name: "frequency",
                        value: *frequency,
                        checked: selected() == *frequency,
                        onchange: {
                            let frequency = frequency.to_string();
                            move |_| selected.set(frequency.clone())
                        },
                    }
                    Label { r#for: *frequency, {*frequency} }
                }
            }
        }
    }
}
