use dioxus::prelude::*;

use crate::ui::button::Button;
use crate::ui::dialog::{
    Dialog, DialogBody, DialogClose, DialogContent, DialogDescription, DialogFooter, DialogHeader,
    DialogTitle, DialogTrigger,
};

#[component]
pub fn DemoDialogScrollable() -> Element {
    let mut open = use_signal(|| false);

    rsx! {
        Dialog { open: open,
            DialogTrigger {
                onclick: move |_| open.set(true),
                "Open Scrollable Dialog"
            }
            DialogContent { open: open,
                DialogBody {
                    DialogHeader {
                        DialogTitle { "Terms of Service" }
                        DialogDescription { "Please read the terms before continuing." }
                    }
                    div { class: "flex flex-col gap-3 text-sm text-muted-foreground",
                        for i in 1..=8 {
                            p {
                                "Section {i}: Lorem ipsum dolor sit amet, consectetur adipiscing elit. \
                                Sed do eiusmod tempor incididunt ut labore et dolore magna aliqua. \
                                Ut enim ad minim veniam, quis nostrud exercitation ullamco laboris."
                            }
                        }
                    }
                    DialogFooter {
                        DialogClose { open: open, "Decline" }
                        Button { onclick: move |_| open.set(false), "Accept" }
                    }
                }
            }
        }
    }
}
