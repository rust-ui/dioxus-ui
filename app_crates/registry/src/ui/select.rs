use dioxus::prelude::*;
use icons::{Check, ChevronDown, ChevronUp};
use tw_merge::tw_merge;

use crate::hooks::use_can_scroll_vertical::use_can_scroll_vertical;
use crate::hooks::use_random::use_random_id_for;

/* ========================================================== */
/*                     ✨ CONTEXT ✨                          */
/* ========================================================== */

#[derive(Clone)]
struct SelectContext {
    target_id: String,
    value: Signal<Option<String>>,
    on_change: EventHandler<Option<String>>,
}

/* ========================================================== */
/*                     ✨ SELECT ✨                           */
/* ========================================================== */

#[component]
pub fn Select(
    children: Element,
    #[props(into, optional)] class: Option<String>,
    #[props(optional)] default_value: Option<String>,
    #[props(optional)] on_change: EventHandler<Option<String>>,
) -> Element {
    let target_id = use_random_id_for("select");
    let value = use_signal(|| default_value);
    provide_context(SelectContext { target_id, value, on_change });

    let merged = tw_merge!("relative w-full", class.as_deref().unwrap_or(""));
    rsx! {
        div { "data-name": "Select", class: "{merged}", {children} }
    }
}

/* ========================================================== */
/*                   ✨ SELECT TRIGGER ✨                     */
/* ========================================================== */

#[component]
pub fn SelectTrigger(children: Element, #[props(into, optional)] class: Option<String>) -> Element {
    let ctx = use_context::<SelectContext>();
    let merged = tw_merge!(
        "w-full px-3 h-9 inline-flex items-center justify-between text-sm font-medium whitespace-nowrap rounded-md transition-colors focus:outline-none focus:ring-1 focus:ring-ring [&_svg:not([class*='size-'])]:size-4 border bg-background border-input hover:bg-accent hover:text-accent-foreground",
        class.as_deref().unwrap_or("")
    );
    rsx! {
        button {
            type: "button",
            class: "{merged}",
            "data-name": "SelectTrigger",
            "data-select-trigger": "{ctx.target_id}",
            tabindex: "0",
            {children}
            ChevronDown { class: "text-muted-foreground shrink-0 ml-2" }
        }
    }
}

/* ========================================================== */
/*                   ✨ SELECT CONTENT ✨                     */
/* ========================================================== */

#[component]
pub fn SelectContent(children: Element, #[props(into, optional)] class: Option<String>) -> Element {
    let ctx = use_context::<SelectContext>();
    let (on_scroll, can_scroll_up, can_scroll_down) = use_can_scroll_vertical();

    let merged = tw_merge!(
        "overflow-auto z-50 p-1 rounded-md border bg-card shadow-md h-fit max-h-[300px] absolute top-[calc(100%+4px)] left-0 transition-all duration-200 data-[state=closed]:opacity-0 data-[state=closed]:scale-95 data-[state=closed]:pointer-events-none data-[state=open]:opacity-100 data-[state=open]:scale-100 origin-top [scrollbar-width:none] [&::-webkit-scrollbar]:hidden",
        class.as_deref().unwrap_or("")
    );

    let target_id = ctx.target_id.clone();
    let script = format!(
        r#"(function() {{
            const setup = () => {{
                const content = document.querySelector('#{tid}');
                const trigger = document.querySelector('[data-select-trigger="{tid}"]');
                if (!content || !trigger) {{ setTimeout(setup, 50); return; }}
                if (content.hasAttribute('data-initialized')) return;
                content.setAttribute('data-initialized', 'true');
                let isOpen = false;
                const open = () => {{
                    isOpen = true;
                    content.style.minWidth = trigger.getBoundingClientRect().width + 'px';
                    content.setAttribute('data-state', 'open');
                    content.dispatchEvent(new Event('scroll'));
                    if (window.ScrollLock) window.ScrollLock.lock();
                    setTimeout(() => document.addEventListener('click', onClickOutside), 0);
                }};
                const close = () => {{
                    isOpen = false;
                    content.setAttribute('data-state', 'closed');
                    document.removeEventListener('click', onClickOutside);
                    if (window.ScrollLock) window.ScrollLock.unlock(200);
                }};
                const onClickOutside = (e) => {{
                    if (!content.contains(e.target) && !trigger.contains(e.target)) close();
                }};
                trigger.addEventListener('click', (e) => {{
                    e.stopPropagation();
                    if (isOpen) close(); else open();
                }});
                content.querySelectorAll('[data-select-option]').forEach(opt => {{
                    opt.addEventListener('click', () => close());
                }});
                document.addEventListener('keydown', (e) => {{
                    if (e.key === 'Escape' && isOpen) {{ e.preventDefault(); close(); }}
                }});
            }};
            if (document.readyState === 'loading') document.addEventListener('DOMContentLoaded', setup);
            else setup();
        }})();"#,
        tid = target_id,
    );

    rsx! {
        div {
            "data-name": "SelectContent",
            id: "{ctx.target_id}",
            class: "{merged}",
            "data-target": "target__select",
            "data-state": "closed",
            onscroll: on_scroll,
            div {
                class: if can_scroll_up() { "sticky -top-1 z-10 flex items-center justify-center py-1 bg-card" } else { "hidden" },
                ChevronUp { class: "size-4 text-muted-foreground" }
            }
            {children}
            div {
                class: if can_scroll_down() { "sticky -bottom-1 z-10 flex items-center justify-center py-1 bg-card" } else { "hidden" },
                ChevronDown { class: "size-4 text-muted-foreground" }
            }
        }
        script { dangerous_inner_html: "{script}" }
    }
}

/* ========================================================== */
/*                    ✨ SELECT GROUP ✨                      */
/* ========================================================== */

#[component]
pub fn SelectGroup(
    children: Element,
    #[props(into, optional)] class: Option<String>,
    #[props(into, optional)] aria_label: Option<String>,
) -> Element {
    let merged = tw_merge!("group", class.as_deref().unwrap_or(""));
    rsx! {
        ul {
            "data-name": "SelectGroup",
            role: "listbox",
            "aria-label": aria_label.as_deref().unwrap_or("Select options"),
            class: "{merged}",
            {children}
        }
    }
}

/* ========================================================== */
/*                   ✨ SELECT OPTION ✨                      */
/* ========================================================== */

#[component]
pub fn SelectOption(
    children: Element,
    #[props(into, optional)] class: Option<String>,
    #[props(into, optional)] value: Option<String>,
) -> Element {
    let ctx = use_context::<SelectContext>();
    let mut value_signal = ctx.value;
    let on_change = ctx.on_change;
    let val = value.clone();
    let val_check = value.clone();

    let is_selected = move || *value_signal.read() == val_check;

    let merged = tw_merge!(
        "group inline-flex gap-2 items-center w-full rounded-sm px-2 py-1.5 text-sm cursor-pointer transition-colors duration-200 text-popover-foreground hover:bg-accent hover:text-accent-foreground [&_svg:not([class*='size-'])]:size-4",
        class.as_deref().unwrap_or("")
    );

    rsx! {
        li {
            "data-name": "SelectOption",
            class: "{merged}",
            role: "option",
            tabindex: "0",
            "aria-selected": if is_selected() { "true" } else { "false" },
            "data-select-option": "true",
            onclick: move |_| {
                value_signal.set(val.clone());
                on_change.call(val.clone());
            },
            {children}
            Check { class: "ml-auto opacity-0 size-4 text-muted-foreground group-aria-selected:opacity-100" }
        }
    }
}
