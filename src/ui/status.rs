use dioxus::prelude::*;
use tw_merge::tw_merge;

#[derive(Default, Clone, PartialEq)]
pub enum StatusVariant {
    #[default]
    Default,
    Active,
    Inactive,
    Normal,
}

impl StatusVariant {
    fn color(&self) -> &'static str {
        match self {
            StatusVariant::Default => "bg-neutral-300",
            StatusVariant::Active => "bg-green-400",
            StatusVariant::Inactive => "bg-orange-400",
            StatusVariant::Normal => "bg-sky-400",
        }
    }
}

#[component]
pub fn Status(
    #[props(into, optional)] class: Option<String>,
    #[props(default = StatusVariant::default())] variant: StatusVariant,
    children: Element,
) -> Element {
    let merged = tw_merge!("relative inline-block", class.as_deref().unwrap_or(""));
    let color = variant.color();
    rsx! {
        div { class: "{merged}",
            {children}
            // Ping animation
            span { class: "absolute top-0 right-0 -mt-1 -mr-1 flex size-3",
                span { class: "animate-ping absolute inline-flex size-full rounded-full opacity-75 {color}" }
                span { class: "relative inline-flex rounded-full size-3 {color}" }
            }
        }
    }
}
