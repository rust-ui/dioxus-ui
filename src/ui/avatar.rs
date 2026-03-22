use dioxus::prelude::*;
use tw_merge::tw_merge;

#[allow(dead_code)]
#[derive(Default, Clone, PartialEq)]
pub enum AvatarSize {
    Sm,
    #[default]
    Default,
    Lg,
}

impl AvatarSize {
    fn as_str(&self) -> &'static str {
        match self {
            AvatarSize::Sm => "size-8 text-xs",
            AvatarSize::Default => "size-10 text-sm",
            AvatarSize::Lg => "size-14 text-base",
        }
    }
}

#[component]
pub fn Avatar(
    #[props(default)] size: AvatarSize,
    #[props(into, default)] class: Option<String>,
    children: Element,
) -> Element {
    let class = tw_merge!(
        "relative flex shrink-0 overflow-hidden rounded-full",
        size.as_str(),
        class.as_deref().unwrap_or("")
    );
    rsx! {
        span { class: "{class}", {children} }
    }
}

#[component]
pub fn AvatarImage(
    src: String,
    #[props(into, default)] alt: Option<String>,
    #[props(into, default)] class: Option<String>,
) -> Element {
    let class = tw_merge!("aspect-square size-full object-cover", class.as_deref().unwrap_or(""));
    rsx! {
        img { class: "{class}", src: "{src}", alt: alt.as_deref().unwrap_or("") }
    }
}

#[component]
pub fn AvatarFallback(
    #[props(into, default)] class: Option<String>,
    children: Element,
) -> Element {
    let class = tw_merge!(
        "flex size-full items-center justify-center rounded-full bg-muted font-medium",
        class.as_deref().unwrap_or("")
    );
    rsx! {
        span { class: "{class}", {children} }
    }
}
