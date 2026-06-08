use dioxus::prelude::*;

use crate::ui::bento_grid::{BentoCell, BentoGrid, BentoRow};

#[component]
pub fn DemoBentoGrid4() -> Element {
    rsx! {
        div { class: "flex flex-col gap-6 py-4 w-full max-w-[800px]",
            Variant1 {}
            Variant2 {}
            Variant3 {}
            Variant4 {}
        }
    }
}

/* ========================================================== */
/*                     ✨ FUNCTIONS ✨                        */
/* ========================================================== */

#[component]
pub fn Variant1() -> Element {
    rsx! {
        div {
            h4 { class: "text-xl font-bold text-pretty", "Variant 1" }

            BentoGrid {
                BentoRow { class: "md:col-span-3", BentoCell { "1" } }
                BentoRow { class: "", BentoCell { "2" } }
                BentoRow { class: "md:col-start-1", BentoCell { "3" } }
                BentoRow { class: "md:col-span-4 md:col-start-2", BentoCell { "4" } }
            }
        }
    }
}

#[component]
pub fn Variant2() -> Element {
    rsx! {
        div {
            h4 { class: "text-xl font-bold text-pretty", "Variant 2" }

            BentoGrid {
                BentoRow { class: "md:col-span-3", BentoCell { "1" } }
                BentoRow { class: "", BentoCell { "2" } }
                BentoRow { class: "md:col-span-2", BentoCell { "3" } }
                BentoRow { class: "md:col-span-2", BentoCell { "4" } }
            }
        }
    }
}

#[component]
pub fn Variant3() -> Element {
    rsx! {
        div {
            h4 { class: "text-xl font-bold text-pretty", "Variant 3" }

            BentoGrid {
                BentoRow { class: "md:col-start-1", BentoCell { "1" } }
                BentoRow { class: "md:col-span-2", BentoCell { "2" } }
                BentoRow { class: "md:col-start-4", BentoCell { "3" } }
                BentoRow { class: "md:col-span-4", BentoCell { "4" } }
            }
        }
    }
}

#[component]
pub fn Variant4() -> Element {
    rsx! {
        div {
            h4 { class: "text-xl font-bold text-pretty", "Variant 4" }

            BentoGrid {
                BentoRow { class: "md:col-span-3 md:row-span-4 md:h-full", BentoCell { "1" } }
                BentoRow { class: "md:col-start-4", BentoCell { "2" } }
                BentoRow { class: "md:col-start-4", BentoCell { "3" } }
                BentoRow { class: "md:col-start-4", BentoCell { "4" } }
            }
        }
    }
}
