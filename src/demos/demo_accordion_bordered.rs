use dioxus::prelude::*;

use crate::ui::accordion::{Accordion, AccordionContent, AccordionItem, AccordionTrigger};

#[component]
pub fn DemoAccordionBordered() -> Element {
    rsx! {
        div { class: "w-full max-w-md",
            Accordion { class: "overflow-hidden rounded-lg border bg-background",
                AccordionItem {
                    AccordionTrigger { class: "hover:bg-accent", "Accordion item 01" }
                    AccordionContent { "This is the Accordion description" }
                }
                AccordionItem {
                    AccordionTrigger { class: "hover:bg-accent", "Accordion item 02" }
                    AccordionContent { "This is the Accordion description" }
                }
                AccordionItem {
                    AccordionTrigger { class: "hover:bg-accent", "Accordion item 03" }
                    AccordionContent { "This is the Accordion description" }
                }
            }
        }
    }
}
