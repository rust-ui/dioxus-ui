use dioxus::prelude::*;

use crate::ui::accordion::{Accordion, AccordionContent, AccordionItem, AccordionTrigger};

#[component]
pub fn DemoAccordion() -> Element {
    let mut open1 = use_signal(|| false);
    let mut open2 = use_signal(|| false);
    let mut open3 = use_signal(|| false);

    rsx! {
        div { class: "w-full max-w-md",
            Accordion {
                AccordionItem {
                    AccordionTrigger {
                        open: open1,
                        onclick: move |_| open1.toggle(),
                        "Is it accessible?"
                    }
                    AccordionContent {
                        open: open1,
                        "Yes. It adheres to the WAI-ARIA design pattern."
                    }
                }
                AccordionItem {
                    AccordionTrigger {
                        open: open2,
                        onclick: move |_| open2.toggle(),
                        "Is it styled?"
                    }
                    AccordionContent {
                        open: open2,
                        "Yes. It comes with default styles that match the other components."
                    }
                }
                AccordionItem {
                    AccordionTrigger {
                        open: open3,
                        onclick: move |_| open3.toggle(),
                        "Is it animated?"
                    }
                    AccordionContent {
                        open: open3,
                        "Yes. It uses Dioxus signals for smooth show/hide transitions."
                    }
                }
            }
        }
    }
}
