use dioxus::document::eval;
use dioxus::prelude::*;
use icons::{Anchor, ArrowDown, ArrowRight, ArrowUp, CircleDashed, CornerDownLeft, Search};
use registry::ui::input_group::{InputGroup, InputGroupAddon};
use registry::ui::kbd::Kbd;

use crate::__registry__::command_bar::{COMPONENTS_ITEMS, CommandCategory, CommandItemData, HOOKS_ITEMS, PAGES_ITEMS};

#[derive(Clone, Copy)]
pub struct CommandBarState {
    pub open: Signal<bool>,
}

pub fn use_command_bar_provider() -> CommandBarState {
    let open = use_context_provider(|| Signal::new(false));
    CommandBarState { open }
}

fn use_command_bar() -> CommandBarState {
    CommandBarState { open: use_context::<Signal<bool>>() }
}

#[component]
pub fn CommandBarTrigger() -> Element {
    let mut state = use_command_bar();

    rsx! {
        button {
            class: "inline-flex items-center gap-2 whitespace-nowrap rounded-md border border-input bg-background text-muted-foreground hover:bg-accent hover:text-accent-foreground flex-1 justify-start pl-3 h-8 text-sm font-normal shadow-none md:flex-none",
            onclick: move |_| state.open.set(true),
            Search { class: "size-4 shrink-0" }
            span { class: "hidden md:inline-flex", "Search..." }
            span { class: "inline-flex md:hidden", "Search documentation..." }
            kbd {
                class: "flex gap-1 items-center px-1.5 h-5 font-mono font-medium rounded border opacity-100 pointer-events-none select-none bg-muted text-[10px] ml-auto",
                span { class: "text-xs", "⌘" }
                span { "K" }
            }
        }
    }
}

