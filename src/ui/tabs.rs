use dioxus::prelude::*;
use tw_merge::tw_merge;

#[component]
pub fn Tabs(
    #[props(into)] default_value: String,
    #[props(into, default)] class: Option<String>,
    children: Element,
) -> Element {
    let active = use_signal(|| default_value);
    use_context_provider(|| active);
    let class = tw_merge!("flex flex-col gap-2", class.as_deref().unwrap_or(""));
    rsx! {
        div { class: "{class}", {children} }
    }
}

#[component]
pub fn TabsList(
    #[props(into, default)] class: Option<String>,
    children: Element,
) -> Element {
    let class = tw_merge!(
        "inline-flex h-10 items-center justify-center rounded-md bg-muted p-1 text-muted-foreground",
        class.as_deref().unwrap_or("")
    );
    rsx! {
        div { role: "tablist", class: "{class}", {children} }
    }
}

#[component]
pub fn TabsTrigger(
    value: String,
    #[props(into, default)] class: Option<String>,
    children: Element,
) -> Element {
    let mut active: Signal<String> = use_context();
    let is_active = active() == value;
    let state = if is_active { "active" } else { "inactive" };
    let active_cls = if is_active {
        "bg-background text-foreground shadow-sm"
    } else {
        "hover:bg-background/50"
    };
    let class = tw_merge!(
        "inline-flex items-center justify-center whitespace-nowrap rounded-sm px-3 py-1.5 text-sm font-medium ring-offset-background transition-all focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring focus-visible:ring-offset-2 disabled:pointer-events-none disabled:opacity-50",
        active_cls,
        class.as_deref().unwrap_or("")
    );
    rsx! {
        button {
            role: "tab",
            "data-state": state,
            aria_selected: "{is_active}",
            class: "{class}",
            onclick: move |_| active.set(value.clone()),
            {children}
        }
    }
}

#[component]
pub fn TabsContent(
    value: String,
    #[props(into, default)] class: Option<String>,
    children: Element,
) -> Element {
    let active: Signal<String> = use_context();
    let class = tw_merge!(
        "mt-2 ring-offset-background focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring focus-visible:ring-offset-2",
        class.as_deref().unwrap_or("")
    );
    rsx! {
        div {
            role: "tabpanel",
            "data-state": if active() == value { "active" } else { "inactive" },
            class: "{class}",
            hidden: active() != value,
            {children}
        }
    }
}
