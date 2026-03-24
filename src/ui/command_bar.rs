use dioxus::document::eval;
use dioxus::prelude::*;
use icons::Search;

use crate::__registry__::command_bar::COMPONENTS_ITEMS;
use crate::Route;

#[component]
pub fn CommandBar() -> Element {
    let mut open = use_signal(|| false);
    let mut query = use_signal(String::new);
    let nav = navigator();

    // Cmd+K / Ctrl+K to open; Escape to close
    use_effect(move || {
        spawn(async move {
            let _ = eval(
                r#"
                (function() {
                    if (window.__commandBarListener) return;
                    window.__commandBarListener = true;
                    document.addEventListener('keydown', function(e) {
                        if ((e.metaKey || e.ctrlKey) && e.key === 'k') {
                            e.preventDefault();
                            document.dispatchEvent(new CustomEvent('command-bar-open'));
                        }
                        if (e.key === 'Escape') {
                            document.dispatchEvent(new CustomEvent('command-bar-close'));
                        }
                    });
                })();
                "#,
            )
            .await;
        });
    });

    // Listen for open/close custom events via polling signal
    let mut open_clone = open;
    use_effect(move || {
        spawn(async move {
            let mut ev = eval(
                r#"
                await new Promise(resolve => {
                    document.addEventListener('command-bar-open', resolve, { once: true });
                    document.addEventListener('command-bar-close', resolve, { once: true });
                });
                "#,
            );
            let _ = ev.await;
            open_clone.toggle();
        });
    });

    let filtered: Vec<_> = COMPONENTS_ITEMS
        .iter()
        .filter(|item| {
            let q = query().to_lowercase();
            q.is_empty() || item.label.to_lowercase().contains(&q)
        })
        .collect();

    rsx! {
        // Trigger button
        button {
            class: "inline-flex items-center gap-2 rounded-md border border-input bg-background px-3 py-1.5 text-sm text-muted-foreground shadow-xs hover:bg-accent hover:text-accent-foreground transition-colors",
            onclick: move |_| { open.set(true); query.set(String::new()); },
            Search { class: "size-3.5" }
            span { "Search" }
            span { class: "hidden sm:flex items-center gap-0.5 text-xs text-muted-foreground",
                kbd { class: "rounded border border-border px-1 font-mono text-[10px]", "⌘" }
                kbd { class: "rounded border border-border px-1 font-mono text-[10px]", "K" }
            }
        }

        // Dialog overlay
        if open() {
            div {
                class: "fixed inset-0 z-50 flex items-start justify-center pt-[10vh]",
                // Backdrop
                div {
                    class: "absolute inset-0 bg-black/50",
                    onclick: move |_| { open.set(false); },
                }
                // Panel
                div {
                    class: "relative z-10 w-full max-w-lg rounded-xl border border-border bg-background shadow-lg overflow-hidden",
                    // Search input row
                    div { class: "flex items-center gap-2 border-b border-border px-4 py-3",
                        Search { class: "size-4 shrink-0 text-muted-foreground" }
                        input {
                            class: "flex-1 bg-transparent text-sm outline-none placeholder:text-muted-foreground",
                            placeholder: "Search components...",
                            autofocus: true,
                            value: "{query}",
                            oninput: move |e| query.set(e.value()),
                        }
                        kbd {
                            class: "rounded border border-border px-1.5 py-0.5 font-mono text-[10px] text-muted-foreground",
                            "Esc"
                        }
                    }
                    // Results list
                    div { class: "max-h-[60vh] overflow-y-auto py-2",
                        if filtered.is_empty() {
                            p { class: "px-4 py-6 text-center text-sm text-muted-foreground",
                                "No components found."
                            }
                        } else {
                            p { class: "px-4 pb-1 pt-1 text-xs font-medium text-muted-foreground uppercase tracking-wider",
                                "Components"
                            }
                            for item in filtered {
                                {
                                    let slug = item.href.trim_start_matches("/components/").to_string();
                                    let label = item.label;
                                    rsx! {
                                        button {
                                            class: "flex w-full items-center gap-2 rounded-md px-4 py-2 text-sm hover:bg-accent hover:text-accent-foreground transition-colors",
                                            onclick: move |_| {
                                                open.set(false);
                                                nav.push(Route::ComponentPage { name: slug.clone() });
                                            },
                                            Search { class: "size-3.5 text-muted-foreground" }
                                            "{label}"
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}
