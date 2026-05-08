use dioxus::prelude::*;
use icons::ChevronDown;

use crate::ui::faq_transition::{Faq, FaqContent, FaqDescription, FaqInput, FaqLabel, FaqSection, FaqTitle};

const FAQS: &[(&str, &str, &str)] = &[
    ("faq1", "What is Dioxus?", "Dioxus is a Rust framework for building fullstack web, desktop, and mobile apps with a single codebase."),
    ("faq2", "Is it production-ready?", "Dioxus has been used in production by many teams. It is actively maintained with a growing ecosystem."),
    ("faq3", "Does it support SSR?", "Yes! Dioxus supports server-side rendering, static site generation, and client-side rendering."),
];

#[component]
pub fn DemoFaqTransition() -> Element {
    rsx! {
        Faq {
            FaqTitle { "Frequently Asked Questions" }
            for (id, question, answer) in FAQS {
                FaqSection {
                    FaqInput { id: *id }
                    FaqLabel { html_for: *id,
                        span { class: "font-medium", {*question} }
                        ChevronDown { class: "size-4 shrink-0 transition-transform peer-checked:rotate-180" }
                    }
                    FaqContent {
                        FaqDescription { {*answer} }
                    }
                }
            }
        }
    }
}
