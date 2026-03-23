use dioxus::document::eval;
use dioxus::prelude::*;
use tw_merge::tw_merge;

#[component]
pub fn Dialog(
    #[props(into, optional)] class: Option<String>,
    open: Signal<bool>,
    children: Element,
) -> Element {
    // Lock/unlock body scroll when open state changes
    use_effect(move || {
        if open() {
            eval("if (window.ScrollLock) window.ScrollLock.lock();");
        } else {
            eval("if (window.ScrollLock) window.ScrollLock.unlock(200);");
        }
    });

    let merged = tw_merge!("w-fit", class.as_deref().unwrap_or(""));

    rsx! {
        div { class: "{merged}", {children} }
    }
}

#[component]
pub fn DialogTrigger(
    #[props(into, optional)] class: Option<String>,
    onclick: EventHandler<MouseEvent>,
    children: Element,
) -> Element {
    rsx! {
        button {
            class: tw_merge!(
                "inline-flex items-center justify-center gap-2 whitespace-nowrap rounded-md text-sm font-medium border bg-background shadow-xs hover:bg-accent hover:text-accent-foreground h-9 px-4 py-2 cursor-pointer",
                class.as_deref().unwrap_or("")
            ),
            onclick: move |e| onclick.call(e),
            {children}
        }
    }
}

#[component]
pub fn DialogContent(
    #[props(into, optional)] class: Option<String>,
    open: Signal<bool>,
    children: Element,
) -> Element {
    let state = if open() { "open" } else { "closed" };

    let merged = tw_merge!(
        "relative bg-background border rounded-2xl shadow-lg p-6 w-full max-w-[calc(100%-2rem)] sm:max-w-[425px] max-h-[85vh] overflow-y-auto fixed top-1/2 left-1/2 -translate-x-1/2 -translate-y-1/2 z-50 transition-all duration-200 data-[state=closed]:opacity-0 data-[state=closed]:scale-95 data-[state=open]:opacity-100 data-[state=open]:scale-100",
        class.as_deref().unwrap_or("")
    );

    rsx! {
        // Backdrop
        div {
            class: "fixed inset-0 z-40 bg-black/50 transition-opacity duration-200 data-[state=closed]:opacity-0 data-[state=open]:opacity-100",
            "data-state": "{state}",
            style: if open() { "" } else { "pointer-events: none;" },
            onclick: move |_| open.set(false),
        }
        // Panel
        div {
            class: "{merged}",
            "data-state": "{state}",
            style: if open() { "" } else { "pointer-events: none;" },
            {children}
        }
    }
}

#[component]
pub fn DialogClose(
    #[props(into, optional)] class: Option<String>,
    open: Signal<bool>,
    children: Element,
) -> Element {
    let merged = tw_merge!(
        "inline-flex items-center justify-center gap-2 whitespace-nowrap rounded-md text-sm font-medium border bg-background shadow-xs hover:bg-accent hover:text-accent-foreground h-9 px-4 py-2 cursor-pointer",
        class.as_deref().unwrap_or("")
    );
    rsx! {
        button { class: "{merged}", onclick: move |_| open.set(false), {children} }
    }
}

#[component]
pub fn DialogHeader(
    #[props(into, optional)] class: Option<String>,
    children: Element,
) -> Element {
    let merged = tw_merge!("flex flex-col gap-2 text-center sm:text-left", class.as_deref().unwrap_or(""));
    rsx! { div { class: "{merged}", {children} } }
}

#[component]
pub fn DialogTitle(
    #[props(into, optional)] class: Option<String>,
    children: Element,
) -> Element {
    let merged = tw_merge!("text-lg leading-none font-semibold", class.as_deref().unwrap_or(""));
    rsx! { h3 { class: "{merged}", {children} } }
}

#[component]
pub fn DialogDescription(
    #[props(into, optional)] class: Option<String>,
    children: Element,
) -> Element {
    let merged = tw_merge!("text-muted-foreground text-sm", class.as_deref().unwrap_or(""));
    rsx! { p { class: "{merged}", {children} } }
}

#[component]
pub fn DialogBody(
    #[props(into, optional)] class: Option<String>,
    children: Element,
) -> Element {
    let merged = tw_merge!("flex flex-col gap-4", class.as_deref().unwrap_or(""));
    rsx! { div { class: "{merged}", {children} } }
}

#[component]
pub fn DialogFooter(
    #[props(into, optional)] class: Option<String>,
    children: Element,
) -> Element {
    let merged = tw_merge!(
        "flex flex-col-reverse gap-2 sm:flex-row sm:justify-end",
        class.as_deref().unwrap_or("")
    );
    rsx! { footer { class: "{merged}", {children} } }
}
