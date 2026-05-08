use dioxus::prelude::*;

use crate::ui::animate::{Animate, AnimateHoverVariant};
use crate::ui::badge::{Badge, BadgeVariant};

const HOVER_VARIANTS: &[(&str, AnimateHoverVariant)] = &[
    ("Bounce H", AnimateHoverVariant::BounceHorizontal),
    ("Bounce V", AnimateHoverVariant::BounceVertical),
    ("Float", AnimateHoverVariant::Float),
    ("Heartbeat", AnimateHoverVariant::Heartbeat),
    ("Jiggle", AnimateHoverVariant::Jiggle),
    ("Jump", AnimateHoverVariant::Jump),
    ("Pop", AnimateHoverVariant::Pop),
    ("Rotate 360", AnimateHoverVariant::Rotate360),
    ("RubberBand", AnimateHoverVariant::RubberBand),
    ("Shake", AnimateHoverVariant::Shake),
    ("Swing", AnimateHoverVariant::Swing),
    ("Tada", AnimateHoverVariant::Tada),
    ("Wobble", AnimateHoverVariant::Wobble),
    ("Zoom In", AnimateHoverVariant::ZoomIn),
    ("Zoom Out", AnimateHoverVariant::ZoomOut),
    ("Flash", AnimateHoverVariant::Flash),
];

#[component]
pub fn DemoAnimate() -> Element {
    rsx! {
        div { class: "flex flex-wrap gap-3 justify-center",
            for (label, variant) in HOVER_VARIANTS {
                Animate { hover_variant: *variant, class: "w-auto cursor-pointer",
                    Badge { variant: BadgeVariant::Outline, "{label}" }
                }
            }
        }
    }
}
