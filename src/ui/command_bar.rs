use dioxus::document::eval;
use dioxus::prelude::*;
use icons::Search;

use crate::__registry__::command_bar::COMPONENTS_ITEMS;
use crate::Route;

// ── Shared state ─────────────────────────────────────────────────────────────

#[derive(Clone, Copy)]
pub struct CommandBarState {
    pub open: Signal<bool>,
    pub query: Signal<String>,
}

/// Call once at the AppLayout level to provide command bar state to children.
pub fn use_command_bar_provider() -> CommandBarState {
    let open = use_context_provider(|| Signal::new(false));
    let query = use_context_provider(|| Signal::new(String::new()));
    CommandBarState { open, query }
}

fn use_command_bar() -> CommandBarState {
    CommandBarState {
        open: use_context::<Signal<bool>>(),
        query: use_context::<Signal<String>>(),
    }
}

// ── Trigger (lives inside Navbar) ─────────────────────────────────────────────

#[component]
pub fn CommandBarTrigger() -> Element {
    let mut state = use_command_bar();

    rsx! {
        button {
            class: "inline-flex items-center gap-2 whitespace-nowrap rounded-md border border-input bg-background text-muted-foreground hover:bg-accent hover:text-accent-foreground flex-1 justify-start pl-3 h-8 text-sm font-normal shadow-none md:flex-none md:w-[220px] w-full",
            onclick: move |_| {
                state.open.set(true);
                state.query.set(String::new());
            },
            Search { class: "size-4 shrink-0" }
            span { class: "hidden md:inline-flex", "Search docs..." }
            span { class: "inline-flex md:hidden", "Search documentation..." }
            kbd {
                class: "flex gap-1 items-center px-1.5 h-5 font-mono font-medium rounded border opacity-100 pointer-events-none select-none bg-muted text-[10px] ml-auto",
                span { class: "text-xs", "⌘" }
                span { "K" }
            }
        }
    }
}

// ── Dialog (lives in AppLayout, outside sticky/backdrop-filter header) ────────

