use dioxus::prelude::*;
use tw_merge::tw_merge;

#[component]
pub fn Breadcrumb(
    #[props(into, optional)] class: Option<String>,
    children: Element,
) -> Element {
    rsx! {
        nav { "aria-label": "breadcrumb", class: "{class.as_deref().unwrap_or(\"\")}",
            {children}
        }
    }
}

#[component]
pub fn BreadcrumbList(
    #[props(into, optional)] class: Option<String>,
    children: Element,
) -> Element {
    let merged = tw_merge!(
        "flex flex-wrap items-center gap-1.5 text-sm text-muted-foreground sm:gap-2",
        class.as_deref().unwrap_or("")
    );
    rsx! { ol { class: "{merged}", {children} } }
}

#[component]
pub fn BreadcrumbItem(
    #[props(into, optional)] class: Option<String>,
    children: Element,
) -> Element {
    let merged = tw_merge!("inline-flex items-center gap-1.5", class.as_deref().unwrap_or(""));
    rsx! { li { class: "{merged}", {children} } }
}

#[component]
pub fn BreadcrumbLink(
    #[props(into)] href: String,
    #[props(into, optional)] class: Option<String>,
    children: Element,
) -> Element {
    let merged = tw_merge!("transition-colors hover:text-foreground", class.as_deref().unwrap_or(""));
    rsx! { a { class: "{merged}", href: "{href}", {children} } }
}

#[component]
pub fn BreadcrumbPage(
    #[props(into, optional)] class: Option<String>,
    children: Element,
) -> Element {
    let merged = tw_merge!("font-normal text-foreground", class.as_deref().unwrap_or(""));
    rsx! {
        span {
            class: "{merged}",
            role: "link",
            "aria-disabled": "true",
            "aria-current": "page",
            {children}
        }
    }
}

#[component]
pub fn BreadcrumbSeparator(
    #[props(into, optional)] class: Option<String>,
) -> Element {
    let merged = tw_merge!("text-muted-foreground/50", class.as_deref().unwrap_or(""));
    rsx! {
        li { class: "{merged}", role: "presentation", "aria-hidden": "true",
            svg {
                xmlns: "http://www.w3.org/2000/svg",
                class: "size-3.5",
                view_box: "0 0 24 24",
                fill: "none",
                stroke: "currentColor",
                stroke_width: "2",
                stroke_linecap: "round",
                stroke_linejoin: "round",
                path { d: "m9 18 6-6-6-6" }
            }
        }
    }
}
