use std::sync::atomic::{AtomicU64, Ordering};

use dioxus::prelude::*;
use tw_merge::tw_merge;

static POPOVER_COUNTER: AtomicU64 = AtomicU64::new(0);

fn use_popover_id() -> String {
    use_hook(|| {
        let id = POPOVER_COUNTER.fetch_add(1, Ordering::Relaxed);
        format!("{id}")
    })
}

#[derive(Clone)]
struct PopoverContext {
    anchor_name: String,
    trigger_id: String,
    content_id: String,
}

#[derive(Clone, Copy, PartialEq, Default)]
pub enum PopoverSide {
    #[default]
    Bottom,
    Top,
    Left,
    Right,
}

impl PopoverSide {
    fn position_area(self) -> &'static str {
        match self {
            Self::Bottom => "bottom center",
            Self::Top => "top center",
            Self::Left => "left center",
            Self::Right => "right center",
        }
    }
}

#[component]
pub fn Popover(
    #[props(default = PopoverSide::Bottom)] side: PopoverSide,
    children: Element,
) -> Element {
    let id = use_popover_id();
    let anchor_name = format!("--popover-anchor-{id}");
    let trigger_id = format!("popover-trigger-{id}");
    let content_id = format!("popover-content-{id}");
    let position_area = side.position_area();

    provide_context(PopoverContext {
        anchor_name: anchor_name.clone(),
        trigger_id: trigger_id.clone(),
        content_id: content_id.clone(),
    });

    let css = format!(
        "#{content_id} {{ position: fixed; position-anchor: {anchor_name}; position-area: {position_area}; margin: 4px; }}"
    );

    let tid = trigger_id.clone();
    let cid = content_id.clone();
    let script = format!(
        r#"(function() {{
        const setup = () => {{
            const trigger = document.getElementById('{tid}');
            const content = document.getElementById('{cid}');
            if (!trigger || !content) {{ setTimeout(setup, 50); return; }}
            if (trigger.hasAttribute('data-popover-init')) return;
            trigger.setAttribute('data-popover-init', 'true');
            const close = () => content.setAttribute('data-state', 'closed');
            trigger.addEventListener('click', e => {{
                e.stopPropagation();
                const isOpen = content.getAttribute('data-state') === 'open';
                content.setAttribute('data-state', isOpen ? 'closed' : 'open');
            }});
            document.addEventListener('click', e => {{
                if (!content.contains(e.target) && e.target !== trigger) close();
            }});
            document.addEventListener('keydown', e => {{ if (e.key === 'Escape') close(); }});
        }};
        if (document.readyState === 'loading') {{ document.addEventListener('DOMContentLoaded', setup); }} else {{ setup(); }}
    }})();"#
    );

    rsx! {
        style { dangerous_inner_html: "{css}" }
        {children}
        script { dangerous_inner_html: "{script}" }
    }
}

#[component]
pub fn PopoverTrigger(
    #[props(into, optional)] class: Option<String>,
    children: Element,
) -> Element {
    let ctx = use_context::<PopoverContext>();
    rsx! {
        button {
            "data-name": "PopoverTrigger",
            id: "{ctx.trigger_id}",
            class: tw_merge!(
                "inline-flex items-center justify-center gap-2 whitespace-nowrap rounded-md text-sm font-medium border bg-background shadow-xs hover:bg-accent hover:text-accent-foreground h-9 px-4 py-2 cursor-pointer",
                class.as_deref().unwrap_or("")
            ),
            style: "anchor-name: {ctx.anchor_name};",
            {children}
        }
    }
}

#[component]
pub fn PopoverContent(
    #[props(into, optional)] class: Option<String>,
    children: Element,
) -> Element {
    let ctx = use_context::<PopoverContext>();
    let c = tw_merge!(
        "z-50 min-w-[8rem] overflow-hidden rounded-md border bg-popover p-4 text-popover-foreground shadow-md outline-none pointer-events-none opacity-0 transition-all duration-150 data-[state=open]:opacity-100 data-[state=open]:pointer-events-auto",
        class.as_deref().unwrap_or("")
    );
    rsx! {
        div {
            "data-name": "PopoverContent",
            id: "{ctx.content_id}",
            class: "{c}",
            "data-state": "closed",
            role: "dialog",
            {children}
        }
    }
}
