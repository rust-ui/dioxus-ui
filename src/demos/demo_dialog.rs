use dioxus::prelude::*;

use crate::ui::button::Button;
use crate::ui::dialog::{
    Dialog, DialogBody, DialogClose, DialogContent, DialogDescription, DialogFooter, DialogHeader,
    DialogTitle, DialogTrigger,
};
use crate::ui::input::Input;
use crate::ui::label::Label;

#[component]
pub fn DemoDialog() -> Element {
    let mut open = use_signal(|| false);
    let mut name = use_signal(|| "Max Wells".to_string());
    let mut username = use_signal(|| "@maxwells".to_string());

    rsx! {
        Dialog { open: open,
            DialogTrigger {
                onclick: move |_| open.set(true),
                "Open Dialog"
            }
            DialogContent { open: open,
                DialogBody {
                    DialogHeader {
                        DialogTitle { "Edit profile" }
                        DialogDescription {
                            "Make changes to your profile here. Click save when you're done."
                        }
                    }
                    div { class: "flex flex-col gap-4",
                        div { class: "flex flex-col gap-2",
                            Label { html_for: "dialog-name", "Name" }
                            Input {
                                id: "dialog-name",
                                placeholder: name(),
                                oninput: move |e: FormEvent| name.set(e.value()),
                            }
                        }
                        div { class: "flex flex-col gap-2",
                            Label { html_for: "dialog-username", "Username" }
                            Input {
                                id: "dialog-username",
                                placeholder: username(),
                                oninput: move |e: FormEvent| username.set(e.value()),
                            }
                        }
                    }
                    DialogFooter {
                        DialogClose { open: open, "Cancel" }
                        Button { onclick: move |_| open.set(false), "Save changes" }
                    }
                }
            }
        }
    }
}
