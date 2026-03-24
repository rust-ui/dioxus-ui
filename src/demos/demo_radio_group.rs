use dioxus::prelude::*;

use crate::ui::label::Label;
use crate::ui::radio_group::{RadioGroup, RadioItem};

const SIZES: &[&str] = &["Small", "Medium", "Large", "Extra Large"];

#[component]
pub fn DemoRadioGroup() -> Element {
    let mut selected = use_signal(|| "Medium".to_string());

    rsx! {
        RadioGroup {
            for size in SIZES.iter() {
                div { class: "flex gap-3 items-center",
                    RadioItem {
                        name: "size",
                        value: *size,
                        checked: selected() == *size,
                        onchange: {
                            let size = size.to_string();
                            move |_| selected.set(size.clone())
                        },
                    }
                    Label { html_for: *size, {*size} }
                }
            }
        }
    }
}
