use dioxus::prelude::*;
use tw_merge::tw_merge;

use crate::ui::separator::{Separator, SeparatorOrientation};

#[allow(dead_code)]
#[derive(Default, Clone, PartialEq)]
pub enum ButtonGroupOrientation {
    #[default]
    Horizontal,
    Vertical,
}

impl ButtonGroupOrientation {
    fn as_str(&self) -> &'static str {
        match self {
            ButtonGroupOrientation::Horizontal => "[&>*:not(:first-child)]:rounded-l-none [&>*:not(:first-child)]:border-l-0 [&>*:not(:last-child)]:rounded-r-none",
            ButtonGroupOrientation::Vertical => "flex-col [&>*:not(:first-child)]:rounded-t-none [&>*:not(:first-child)]:border-t-0 [&>*:not(:last-child)]:rounded-b-none",
        }
    }
}

#[component]
pub fn ButtonGroup(
    #[props(into, optional)] class: Option<String>,
    #[props(default = ButtonGroupOrientation::default())] orientation: ButtonGroupOrientation,
    children: Element,
) -> Element {
    let merged_class = tw_merge!(
        "flex w-fit items-stretch [&>*]:focus-visible:z-10 [&>*]:focus-visible:relative",
        orientation.as_str(),
        class.as_deref().unwrap_or("")
    );

    rsx! {
        div { "data-name": "ButtonGroup", role: "group", class: "{merged_class}", {children} }
    }
}

#[component]
pub fn ButtonGroupSeparator(
    #[props(into, optional)] class: Option<String>,
) -> Element {
    let merged = tw_merge!(
        "relative !m-0 self-stretch data-[orientation=vertical]:h-auto",
        class.as_deref().unwrap_or("")
    );
    rsx! {
        Separator {
            class: "{merged}",
            orientation: SeparatorOrientation::Vertical,
        }
    }
}
