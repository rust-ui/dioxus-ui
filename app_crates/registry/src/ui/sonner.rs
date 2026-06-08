use dioxus::prelude::*;
use tw_merge::tw_merge;

#[derive(Clone, Copy, PartialEq, Eq, Default, strum::Display)]
pub enum ToastType {
    #[default]
    Default,
    Success,
    Error,
    Warning,
    Info,
    Loading,
}

#[derive(Clone, Copy, PartialEq, Eq, Default, strum::Display)]
pub enum SonnerPosition {
    TopLeft,
    TopCenter,
    TopRight,
    #[default]
    BottomRight,
    BottomCenter,
    BottomLeft,
}

#[derive(Clone, Copy, PartialEq, Eq, Default, strum::Display)]
pub enum SonnerDirection {
    TopDown,
    #[default]
    BottomUp,
}

#[component]
pub fn SonnerTrigger(
    children: Element,
    #[props(into, optional)] class: Option<String>,
    #[props(default = ToastType::Default)] variant: ToastType,
    #[props(into)] title: String,
    #[props(into)] description: String,
    #[props(into, optional)] position: Option<String>,
) -> Element {
    let merged_class = tw_merge!(
        "inline-flex gap-2 items-center px-4 py-2 h-9 text-sm font-medium whitespace-nowrap rounded-md transition-colors bg-primary text-primary-foreground hover:bg-primary/90 focus:outline-hidden focus:ring-2 focus:ring-ring focus:ring-offset-2 cursor-pointer",
        class.as_deref().unwrap_or("")
    );

    rsx! {
        button {
            class: "{merged_class}",
            "data-name": "SonnerTrigger",
            "data-variant": "{variant}",
            "data-toast-title": "{title}",
            "data-toast-description": "{description}",
            "data-toast-position": position,
            r#type: "button",
            {children}
        }
    }
}

#[component]
pub fn SonnerContainer(
    children: Element,
    #[props(into, optional)] class: Option<String>,
    #[props(default = SonnerPosition::BottomRight)] position: SonnerPosition,
) -> Element {
    let merged_class = tw_merge!("toast__container fixed z-50", class.as_deref().unwrap_or(""));

    rsx! {
        div {
            class: "{merged_class}",
            "data-position": "{position}",
            {children}
        }
    }
}

#[component]
pub fn SonnerList(
    children: Element,
    #[props(into, optional)] class: Option<String>,
    #[props(default = SonnerPosition::BottomRight)] position: SonnerPosition,
    #[props(default = SonnerDirection::BottomUp)] direction: SonnerDirection,
    #[props(into, optional)] expanded: Option<String>,
    #[props(into, optional)] style: Option<String>,
) -> Element {
    let merged_class = tw_merge!(
        "flex relative flex-col opacity-100 gap-[15px] h-[100px] w-[400px] pointer-events-none [&>*]:pointer-events-auto",
        class.as_deref().unwrap_or("")
    );
    let expanded_val = expanded.as_deref().unwrap_or("false");

    rsx! {
        ol {
            class: "{merged_class}",
            "data-name": "SonnerList",
            "data-sonner-toaster": "true",
            "data-sonner-theme": "light",
            "data-position": "{position}",
            "data-expanded": "{expanded_val}",
            "data-direction": "{direction}",
            style: style,
            {children}
        }
    }
}

#[component]
pub fn SonnerToaster(#[props(default = SonnerPosition::BottomRight)] position: SonnerPosition) -> Element {
    let direction = match position {
        SonnerPosition::TopLeft | SonnerPosition::TopCenter | SonnerPosition::TopRight => SonnerDirection::TopDown,
        _ => SonnerDirection::BottomUp,
    };

    let container_class = match position {
        SonnerPosition::TopLeft => "left-6 top-6",
        SonnerPosition::TopRight => "right-6 top-6",
        SonnerPosition::TopCenter => "left-1/2 -translate-x-1/2 top-6",
        SonnerPosition::BottomCenter => "left-1/2 -translate-x-1/2 bottom-6",
        SonnerPosition::BottomLeft => "left-6 bottom-6",
        SonnerPosition::BottomRight => "right-6 bottom-6",
    };

    rsx! {
        SonnerContainer { class: container_class, position,
            SonnerList { position, direction, "" }
        }
    }
}
