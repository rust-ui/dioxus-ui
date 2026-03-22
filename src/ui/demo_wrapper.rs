use std::sync::atomic::{AtomicUsize, Ordering};

use dioxus::document::eval;
use dioxus::prelude::*;

static DEMO_COUNTER: AtomicUsize = AtomicUsize::new(0);

#[derive(Clone, Copy, PartialEq)]
enum DemoTab {
    Preview,
    Code,
}

#[component]
pub fn DemoWrapper(children: Element) -> Element {
    let id = use_hook(|| DEMO_COUNTER.fetch_add(1, Ordering::Relaxed));
    let container_id = format!("demo-preview-{id}");
    let handle_id = format!("demo-handle-{id}");

    let mut tab = use_signal(|| DemoTab::Preview);

    // Inject JS drag-to-resize on mount
    let cid = container_id.clone();
    let hid = handle_id.clone();
    use_effect(move || {
        let js = format!(
            r#"
            (function() {{
                const handle = document.getElementById('{hid}');
                const container = document.getElementById('{cid}');
                if (!handle || !container) return;

                let startX, startW;

                handle.addEventListener('mousedown', function(e) {{
                    startX = e.clientX;
                    startW = container.offsetWidth;
                    document.addEventListener('mousemove', onMove);
                    document.addEventListener('mouseup', onUp);
                    e.preventDefault();
                }});

                function onMove(e) {{
                    const dx = e.clientX - startX;
                    const newW = Math.max(200, startW + dx);
                    container.style.width = newW + 'px';
                    container.style.flex = 'none';
                }}

                function onUp() {{
                    document.removeEventListener('mousemove', onMove);
                    document.removeEventListener('mouseup', onUp);
                }}
            }})();
            "#
        );
        spawn(async move {
            let _ = eval(&js).await;
        });
    });

    let tab_base = "inline-flex items-center gap-1.5 rounded-sm px-3 py-1.5 text-sm font-medium transition-colors";
    let active = "bg-background text-foreground shadow-sm";
    let inactive = "text-muted-foreground hover:text-foreground";

    let preview_cls = move || format!("{tab_base} {}", if tab() == DemoTab::Preview { active } else { inactive });
    let code_cls = move || format!("{tab_base} {}", if tab() == DemoTab::Code { active } else { inactive });
    let preview_display = move || if tab() == DemoTab::Preview { "display:block" } else { "display:none" };
    let code_display = move || if tab() == DemoTab::Code { "display:block" } else { "display:none" };

    rsx! {
        div { class: "flex flex-col gap-2 w-full",

            // Toolbar
            div { class: "inline-flex h-9 items-center rounded-md bg-muted p-1 text-muted-foreground",
                button {
                    class: "{preview_cls()}",
                    onclick: move |_| tab.set(DemoTab::Preview),
                    svg {
                        xmlns: "http://www.w3.org/2000/svg",
                        width: "14", height: "14",
                        view_box: "0 0 24 24",
                        fill: "none", stroke: "currentColor",
                        stroke_width: "2",
                        stroke_linecap: "round", stroke_linejoin: "round",
                        path { d: "M2 12s3-7 10-7 10 7 10 7-3 7-10 7-10-7-10-7Z" }
                        circle { cx: "12", cy: "12", r: "3" }
                    }
                    "Preview"
                }
                button {
                    class: "{code_cls()}",
                    onclick: move |_| tab.set(DemoTab::Code),
                    svg {
                        xmlns: "http://www.w3.org/2000/svg",
                        width: "14", height: "14",
                        view_box: "0 0 24 24",
                        fill: "none", stroke: "currentColor",
                        stroke_width: "2",
                        stroke_linecap: "round", stroke_linejoin: "round",
                        polyline { points: "16 18 22 12 16 6" }
                        polyline { points: "8 6 2 12 8 18" }
                    }
                    "Code"
                }
            }

            // Preview panel
            div { style: "{preview_display()}",
                div { class: "border rounded-xl flex flex-row w-full overflow-hidden",
                    // Resizable content area
                    div {
                        id: "{container_id}",
                        class: "flex items-center justify-center min-h-[370px] flex-1 px-4 bg-background",
                        {children}
                    }
                    // Drag handle
                    div {
                        id: "{handle_id}",
                        class: "hidden md:flex items-center justify-center w-3 shrink-0 cursor-col-resize select-none z-10",
                        div { class: "h-8 w-1.5 rounded-full bg-border" }
                    }
                    // Dotted background
                    div {
                        class: "flex-1 bg-muted min-w-0",
                        style: "background-image: radial-gradient(circle, color-mix(in srgb, currentColor 25%, transparent) 1px, transparent 1px); background-size: 20px 20px; background-attachment: fixed;",
                    }
                }
            }

            // Code panel — Soon
            div { style: "{code_display()}",
                div { class: "rounded-xl border bg-muted flex items-center justify-center min-h-[200px]",
                    p { class: "text-sm text-muted-foreground", "Code display — coming soon" }
                }
            }
        }
    }
}
