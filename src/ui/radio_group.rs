use dioxus::prelude::*;
use tw_merge::tw_merge;

#[component]
pub fn RadioGroup(#[props(into, optional)] class: Option<String>, children: Element) -> Element {
    let merged = tw_merge!("flex flex-col gap-2", class.as_deref().unwrap_or(""));
    rsx! { div { class: "{merged}", role: "radiogroup", {children} } }
}

#[component]
pub fn RadioGroupItem(
    #[props(into)] name: String,
    #[props(into)] value: String,
    #[props(into, optional)] id: Option<String>,
    #[props(optional)] checked: bool,
    #[props(optional)] disabled: bool,
    #[props(optional)] onchange: Option<EventHandler<FormEvent>>,
) -> Element {
    let id_val = id.unwrap_or_else(|| format!("radio-{value}"));
    rsx! {
        input {
            r#type: "radio",
            id: "{id_val}",
            name: "{name}",
            value: "{value}",
            checked: checked,
            disabled: disabled,
            class: "peer sr-only",
            onchange: move |e| {
                if let Some(handler) = &onchange {
                    handler.call(e);
                }
            },
        }
    }
}

#[component]
pub fn RadioGroupLabel(
    #[props(into)] r#for: String,
    #[props(into, optional)] class: Option<String>,
    children: Element,
) -> Element {
    let merged = tw_merge!(
        "flex items-center gap-2 cursor-pointer text-sm",
        class.as_deref().unwrap_or("")
    );
    let for_val = r#for;
    rsx! {
        label { class: "{merged}", r#for: "{for_val}",
            // Visual radio circle
            span { class: "flex h-4 w-4 items-center justify-center rounded-full border border-input bg-background peer-checked:border-primary peer-checked:bg-primary transition-colors",
                span { class: "h-1.5 w-1.5 rounded-full bg-background" }
            }
            {children}
        }
    }
}

/// Convenience: wraps input + label together.
#[component]
pub fn RadioItem(
    #[props(into)] name: String,
    #[props(into)] value: String,
    #[props(optional)] checked: bool,
    #[props(optional)] disabled: bool,
    #[props(optional)] onchange: Option<EventHandler<FormEvent>>,
    children: Element,
) -> Element {
    let id = format!("radio-{name}-{value}");
    let id2 = id.clone();
    rsx! {
        div { class: "flex items-center gap-2",
            input {
                r#type: "radio",
                id: "{id}",
                name: "{name}",
                value: "{value}",
                checked: checked,
                disabled: disabled,
                class: "h-4 w-4 border-input accent-primary cursor-pointer",
                onchange: move |e| {
                    if let Some(handler) = &onchange {
                        handler.call(e);
                    }
                },
            }
            label { class: "text-sm cursor-pointer", r#for: "{id2}", {children} }
        }
    }
}
