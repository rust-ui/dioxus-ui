use dioxus::prelude::*;

use crate::ui::button::{Button, ButtonVariant};
use crate::ui::card::{Card, CardContent, CardDescription, CardFooter, CardHeader, CardTitle};

#[component]
pub fn DemoCard() -> Element {
    rsx! {
        div { class: "space-y-6 w-full",
            Card { class: "max-w-lg",
                CardHeader {
                    CardTitle { "Card Title" }
                }
                CardContent {
                    CardDescription {
                        "Hello, this is a more detailed description of the card content. You can add more text here to provide additional information about the card's purpose, features, or any other relevant details that might interest the viewer."
                    }
                }
                CardFooter { class: "justify-end",
                    Button { variant: ButtonVariant::Outline, "Cancel" }
                    Button { "Confirm" }
                }
            }
            Card { class: "flex flex-col gap-6 px-6 md:flex-row",
                div { class: "w-full h-48 rounded-lg md:w-1/3 md:h-32 bg-muted" }
                div { class: "flex-1 pt-0",
                    CardTitle { class: "mb-3", "Nature's Beauty" }
                    CardDescription {
                        "Nature's beauty encompasses a vast array of colors, sounds, and textures that evoke a sense of wonder. Its rhythms and patterns create a calming atmosphere that can rejuvenate the spirit."
                    }
                }
            }
            Card { class: "flex flex-col gap-6 px-6 md:flex-row-reverse",
                div { class: "w-full h-48 rounded-lg md:w-1/3 md:h-32 bg-accent" }
                div { class: "flex-1 pt-0",
                    CardTitle { class: "mb-3", "Ecosystem Balance" }
                    CardDescription {
                        "The intricate balance of ecosystems showcases the interdependence of all living beings. Each element, from the smallest insect to the largest tree, plays a vital role in sustaining life."
                    }
                }
            }
            Card { class: "flex flex-col gap-6 px-6 md:flex-row",
                div { class: "w-full h-48 rounded-lg md:w-1/3 md:h-32 bg-muted" }
                div { class: "flex-1 pt-0",
                    CardTitle { class: "mb-3", "Changing Landscapes" }
                    CardDescription {
                        "The ever-changing landscapes of nature remind us of the passage of time. Seasons bring transformations that create a dynamic environment filled with diverse flora and fauna."
                    }
                }
            }
        }
    }
}