#[component]
pub fn CommandBarDialog() -> Element {
    let mut state = use_command_bar();

    use_effect(move || {
        let mut open = state.open;
        spawn(async move {
            let _ = eval(
                r#"
                if (!window.__cmdBarGlobalInit) {
                    window.__cmdBarGlobalInit = true;
                    document.addEventListener('keydown', function(e) {
                        if ((e.metaKey || e.ctrlKey) && e.key.toLowerCase() === 'k') {
                            e.preventDefault();
                            document.dispatchEvent(new CustomEvent('cmd-open'));
                        }
                    });
                }
            "#,
            )
            .await;

            loop {
                let _ = eval(
                    r#"
                    await new Promise(resolve => {
                        document.addEventListener('cmd-open', resolve, { once: true });
                    });
                "#,
                )
                .await;
                open.set(true);
            }
        });
    });

    let is_open = (state.open)();
    use_effect(move || {
        if !is_open {
            return;
        }

        spawn(async move {
            let _ = eval(
                r#"
                (function() {
                    const setup = () => {
                        const dialog = document.querySelector('#command-search-docs');
                        const copyFooter = document.getElementById('cmd-copy-footer');
                        const copyLabel = document.getElementById('cmd-copy-label');

                        if (!dialog || !copyFooter || !copyLabel) {
                            setTimeout(setup, 50);
                            return;
                        }

                        if (dialog.dataset.ready === 'true') {
                            const input = dialog.querySelector('[data-name="CommandInput"]');
                            if (input) {
                                input.value = '';
                                setTimeout(() => input.focus(), 10);
                            }
                            dialog.setAttribute('data-state', 'open');
                            return;
                        }
                        dialog.dataset.ready = 'true';

                        const tabButtons = dialog.querySelectorAll('[data-name="CommandTabBar"] [data-tab]');
                        const groups = dialog.querySelectorAll('[data-name="CommandGroup"][data-category]');
                        const input = dialog.querySelector('[data-name="CommandInput"]');

                        let activeTab = 'all';

                        const visibleItems = () => Array
                            .from(dialog.querySelectorAll('[data-name="CommandItemLink"]'))
                            .filter(el => el.offsetParent !== null && el.style.display !== 'none');

                        const setSelected = (items, idx) => {
                            items.forEach((el, i) => {
                                el.setAttribute('aria-selected', i === idx ? 'true' : 'false');
                            });

                            if (idx >= 0 && idx < items.length) {
                                items[idx].scrollIntoView({ block: 'nearest' });
                            }
                        };

                        const updateCopyHint = () => {
                            const selected = dialog.querySelector('[data-name="CommandItemLink"][aria-selected="true"]');
                            const slug = selected?.getAttribute('data-add-cmd');
                            if (slug) {
                                copyLabel.textContent = 'ui add ' + slug;
                                copyFooter.style.display = '';
                            } else {
                                copyFooter.style.display = 'none';
                            }
                        };

                        const filterItems = () => {
                            const term = (input?.value || '').trim().toLowerCase();

                            groups.forEach(group => {
                                const inTab = activeTab === 'all' || group.dataset.category === activeTab;
                                let visibleCount = 0;

                                group.querySelectorAll('[data-name="CommandItemLink"]').forEach(item => {
                                    const label = (item.getAttribute('data-search') || item.textContent || '').toLowerCase();
                                    const matches = term === '' || label.includes(term);
                                    item.style.display = inTab && matches ? '' : 'none';
                                    if (inTab && matches) {
                                        visibleCount += 1;
                                    }
                                });

                                group.style.display = inTab && visibleCount > 0 ? '' : 'none';
                            });

                            const items = visibleItems();
                            setSelected(items, items.length > 0 ? 0 : -1);
                            updateCopyHint();
                        };

                        const applyTab = (tab) => {
                            activeTab = tab;
                            tabButtons.forEach(btn => {
                                btn.setAttribute('data-active', btn.dataset.tab === tab ? 'true' : 'false');
                            });

                            if (input && input.value !== '') {
                                input.value = '';
                            }

                            filterItems();
                        };

                        tabButtons.forEach(btn => {
                            btn.addEventListener('click', (e) => {
                                e.preventDefault();
                                e.stopPropagation();
                                applyTab(btn.dataset.tab);
                            });
                        });

                        const ariaObserver = new MutationObserver((mutations) => {
                            for (const m of mutations) {
                                if (m.attributeName === 'aria-selected') {
                                    updateCopyHint();
                                    break;
                                }
                            }
                        });

                        dialog.querySelectorAll('[data-name="CommandItemLink"]').forEach(item => {
                            ariaObserver.observe(item, { attributes: true, attributeFilter: ['aria-selected'] });

                            item.addEventListener('mouseenter', () => {
                                const slug = item.getAttribute('data-add-cmd');
                                if (slug) {
                                    copyLabel.textContent = 'ui add ' + slug;
                                    copyFooter.style.display = '';
                                } else {
                                    copyFooter.style.display = 'none';
                                }
                            });

                            item.addEventListener('mouseleave', updateCopyHint);
                            item.addEventListener('click', () => {
                                setTimeout(() => {
                                    dialog.dispatchEvent(new CustomEvent('cmd-close'));
                                }, 0);
                            });
                        });

                        if (input) {
                            input.addEventListener('input', filterItems);
                        }

                        dialog.addEventListener('keydown', (e) => {
                            const items = visibleItems();
                            const current = items.findIndex(el => el.getAttribute('aria-selected') === 'true');

                            if (e.key === 'ArrowDown') {
                                e.preventDefault();
                                setSelected(items, Math.min(current + 1, items.length - 1));
                            } else if (e.key === 'ArrowUp') {
                                e.preventDefault();
                                setSelected(items, Math.max(current - 1, 0));
                            } else if (e.key === 'Enter') {
                                const selected = items[current];
                                if (selected) {
                                    e.preventDefault();
                                    selected.click();
                                }
                            } else if (e.key === 'Escape') {
                                e.preventDefault();
                                dialog.dispatchEvent(new CustomEvent('cmd-close'));
                            }
                        });

                        document.addEventListener('keydown', (e) => {
                            if (dialog.getAttribute('data-state') !== 'open') return;
                            if ((e.metaKey || e.ctrlKey) && e.key.toLowerCase() === 'c') {
                                const selected = dialog.querySelector('[data-name="CommandItemLink"][aria-selected="true"]');
                                const slug = selected?.getAttribute('data-add-cmd');
                                if (slug) {
                                    e.preventDefault();
                                    const cmd = 'ui add ' + slug;
                                    navigator.clipboard.writeText(cmd).catch(() => {});
                                    const orig = copyLabel.textContent;
                                    copyLabel.textContent = 'Copied!';
                                    setTimeout(() => { copyLabel.textContent = orig; }, 1500);
                                }
                            }
                        });

                        if (input) {
                            input.value = '';
                            setTimeout(() => input.focus(), 10);
                        }

                        applyTab('all');
                    };

                    setup();
                })();
            "#,
            )
            .await;
        });
    });

    use_effect(move || {
        if !is_open {
            return;
        }

        let mut open = state.open;
        spawn(async move {
            let _ = eval(
                r#"
                await new Promise(resolve => {
                    const dialog = document.getElementById('command-search-docs');
                    if (!dialog) {
                        resolve('missing');
                        return;
                    }
                    dialog.addEventListener('cmd-close', resolve, { once: true });
                });
            "#,
            )
            .await;
            open.set(false);
        });
    });

    if !(state.open)() {
        return rsx! {
            div { id: "command-search-docs", "data-state": "closed" }
        };
    }

    rsx! {
        div { id: "command-search-docs", "data-state": "open",
            div {
                class: "fixed inset-0 z-50 flex items-start justify-center",
                style: "padding-top: 10vh;",
                div {
                    class: "absolute inset-0 bg-black/50",
                    onclick: move |_| state.open.set(false),
                }
                div {
                    class: "relative z-10 w-full max-w-lg rounded-xl border border-border bg-background shadow-lg overflow-hidden",
                    role: "dialog",
                    aria_modal: "true",
                    tabindex: "0",
                    div {
                        class: "sr-only",
                        h2 { "Search documentation..." }
                        p { "Search for a command to run..." }
                    }

                    InputGroup { class: "h-9 bg-input/50 rounded-none border-x-0 border-t-0 shadow-none",
                        InputGroupAddon {
                            Search {}
                        }
                        input {
                            "data-name": "CommandInput",
                            class: "flex-1 py-0 h-9 rounded-none border-0 shadow-none bg-transparent px-3 text-sm outline-none placeholder:text-muted-foreground",
                            placeholder: "Search documentation...",
                        }
                    }

                    div { class: "flex gap-1 px-3 pt-1 pb-2 border-b border-border", "data-name": "CommandTabBar",
                        button {
                            "data-tab": "all",
                            "data-active": "true",
                            class: "py-1 px-2.5 text-xs font-medium rounded-md transition-colors data-[active=true]:bg-muted data-[active=true]:text-foreground data-[active=false]:text-muted-foreground data-[active=false]:hover:text-foreground",
                            "All"
                        }
                        button {
                            "data-tab": "pages",
                            "data-active": "false",
                            class: "py-1 px-2.5 text-xs font-medium rounded-md transition-colors data-[active=true]:bg-muted data-[active=true]:text-foreground data-[active=false]:text-muted-foreground data-[active=false]:hover:text-foreground",
                            "Pages"
                        }
                        button {
                            "data-tab": "components",
                            "data-active": "false",
                            class: "py-1 px-2.5 text-xs font-medium rounded-md transition-colors data-[active=true]:bg-muted data-[active=true]:text-foreground data-[active=false]:text-muted-foreground data-[active=false]:hover:text-foreground",
                            "Components"
                        }
                        button {
                            "data-tab": "hooks",
                            "data-active": "false",
                            class: "py-1 px-2.5 text-xs font-medium rounded-md transition-colors data-[active=true]:bg-muted data-[active=true]:text-foreground data-[active=false]:text-muted-foreground data-[active=false]:hover:text-foreground",
                            "Hooks"
                        }
                    }

                    div { id: "command_demo", tabindex: "-1", class: "max-h-[60vh] overflow-y-auto",
                        CommandGroupView { category: CommandCategory::Pages, items: PAGES_ITEMS }
                        CommandGroupView { category: CommandCategory::Components, items: COMPONENTS_ITEMS }
                        CommandGroupView { category: CommandCategory::Hooks, items: HOOKS_ITEMS }
                    }

                    div { class: "flex items-center gap-4 px-3 py-2 border-t border-border text-xs text-muted-foreground",
                        div { class: "flex gap-2 items-center",
                            Kbd { ArrowUp {} }
                            Kbd { ArrowDown {} }
                            span { "Navigate" }
                        }
                        div { class: "flex gap-2 items-center",
                            Kbd { CornerDownLeft {} }
                            span { "Go to Page" }
                        }
                        div { id: "cmd-copy-footer", class: "flex gap-2 items-center ml-auto", style: "display: none;",
                            Kbd { "⌘" }
                            Kbd { "C" }
                            span { id: "cmd-copy-label" }
                        }
                    }
                }
            }
        }
    }
}

