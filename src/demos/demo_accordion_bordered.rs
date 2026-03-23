use dioxus::prelude::*;

use crate::ui::accordion::{Accordion, AccordionContent, AccordionItem, AccordionTrigger};

#[component]
pub fn DemoAccordionBordered() -> Element {
    let mut open1 = use_signal(|| true);
    let mut open2 = use_signal(|| false);
    let mut open3 = use_signal(|| false);

    rsx! {
        div { class: "w-full max-w-md",
            Accordion { class: "overflow-hidden rounded-lg border bg-background",
                AccordionItem {
                    AccordionTrigger {
                        class: "hover:bg-accent",
                        open: open1,
                        onclick: move |_| open1.toggle(),
                        "Accordion item 01"
                    }
                    AccordionContent { open: open1, "This is the Accordion description" }
                }
                AccordionItem {
                    AccordionTrigger {
                        class: "hover:bg-accent",
                        open: open2,
                        onclick: move |_| open2.toggle(),
                        "Accordion item 02"
                    }
                    AccordionContent { open: open2, "This is the Accordion description" }
                }
                AccordionItem {
                    AccordionTrigger {
                        class: "hover:bg-accent",
                        open: open3,
                        onclick: move |_| open3.toggle(),
                        "Accordion item 03"
                    }
                    AccordionContent { open: open3, "This is the Accordion description" }
                }
            }
        }
    }
}
