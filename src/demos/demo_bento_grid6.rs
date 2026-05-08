use dioxus::prelude::*;

use crate::ui::bento_grid::{BentoCell, BentoGrid6, BentoRow};

#[component]
pub fn DemoBentoGrid6() -> Element {
    rsx! {
        div { class: "w-full max-w-[800px]",
            BentoGrid6 {
                BentoRow {
                    BentoCell { "1" }
                }
                BentoRow { class: "md:col-span-2 md:row-span-2 md:h-full",
                    BentoCell { "2" }
                }
                BentoRow {
                    BentoCell { "3" }
                }
                BentoRow {
                    BentoCell { "4" }
                }
                BentoRow { class: "md:col-start-4",
                    BentoCell { "5" }
                }
                BentoRow { class: "md:col-span-4",
                    BentoCell { "6" }
                }
            }
        }
    }
}
