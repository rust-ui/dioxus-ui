use dioxus::prelude::*;
use tw_merge::tw_merge;

#[component]
pub fn AlertDialog(
    #[props(into, optional)] class: Option<String>,
    open: Signal<bool>,
    children: Element,
) -> Element {
    if !open() {
        return rsx! {};
    }
    rsx! {
        div { class: "fixed inset-0 z-50 flex items-center justify-center",
            // Backdrop
            div { class: "fixed inset-0 bg-black/50 backdrop-blur-sm" }
            // Dialog panel
            div {
                class: tw_merge!(
                    "relative z-50 w-full max-w-md rounded-xl border bg-background p-6 shadow-lg",
                    class.as_deref().unwrap_or("")
                ),
                {children}
            }
        }
    }
}

#[component]
pub fn AlertDialogHeader(
    #[props(into, optional)] class: Option<String>,
    children: Element,
) -> Element {
    let merged = tw_merge!("flex flex-col gap-2 mb-4", class.as_deref().unwrap_or(""));
    rsx! { div { class: "{merged}", {children} } }
}

#[component]
pub fn AlertDialogTitle(
    #[props(into, optional)] class: Option<String>,
    children: Element,
) -> Element {
    let merged = tw_merge!("text-lg font-semibold", class.as_deref().unwrap_or(""));
    rsx! { h2 { class: "{merged}", {children} } }
}

#[component]
pub fn AlertDialogDescription(
    #[props(into, optional)] class: Option<String>,
    children: Element,
) -> Element {
    let merged = tw_merge!("text-sm text-muted-foreground", class.as_deref().unwrap_or(""));
    rsx! { p { class: "{merged}", {children} } }
}

#[component]
pub fn AlertDialogFooter(
    #[props(into, optional)] class: Option<String>,
    children: Element,
) -> Element {
    let merged = tw_merge!(
        "flex justify-end gap-2 mt-4",
        class.as_deref().unwrap_or("")
    );
    rsx! { div { class: "{merged}", {children} } }
}
