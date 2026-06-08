use dioxus::prelude::*;

use crate::ui::bento_grid::{BentoCell, BentoGrid, BentoRow};

#[component]
pub fn DemoBentoGrid5() -> Element {
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
                BentoRow { class: "md:col-start-1", BentoCell { "1" } }
                BentoRow { class: "md:col-span-2", BentoCell { "2" } }
                BentoRow { class: "md:col-start-4", BentoCell { "3" } }
                BentoRow { class: "md:col-span-2", BentoCell { "4" } }
                BentoRow { class: "md:col-span-2", BentoCell { "5" } }
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
                BentoRow { class: "md:col-span-2", BentoCell { "1" } }
                BentoRow { class: "md:col-span-2", BentoCell { "2" } }
                BentoRow { class: "md:col-span-4", BentoCell { "3" } }
                BentoRow { class: "md:col-span-2", BentoCell { "4" } }
                BentoRow { class: "md:col-span-2", BentoCell { "5" } }
            }
        }
    }
}

#[component]
pub fn Variant3() -> Element {
    rsx! {
        div {
            h4 { class: "text-xl font-bold text-pretty", "Variant 3" }

            div { class: "grid gap-2 sm:grid-cols-2 md:grid-cols-9",
                BentoRow { class: "md:col-span-5", BentoCell { "1" } }
                BentoRow { class: "md:col-span-4", BentoCell { "2" } }
                BentoRow { class: "md:col-span-3", BentoCell { "3" } }
                BentoRow { class: "md:col-span-3", BentoCell { "4" } }
                BentoRow { class: "md:col-span-3", BentoCell { "5" } }
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
                BentoRow { class: "", BentoCell { "1" } }
                BentoRow { class: "md:col-span-2 md:row-span-2 md:h-full", BentoCell { "2" } }
                BentoRow { class: "", BentoCell { "3" } }
                BentoRow { class: "", BentoCell { "4" } }
                BentoRow { class: "md:col-start-4", BentoCell { "5" } }
            }
        }
    }
}
