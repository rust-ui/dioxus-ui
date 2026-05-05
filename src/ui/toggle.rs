use dioxus::prelude::*;
use tw_merge::tw_merge;

#[derive(Default, Clone, PartialEq)]
pub enum ToggleVariant {
    #[default]
    Default,
    Outline,
}

#[derive(Default, Clone, PartialEq)]
pub enum ToggleSize {
    #[default]
    Default,
    Sm,
    Lg,
}

impl ToggleVariant {
    fn as_str(&self) -> &'static str {
        match self {
            ToggleVariant::Default => {
                "bg-transparent hover:bg-muted hover:text-muted-foreground data-[state=on]:bg-accent data-[state=on]:text-accent-foreground"
            }
            ToggleVariant::Outline => {
                "border border-input bg-transparent hover:bg-accent hover:text-accent-foreground data-[state=on]:bg-accent data-[state=on]:text-accent-foreground"
            }
        }
    }
}

impl ToggleSize {
    fn as_str(&self) -> &'static str {
        match self {
            ToggleSize::Default => "h-9 px-3 min-w-9",
            ToggleSize::Sm => "h-8 px-2 min-w-8 text-xs",
            ToggleSize::Lg => "h-10 px-4 min-w-10",
        }
    }
}

#[component]
pub fn Toggle(
    #[props(into, optional)] class: Option<String>,
    #[props(default = ToggleVariant::default())] variant: ToggleVariant,
    #[props(default = ToggleSize::default())] size: ToggleSize,
    #[props(default = false)] pressed: bool,
    #[props(optional)] disabled: bool,
    #[props(optional)] onclick: Option<EventHandler<MouseEvent>>,
    children: Element,
) -> Element {
    let state = if pressed { "on" } else { "off" };
    let merged = tw_merge!(
        "inline-flex items-center justify-center gap-2 rounded-md text-sm font-medium transition-colors disabled:pointer-events-none disabled:opacity-50 cursor-pointer",
        variant.as_str(),
        size.as_str(),
        class.as_deref().unwrap_or("")
    );
    rsx! {
        button {
            class: "{merged}",
            "data-state": "{state}",
            "aria-pressed": "{pressed}",
            disabled: disabled,
            onclick: move |e| {
                if let Some(handler) = &onclick {
                    handler.call(e);
                }
            },
            {children}
        }
    }
}
