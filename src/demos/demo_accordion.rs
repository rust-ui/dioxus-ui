use dioxus::prelude::*;

use crate::ui::accordion::{Accordion, AccordionContent, AccordionItem, AccordionTrigger};

#[component]
pub fn DemoAccordion() -> Element {
    rsx! {
        div { class: "w-full max-w-md",
            Accordion {
                AccordionItem {
                    AccordionTrigger { "Is it accessible?" }
                    AccordionContent { "Yes. It adheres to the WAI-ARIA design pattern." }
                }
                AccordionItem {
                    AccordionTrigger { "Is it styled?" }
                    AccordionContent { "Yes. It comes with default styles that match the other components." }
                }
                AccordionItem {
                    AccordionTrigger { "Is it animated?" }
                    AccordionContent { "Yes. It uses CSS grid transitions for smooth show/hide animations." }
                }
            }
        }
    }
}
