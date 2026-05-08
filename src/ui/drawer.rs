use std::sync::atomic::{AtomicUsize, Ordering};

use dioxus::prelude::*;
use tw_merge::tw_merge;

static DRAWER_COUNTER: AtomicUsize = AtomicUsize::new(0);

#[derive(Clone, PartialEq)]
struct DrawerContext {
    target_id: String,
}

#[component]
pub fn Drawer(#[props(into, optional)] class: Option<String>, children: Element) -> Element {
    let drawer_id = use_hook(|| {
        let id = DRAWER_COUNTER.fetch_add(1, Ordering::Relaxed);
        format!("drawer_{id}")
    });
    provide_context(DrawerContext { target_id: drawer_id });
    let c = tw_merge!("w-fit", class.as_deref().unwrap_or(""));
    rsx! { div { "data-name": "Drawer", class: "{c}", {children} } }
}

#[component]
pub fn DrawerTrigger(#[props(into, optional)] class: Option<String>, children: Element) -> Element {
    let ctx = use_context::<DrawerContext>();
    rsx! {
        button {
            "data-name": "DrawerTrigger",
            "data-drawer-trigger": "{ctx.target_id}",
            class: tw_merge!(
                "inline-flex items-center justify-center gap-2 whitespace-nowrap rounded-md text-sm font-medium border bg-background shadow-xs hover:bg-accent hover:text-accent-foreground h-9 px-4 py-2 cursor-pointer",
                class.as_deref().unwrap_or("")
            ),
            {children}
        }
    }
}

#[component]
pub fn DrawerContent(
    #[props(into, optional)] class: Option<String>,
    #[props(default = true)] close_on_backdrop_click: bool,
    children: Element,
) -> Element {
    let ctx = use_context::<DrawerContext>();
    let target_id = ctx.target_id.clone();
    let backdrop_id = format!("{}_backdrop", target_id);
    let backdrop_behavior = if close_on_backdrop_click { "auto" } else { "manual" };

    let c = tw_merge!(
        "fixed inset-x-0 bottom-0 z-100 flex flex-col bg-background rounded-t-[10px] pt-3 pb-6 px-6 max-h-[90vh] overflow-auto shadow-lg border-t transition-transform duration-300 ease-in-out data-[state=closed]:translate-y-full data-[state=open]:translate-y-0",
        class.as_deref().unwrap_or("")
    );

    let tid = target_id.clone();
    let bid = backdrop_id.clone();
    let script = format!(
        r#"(function() {{
        const setup = () => {{
            const drawer = document.getElementById('{tid}');
            const backdrop = document.getElementById('{bid}');
            const trigger = document.querySelector('[data-drawer-trigger="{tid}"]');
            if (!drawer || !backdrop || !trigger) {{ setTimeout(setup, 50); return; }}
            if (drawer.hasAttribute('data-initialized')) return;
            drawer.setAttribute('data-initialized', 'true');
            const open = () => {{
                drawer.setAttribute('data-state', 'open');
                backdrop.setAttribute('data-state', 'open');
                drawer.style.pointerEvents = 'auto';
                backdrop.style.pointerEvents = 'auto';
                if (window.ScrollLock) window.ScrollLock.lock();
            }};
            const close = () => {{
                drawer.setAttribute('data-state', 'closed');
                backdrop.setAttribute('data-state', 'closed');
                drawer.style.pointerEvents = 'none';
                backdrop.style.pointerEvents = 'none';
                if (window.ScrollLock) window.ScrollLock.unlock(300);
            }};
            trigger.addEventListener('click', open);
            drawer.querySelectorAll('[data-drawer-close]').forEach(b => b.addEventListener('click', close));
            backdrop.addEventListener('click', () => {{ if (drawer.getAttribute('data-backdrop') === 'auto') close(); }});
            document.addEventListener('keydown', e => {{ if (e.key === 'Escape' && drawer.getAttribute('data-state') === 'open') {{ e.preventDefault(); close(); }} }});
        }};
        if (document.readyState === 'loading') {{ document.addEventListener('DOMContentLoaded', setup); }} else {{ setup(); }}
    }})();"#
    );

    rsx! {
        div {
            "data-name": "DrawerBackdrop",
            id: "{backdrop_id}",
            class: "fixed inset-0 z-60 bg-black/50 pointer-events-none opacity-0 transition-opacity duration-300 data-[state=open]:opacity-100",
            "data-state": "closed",
            "data-backdrop": "{backdrop_behavior}",
        }
        div {
            "data-name": "DrawerContent",
            id: "{target_id}",
            class: "{c}",
            "data-state": "closed",
            style: "pointer-events: none;",
            {children}
        }
        script { dangerous_inner_html: "{script}" }
    }
}

#[component]
pub fn DrawerHandle() -> Element {
    rsx! {
        div { class: "block relative mx-auto mb-6 w-8 rounded-2xl opacity-70 hover:opacity-100 shrink-0 bg-muted-foreground/30 h-[5px]" }
    }
}

#[component]
pub fn DrawerClose(#[props(into, optional)] class: Option<String>, children: Element) -> Element {
    let ctx = use_context::<DrawerContext>();
    rsx! {
        button {
            "data-name": "DrawerClose",
            "data-drawer-close": "{ctx.target_id}",
            class: tw_merge!(
                "inline-flex items-center justify-center gap-2 whitespace-nowrap rounded-md text-sm font-medium border bg-background shadow-xs hover:bg-accent hover:text-accent-foreground h-9 px-4 py-2 cursor-pointer",
                class.as_deref().unwrap_or("")
            ),
            {children}
        }
    }
}

#[component]
pub fn DrawerHeader(#[props(into, optional)] class: Option<String>, children: Element) -> Element {
    let c = tw_merge!("flex flex-col gap-2", class.as_deref().unwrap_or(""));
    rsx! { div { "data-name": "DrawerHeader", class: "{c}", {children} } }
}

#[component]
pub fn DrawerTitle(#[props(into, optional)] class: Option<String>, children: Element) -> Element {
    let c = tw_merge!("text-lg leading-none font-semibold", class.as_deref().unwrap_or(""));
    rsx! { h2 { "data-name": "DrawerTitle", class: "{c}", {children} } }
}

#[component]
pub fn DrawerDescription(
    #[props(into, optional)] class: Option<String>,
    children: Element,
) -> Element {
    let c = tw_merge!("text-sm text-muted-foreground", class.as_deref().unwrap_or(""));
    rsx! { p { "data-name": "DrawerDescription", class: "{c}", {children} } }
}

#[component]
pub fn DrawerBody(#[props(into, optional)] class: Option<String>, children: Element) -> Element {
    let c = tw_merge!("flex flex-col gap-4 py-4 mx-auto w-full max-w-[500px]", class.as_deref().unwrap_or(""));
    rsx! { div { "data-name": "DrawerBody", class: "{c}", {children} } }
}

#[component]
pub fn DrawerFooter(#[props(into, optional)] class: Option<String>, children: Element) -> Element {
    let c = tw_merge!(
        "flex flex-col-reverse gap-2 sm:flex-row sm:justify-end mx-auto w-full max-w-[500px]",
        class.as_deref().unwrap_or("")
    );
    rsx! { footer { "data-name": "DrawerFooter", class: "{c}", {children} } }
}
