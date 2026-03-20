use dioxus::prelude::*;
use tw_merge::tw_merge;

#[allow(dead_code)]
#[derive(Default, Clone, PartialEq)]
pub enum BadgeVariant {
    #[default]
    Default,
    Secondary,
    Destructive,
    Outline,
    Success,
    Warning,
    Info,
}

impl BadgeVariant {
    fn as_str(&self) -> &'static str {
        match self {
            BadgeVariant::Default => "bg-primary text-primary-foreground",
            BadgeVariant::Secondary => "bg-secondary text-secondary-foreground",
            BadgeVariant::Destructive => "bg-destructive text-white",
            BadgeVariant::Outline => "border text-foreground",
            BadgeVariant::Success => "bg-green-500 text-white",
            BadgeVariant::Warning => "bg-yellow-500 text-white",
            BadgeVariant::Info => "bg-blue-500 text-white",
        }
    }
}

#[component]
pub fn Badge(
    #[props(into, optional)] class: Option<String>,
    #[props(default = BadgeVariant::default())] variant: BadgeVariant,
    children: Element,
) -> Element {
    let merged_class = tw_merge!(
        "inline-flex items-center rounded-full px-2.5 py-0.5 text-xs font-medium transition-colors",
        variant.as_str(),
        class.as_deref().unwrap_or("")
    );

    rsx! {
        span { class: "{merged_class}", {children} }
    }
}