#[component]
fn CommandGroupView(category: CommandCategory, items: &'static [CommandItemData]) -> Element {
    let category_label = category.as_str();
    let category_slug = category.slug();

    rsx! {
        div {
            "data-name": "CommandGroup",
            "data-category": "{category_slug}",
            role: "presentation",
            class: "p-0",
            div { aria_hidden: "true", class: "p-3 text-xs font-medium text-muted-foreground", "{category_label}" }
            for item in items.iter().copied() {
                CommandItemLink { item }
            }
        }
    }
}

#[component]
fn CommandItemLink(item: CommandItemData) -> Element {
    let add_cmd = item.add_cmd.unwrap_or("");
    let href = item.href;
    let label = item.label;

    rsx! {
        a {
            "data-name": "CommandItemLink",
            "data-add-cmd": "{add_cmd}",
            "data-search": "{label}",
            href: "{href}",
            aria_selected: "false",
            class: "flex w-full items-center gap-2 px-3 py-2 text-sm hover:bg-accent hover:text-accent-foreground transition-colors cursor-pointer aria-selected:bg-accent aria-selected:text-accent-foreground",
            CategoryIcon { category: item.category }
            span { "{label}" }
        }
    }
}

#[component]
fn CategoryIcon(category: CommandCategory) -> Element {
    rsx! {
        match category {
            CommandCategory::Pages => rsx! { ArrowRight { class: "size-4 text-muted-foreground shrink-0" } },
            CommandCategory::Components => rsx! { CircleDashed { class: "size-4 text-muted-foreground shrink-0" } },
            CommandCategory::Hooks => rsx! { Anchor { class: "size-4 text-muted-foreground shrink-0" } },
        }
    }
}
