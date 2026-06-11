use dioxus::prelude::*;
use icons::{Check, Copy, Fullscreen, Monitor, Share2, Smartphone, Tablet};
use registry::hooks::use_copy_clipboard::use_copy_clipboard;
use registry::ui::button::{Button, ButtonVariant};
use registry::ui::dialog::{
    Dialog, DialogBody, DialogClose, DialogContent, DialogDescription, DialogFooter, DialogHeader,
    DialogTitle, DialogTrigger,
};
use registry::ui::input::Input;

use crate::domain::blocks::components::block_viewer_toolbar::{BlockView, ScreenSize};
use crate::domain::workflows::workflow_entry::WorkflowEntry;

fn dispatch_resize_event(instance_id: &str, screen_size: ScreenSize) {
    let screen_type = match screen_size {
        ScreenSize::Desktop => "Desktop",
        ScreenSize::Tablet => "Tablet",
        ScreenSize::Phone => "Phone",
    };
    let js = format!(
        "document.dispatchEvent(new CustomEvent('resizable:resize_by_screen__interop', \
         {{ detail: {{ instanceId: '{}', screenType: '{}' }} }}));",
        instance_id, screen_type
    );
    let _ = document::eval(&js);
}

#[component]
pub fn WorkflowViewerToolbar(
    workflow_entry: WorkflowEntry,
    screen_size: Signal<ScreenSize>,
    block_view: Signal<BlockView>,
    instance_id: String,
) -> Element {
    let workflow_id = workflow_entry.workflow_id_kebab;
    let workflow_id_str = workflow_entry.workflow_id_str;
    let share_url = format!("https://rust-ui.dioxus-ui.com/workflows/#{}", workflow_id_str);
    let share_url_signal = use_signal(move || share_url.clone());

    let (copy_fn, copied) = use_copy_clipboard(Some(2000));

    rsx! {
        div { class: "hidden gap-2 items-center pl-2 w-full md:pr-6 lg:flex",
            // ── Preview / Code tabs ──────────────────────────────────────────
            div { class: "grid grid-cols-2 justify-center items-center p-1 h-8 rounded-md bg-muted text-muted-foreground w-fit",
                button {
                    class: "inline-flex flex-1 gap-1.5 justify-center items-center py-1 px-2 text-xs font-medium rounded-sm border border-transparent h-[calc(100%-1px)] transition-[color,box-shadow]",
                    class: if block_view() == BlockView::Preview { "bg-background text-foreground shadow-sm" },
                    onclick: move |_| block_view.set(BlockView::Preview),
                    "Preview"
                }
                button {
                    class: "inline-flex flex-1 gap-1.5 justify-center items-center py-1 px-2 text-xs font-medium rounded-sm border border-transparent h-[calc(100%-1px)] transition-[color,box-shadow]",
                    class: if block_view() == BlockView::Code { "bg-background text-foreground shadow-sm" },
                    onclick: move |_| block_view.set(BlockView::Code),
                    "Code"
                }
            }

            div { class: "mx-2 bg-border shrink-0 data-[orientation=horizontal]:h-px data-[orientation=vertical]:h-full data-[orientation=vertical]:w-px !h-4 w-px" }

            // ── Title ────────────────────────────────────────────────────────
            div { class: "w-full md:w-fit",
                a {
                    href: "#{workflow_id_str}",
                    class: "flex-1 text-sm font-medium text-center md:flex-auto md:text-left hover:underline underline-offset-2",
                    {workflow_id.to_title()}
                }
            }

            // ── Right side controls ──────────────────────────────────────────
            div { class: "flex gap-2 items-center ml-auto",
                div { class: "flex gap-1.5 items-center p-1 h-8 rounded-md border shadow-none",
                    // Desktop
                    {
                        let iid = instance_id.clone();
                        rsx! {
                            button {
                                class: "flex-none px-0 w-6 h-6 inline-flex items-center justify-center rounded-sm transition-colors hover:bg-accent",
                                class: if screen_size() == ScreenSize::Desktop { "bg-accent text-accent-foreground" },
                                title: "Desktop size",
                                onclick: move |_| {
                                    screen_size.set(ScreenSize::Desktop);
                                    dispatch_resize_event(&iid, ScreenSize::Desktop);
                                },
                                Monitor {}
                            }
                        }
                    }
                    // Tablet
                    {
                        let iid = instance_id.clone();
                        rsx! {
                            button {
                                class: "flex-none px-0 w-6 h-6 inline-flex items-center justify-center rounded-sm transition-colors hover:bg-accent",
                                class: if screen_size() == ScreenSize::Tablet { "bg-accent text-accent-foreground" },
                                title: "Tablet size",
                                onclick: move |_| {
                                    screen_size.set(ScreenSize::Tablet);
                                    dispatch_resize_event(&iid, ScreenSize::Tablet);
                                },
                                Tablet {}
                            }
                        }
                    }
                    // Phone
                    {
                        let iid = instance_id.clone();
                        rsx! {
                            button {
                                class: "flex-none px-0 w-6 h-6 inline-flex items-center justify-center rounded-sm transition-colors hover:bg-accent",
                                class: if screen_size() == ScreenSize::Phone { "bg-accent text-accent-foreground" },
                                title: "Phone size",
                                onclick: move |_| {
                                    screen_size.set(ScreenSize::Phone);
                                    dispatch_resize_event(&iid, ScreenSize::Phone);
                                },
                                Smartphone {}
                            }
                        }
                    }

                    div { class: "w-px h-4 bg-border mx-0.5" }

                    // Fullscreen
                    a {
                        href: workflow_id.to_full_view_url(),
                        target: "_blank",
                        class: "flex-none px-0 w-6 h-6 inline-flex items-center justify-center rounded-sm transition-colors hover:bg-accent",
                        title: "Open in New Tab",
                        Fullscreen {}
                    }

                    div { class: "w-px h-4 bg-border mx-0.5" }

                    // Share dialog
                    Dialog {
                        DialogTrigger {
                            class: "p-0 border-none bg-transparent shadow-none flex-none w-6 h-6 inline-flex items-center justify-center rounded-sm hover:bg-accent",
                            Share2 {}
                        }
                        DialogContent {
                            DialogBody {
                                DialogHeader {
                                    div { class: "flex gap-2 items-center",
                                        DialogTitle { "Share Workflow" }
                                        Share2 { class: "size-5" }
                                    }
                                    DialogDescription {
                                        "Copy the URL below to share this workflow with others."
                                    }
                                }
                                div { class: "flex gap-2",
                                    Input {
                                        value: share_url_signal(),
                                        readonly: true,
                                        class: "flex-1",
                                    }
                                    Button {
                                        variant: ButtonVariant::Outline,
                                        onclick: {
                                            let copy_fn2 = copy_fn.clone();
                                            let url = share_url_signal();
                                            move |_| copy_fn2(&url)
                                        },
                                        if copied() {
                                            Check {}
                                        } else {
                                            Copy {}
                                        }
                                    }
                                }
                                DialogFooter {
                                    DialogClose { "Close" }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}
