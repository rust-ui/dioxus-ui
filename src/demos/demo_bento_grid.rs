use dioxus::prelude::*;

use crate::ui::bento_grid::{BentoCell, BentoGrid, BentoRow};

#[component]
pub fn DemoBentoGrid() -> Element {
    rsx! {
        BentoGrid {
            BentoRow { class: "md:col-span-2",
                BentoCell { "1" }
            }
            BentoRow {
                BentoCell { "2" }
            }
            BentoRow {
                BentoCell { "3" }
            }
            BentoRow {
                BentoCell { "4" }
            }
        }
    }
}
