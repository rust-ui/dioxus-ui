use dioxus::prelude::*;

use crate::ui::label::Label;
use crate::ui::radio_group::{RadioGroup, RadioItem};

const BUDGETS: &[&str] = &["<$1K", "$1K - $2K", "$2K - $5K", "$5K - $10K", ">$10K"];

#[component]
pub fn DemoRadioButtonCustom() -> Element {
    let mut selected = use_signal(|| "$2K - $5K".to_string());

    rsx! {
        RadioGroup {
            for budget in BUDGETS.iter() {
                div { class: "flex gap-3 items-center",
                    RadioItem {
                        name: "budget",
                        value: *budget,
                        checked: selected() == *budget,
                        onchange: {
                            let budget = budget.to_string();
                            move |_| selected.set(budget.clone())
                        },
                    }
                    Label { {*budget} }
                }
            }
        }
    }
}
