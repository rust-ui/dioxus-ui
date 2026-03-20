use dioxus::prelude::*;

use crate::demos::demo_badge::DemoBadge;
use crate::demos::demo_badge_colors::DemoBadgeColors;
use crate::demos::demo_badge_custom::DemoBadgeCustom;
use crate::demos::demo_badge_variants::DemoBadgeVariants;
use crate::demos::demo_button::DemoButton;
use crate::demos::demo_button_disabled::DemoButtonDisabled;
use crate::demos::demo_button_group::DemoButtonGroup;
use crate::demos::demo_button_group_icon::DemoButtonGroupIcon;
use crate::demos::demo_button_href::DemoButtonHref;
use crate::demos::demo_button_override::DemoButtonOverride;
use crate::demos::demo_button_reactive::DemoButtonReactive;
use crate::demos::demo_button_sizes::DemoButtonSizes;
use crate::demos::demo_button_stateful::DemoButtonStateful;
use crate::demos::demo_button_variants::DemoButtonVariants;
use crate::demos::demo_card::DemoCard;
use crate::demos::demo_card_action::DemoCardAction;
use crate::demos::demo_card_group::DemoCardGroup;
use crate::demos::demo_card_reverse::DemoCardReverse;
use crate::demos::demo_card_sm::DemoCardSm;
use crate::demos::demo_input::DemoInput;
use crate::demos::demo_input_copy::DemoInputCopy;
use crate::demos::demo_separator::DemoSeparator;
use crate::demos::demo_skeleton::DemoSkeleton;
use crate::demos::demo_skeleton_avatar::DemoSkeletonAvatar;
use crate::demos::demo_skeleton_form::DemoSkeletonForm;
use crate::demos::demo_skeleton_image::DemoSkeletonImage;
use crate::demos::demo_skeleton_table::DemoSkeletonTable;
use crate::demos::demo_skeleton_text::DemoSkeletonText;
use crate::demos::demo_spinner::DemoSpinner;
use crate::demos::demo_spinner_button::DemoSpinnerButton;

#[component]
pub fn ComponentPage(name: String) -> Element {
    let (title, description) = match name.as_str() {
        "button" => ("Button", "Displays a button or a component that looks like a button."),
        "card" => ("Card", "Displays a card with header, content, and footer sections."),
        "input" => ("Input", "Displays a form input field for user text entry."),
        "badge" => ("Badge", "Displays a badge or a component that looks like a badge."),
        "separator" => ("Separator", "Visually or semantically separates content."),
        "skeleton" => ("Skeleton", "Use to show a placeholder while content is loading."),
        "spinner" => ("Spinner", "Displays an animated spinner to indicate loading state."),
        _ => ("Not Found", "This component does not exist."),
    };

    rsx! {
        div { class: "flex flex-col gap-8 max-w-2xl",
            div {
                h1 { class: "text-2xl font-bold mb-1", "{title}" }
                p { class: "text-muted-foreground text-sm", "{description}" }
            }
            match name.as_str() {
                "button" => rsx! {
                    DemoButton {}
                    DemoButtonVariants {}
                    DemoButtonSizes {}
                    DemoButtonDisabled {}
                    DemoButtonStateful {}
                    DemoButtonReactive {}
                    DemoButtonOverride {}
                    DemoButtonHref {}
                    DemoButtonGroup {}
                    DemoButtonGroupIcon {}
                },
                "card" => rsx! {
                    DemoCard {}
                    DemoCardAction {}
                    DemoCardGroup {}
                    DemoCardSm {}
                    DemoCardReverse {}
                },
                "input" => rsx! {
                    DemoInput {}
                    DemoInputCopy {}
                },
                "badge" => rsx! {
                    DemoBadge {}
                    DemoBadgeVariants {}
                    DemoBadgeColors {}
                    DemoBadgeCustom {}
                },
                "separator" => rsx! {
                    DemoSeparator {}
                },
                "skeleton" => rsx! {
                    DemoSkeleton {}
                    DemoSkeletonText {}
                    DemoSkeletonAvatar {}
                    DemoSkeletonForm {}
                    DemoSkeletonImage {}
                    DemoSkeletonTable {}
                },
                "spinner" => rsx! {
                    DemoSpinner {}
                    DemoSpinnerButton {}
                },
                _ => rsx! {
                    p { class: "text-muted-foreground", "Component not found." }
                },
            }
        }
    }
}
