use dioxus::prelude::*;

/// Internal navigation link that routes through Dioxus instead of relying on
/// the platform WebView to resolve relative hrefs.
///
/// TODO: Replace this workaround with a cleaner cross-platform internal-link
/// solution once WRY/iOS navigation is handled centrally.
#[component]
pub fn MobileLink(
    #[props(into)] href: String,
    #[props(into, optional)] class: Option<String>,
    #[props(into, optional)] id: Option<String>,
    #[props(into, optional)] data_name: Option<String>,
    #[props(into, optional)] data_state: Option<String>,
    #[props(into, optional)] aria_label: Option<String>,
    #[props(into, optional)] aria_expanded: Option<String>,
    #[props(into, optional)] aria_controls: Option<String>,
    children: Element,
) -> Element {
    let nav = use_navigator();

    rsx! {
        a {
            id: id.as_deref(),
            "data-name": data_name.as_deref().unwrap_or("MobileLink"),
            "data-state": data_state.as_deref().unwrap_or(""),
            class: class.unwrap_or_default(),
            href: "{href}",
            "aria-label": aria_label.as_deref().unwrap_or(""),
            "aria-expanded": aria_expanded.as_deref().unwrap_or(""),
            "aria-controls": aria_controls.as_deref().unwrap_or(""),
            onclick: move |e| {
                e.prevent_default();
                nav.push(href.clone());
            },
            {children}
        }
    }
}