#[component]
pub fn CommandBarDialog() -> Element {
    let mut state = use_command_bar();
    let nav = navigator();

    // Global Cmd+K listener
    use_effect(move || {
        spawn(async move {
            let _ = eval(
                r#"
                if (!window.__cmdBarGlobalInit) {
                    window.__cmdBarGlobalInit = true;
                    document.addEventListener('keydown', function(e) {
                        if ((e.metaKey || e.ctrlKey) && e.key === 'k') {
                            e.preventDefault();
                            const d = document.getElementById('command-search-docs');
                            if (d) d.dispatchEvent(new CustomEvent('cmd-open'));
                        }
                    });
                }
            "#,
            )
            .await;
        });
    });

    // Listen for cmd-open event → open dialog
    let mut open_w = state.open;
    use_effect(move || {
        spawn(async move {
            let mut ev = eval(
                r#"
                await new Promise(resolve => {
                    const d = document.getElementById('command-search-docs');
                    const handler = () => resolve('open');
                    if (d) d.addEventListener('cmd-open', handler, { once: true });
                    else document.addEventListener('cmd-open', handler, { once: true });
                });
            "#,
            );
            let _ = ev.await;
            open_w.set(true);
        });
    });

    // Run RUST-UI tab/copy/keyboard JS each time dialog opens
    let is_open = (state.open)();
    use_effect(move || {
        if !is_open {
            return;
        }
        spawn(async move {
            let _ = eval(r#"
            (function() {
                const setup = () => {
                    const dialog = document.querySelector('#command-search-docs');
                    const copyFooter = document.getElementById('cmd-copy-footer');
                    const copyLabel = document.getElementById('cmd-copy-label');
                    if (!dialog || !copyFooter || !copyLabel) { setTimeout(setup, 50); return; }

                    // ── Tab switching ───────────────────────────────────
                    const tabButtons = dialog.querySelectorAll('[data-name="CommandTabBar"] [data-tab]');
                    const groups = dialog.querySelectorAll('[data-name="CommandGroup"][data-category]');

                    const applyTab = (tab) => {
                        tabButtons.forEach(btn => btn.setAttribute('data-active', btn.dataset.tab === tab ? 'true' : 'false'));
                        groups.forEach(g => { g.style.display = (tab === 'all' || g.dataset.category === tab) ? '' : 'none'; });
                        const input = dialog.querySelector('[data-name="CommandInput"]');
                        if (input && input.value !== '') { input.value = ''; input.dispatchEvent(new Event('input')); }
                    };

                    tabButtons.forEach(btn => btn.addEventListener('click', (e) => { e.preventDefault(); applyTab(btn.dataset.tab); }));

                    // ── Copy hint ───────────────────────────────────────
                    const updateCopyHint = () => {
                        const sel = dialog.querySelector('[data-name="CommandItemLink"][aria-selected="true"]');
                        const slug = sel?.getAttribute('data-add-cmd');
                        if (slug) { copyLabel.textContent = 'ui add ' + slug; copyFooter.style.display = ''; }
                        else { copyFooter.style.display = 'none'; }
                    };

                    const ariaObs = new MutationObserver(mutations => {
                        for (const m of mutations) { if (m.attributeName === 'aria-selected') { updateCopyHint(); break; } }
                    });

                    const attachItemListeners = () => {
                        dialog.querySelectorAll('[data-name="CommandItemLink"]').forEach(item => {
                            ariaObs.observe(item, { attributes: true, attributeFilter: ['aria-selected'] });
                            item.addEventListener('mouseenter', () => {
                                const slug = item.getAttribute('data-add-cmd');
                                if (slug) { copyLabel.textContent = 'ui add ' + slug; copyFooter.style.display = ''; }
                                else { copyFooter.style.display = 'none'; }
                            });
                            item.addEventListener('mouseleave', updateCopyHint);
                        });
                    };
                    attachItemListeners();

                    // ── Keyboard navigation ──────────────────────────────
                    const getVisible = () => Array.from(dialog.querySelectorAll('[data-name="CommandItemLink"]')).filter(el => el.offsetParent !== null);
                    const setSelected = (items, idx) => {
                        items.forEach((el, i) => el.setAttribute('aria-selected', i === idx ? 'true' : 'false'));
                        if (idx >= 0 && idx < items.length) items[idx].scrollIntoView({ block: 'nearest' });
                    };

                    dialog.addEventListener('keydown', (e) => {
                        const items = getVisible();
                        const cur = items.findIndex(el => el.getAttribute('aria-selected') === 'true');
                        if (e.key === 'ArrowDown') { e.preventDefault(); setSelected(items, Math.min(cur + 1, items.length - 1)); }
                        else if (e.key === 'ArrowUp') { e.preventDefault(); setSelected(items, Math.max(cur - 1, 0)); }
                        else if (e.key === 'Enter') { const s = items[cur]; if (s) { e.preventDefault(); s.click(); } }
                        else if (e.key === 'Escape') { dialog.dispatchEvent(new CustomEvent('cmd-close')); }
                    });

                    // ── ⌘C copy ─────────────────────────────────────────
                    document.addEventListener('keydown', (e) => {
                        if (dialog.getAttribute('data-state') !== 'open') return;
                        if ((e.metaKey || e.ctrlKey) && e.key === 'c') {
                            const sel = dialog.querySelector('[data-name="CommandItemLink"][aria-selected="true"]');
                            const slug = sel?.getAttribute('data-add-cmd');
                            if (slug) {
                                e.preventDefault();
                                navigator.clipboard.writeText('ui add ' + slug).catch(() => {});
                                const orig = copyLabel.textContent;
                                copyLabel.textContent = 'Copied!';
                                setTimeout(() => { copyLabel.textContent = orig; }, 1500);
                            }
                        }
                    });

                    // Focus input
                    const input = dialog.querySelector('[data-name="CommandInput"]');
                    if (input) setTimeout(() => input.focus(), 10);

                    applyTab('all');
                };
                setup();
            })();
            "#).await;
        });
    });

    let filtered: Vec<_> = COMPONENTS_ITEMS
        .iter()
        .filter(|item| {
            let q = (state.query)().to_lowercase();
            q.is_empty() || item.label.to_lowercase().contains(&q)
        })
        .collect();

    let kbd_cls = "inline-flex items-center justify-center rounded border border-border px-1.5 h-5 font-mono font-medium text-[10px] bg-muted text-muted-foreground";

    if !(state.open)() {
        return rsx! {
            div { id: "command-search-docs", "data-state": "closed" }
        };
    }

    rsx! {
        div {
            id: "command-search-docs",
            "data-state": "open",
            // Full-viewport overlay — rendered at AppLayout level, outside sticky header
            div {
                class: "fixed inset-0 z-50 flex items-start justify-center",
                style: "padding-top: 10vh;",
                // Backdrop
                div {
                    class: "absolute inset-0 bg-black/50",
                    onclick: move |_| state.open.set(false),
                }
                // Panel
                div {
                    class: "relative z-10 w-full max-w-lg rounded-xl border border-border bg-background shadow-lg overflow-hidden",
                    role: "dialog",
                    aria_modal: "true",
                    div { class: "sr-only", h2 { "Search documentation..." } p { "Search for a command to run..." } }

                    // Search input
                    div { class: "flex items-center gap-2 border-b border-border bg-input/50 px-3", style: "height: 2.25rem;",
                        Search { class: "size-4 shrink-0 text-muted-foreground" }
                        input {
                            "data-name": "CommandInput",
                            class: "flex-1 bg-transparent text-sm outline-none placeholder:text-muted-foreground border-0 shadow-none",
                            style: "height: 2.25rem;",
                            placeholder: "Search documentation...",
                            value: "{state.query}",
                            oninput: move |e| state.query.set(e.value()),
                        }
                    }

                    // Tab bar
                    div { "data-name": "CommandTabBar", class: "flex gap-1 px-3 pt-1 pb-2 border-b border-border",
                        button {
                            "data-tab": "all", "data-active": "true",
                            class: "py-1 px-2.5 text-xs font-medium rounded-md transition-colors data-[active=true]:bg-muted data-[active=true]:text-foreground data-[active=false]:text-muted-foreground data-[active=false]:hover:text-foreground",
                            "All"
                        }
                        button {
                            "data-tab": "components", "data-active": "false",
                            class: "py-1 px-2.5 text-xs font-medium rounded-md transition-colors data-[active=true]:bg-muted data-[active=true]:text-foreground data-[active=false]:text-muted-foreground data-[active=false]:hover:text-foreground",
                            "Components"
                        }
                    }

                    // List
                    div { id: "command_demo", tabindex: "-1", style: "max-height: 60vh; overflow-y: auto;",
                        div {
                            "data-name": "CommandGroup", "data-category": "components",
                            role: "presentation", class: "p-0",
                            div { aria_hidden: "true", class: "p-3 text-xs font-medium text-muted-foreground", "Components" }
                            if filtered.is_empty() {
                                p { class: "px-4 py-6 text-center text-sm text-muted-foreground", "No results found." }
                            }
                            for item in &filtered {
                                {
                                    let slug = item.href.trim_start_matches("/components/").to_string();
                                    let label = item.label;
                                    rsx! {
                                        button {
                                            "data-name": "CommandItemLink",
                                            "data-add-cmd": slug.clone(),
                                            aria_selected: "false",
                                            class: "flex w-full items-center gap-2 px-3 py-2 text-sm hover:bg-accent hover:text-accent-foreground transition-colors cursor-pointer aria-selected:bg-accent aria-selected:text-accent-foreground",
                                            onclick: {
                                                let slug = slug.clone();
                                                let nav = nav.clone();
                                                move |_| {
                                                    state.open.set(false);
                                                    nav.push(Route::ComponentPage { name: slug.clone() });
                                                }
                                            },
                                            Search { class: "size-3.5 text-muted-foreground shrink-0" }
                                            span { "{label}" }
                                        }
                                    }
                                }
                            }
                        }
                    }

                    // Footer
                    div { class: "flex items-center gap-4 px-3 py-2 border-t border-border text-xs text-muted-foreground",
                        div { class: "flex gap-2 items-center",
                            kbd { class: "{kbd_cls}", "↑" }
                            kbd { class: "{kbd_cls}", "↓" }
                            span { "Navigate" }
                        }
                        div { class: "flex gap-2 items-center",
                            kbd { class: "{kbd_cls}", "↵" }
                            span { "Go to Page" }
                        }
                        div { id: "cmd-copy-footer", class: "flex gap-2 items-center ml-auto", style: "display: none;",
                            kbd { class: "{kbd_cls}", "⌘" }
                            kbd { class: "{kbd_cls}", "C" }
                            span { id: "cmd-copy-label" }
                        }
                    }
                }
            }
        }
    }
}
