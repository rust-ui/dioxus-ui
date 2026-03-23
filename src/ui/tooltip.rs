use dioxus::prelude::*;
use tw_merge::tw_merge;

#[derive(Default, Clone, PartialEq)]
pub enum TooltipPosition {
    #[default]
    Top,
    Bottom,
    Left,
    Right,
}

#[component]
pub fn Tooltip(
    #[props(into, optional)] class: Option<String>,
    children: Element,
) -> Element {
    let merged = tw_merge!(
        "inline-block relative group/tooltip",
        class.as_deref().unwrap_or("")
    );
    rsx! { div { class: "{merged}", {children} } }
}

#[component]
pub fn TooltipContent(
    #[props(into, optional)] class: Option<String>,
    #[props(default = TooltipPosition::default())] position: TooltipPosition,
    children: Element,
) -> Element {
    let base = "absolute z-50 pointer-events-none opacity-0 group-hover/tooltip:opacity-100 transition-opacity duration-200 whitespace-nowrap rounded-md bg-foreground/90 px-2.5 py-1.5 text-xs text-background shadow-lg";

    let pos_class = match position {
        TooltipPosition::Top => "bottom-full left-1/2 -translate-x-1/2 mb-2",
        TooltipPosition::Bottom => "top-full left-1/2 -translate-x-1/2 mt-2",
        TooltipPosition::Left => "right-full top-1/2 -translate-y-1/2 mr-2",
        TooltipPosition::Right => "left-full top-1/2 -translate-y-1/2 ml-2",
    };

    let merged = tw_merge!(base, pos_class, class.as_deref().unwrap_or(""));
    rsx! { div { class: "{merged}", {children} } }
}
