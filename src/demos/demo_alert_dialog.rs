use dioxus::prelude::*;

use crate::ui::alert_dialog::{
    AlertDialog, AlertDialogDescription, AlertDialogFooter, AlertDialogHeader, AlertDialogTitle,
};
use crate::ui::button::{Button, ButtonVariant};

#[component]
pub fn DemoAlertDialog() -> Element {
    let mut open = use_signal(|| false);

    rsx! {
        div { class: "flex justify-center",
            Button {
                variant: ButtonVariant::Outline,
                onclick: move |_| open.set(true),
                "Show Dialog"
            }
            AlertDialog {
                open: open,
                AlertDialogHeader {
                    AlertDialogTitle { "Are you absolutely sure?" }
                    AlertDialogDescription {
                        "This action cannot be undone. This will permanently delete your account and remove your data from our servers."
                    }
                }
                AlertDialogFooter {
                    Button {
                        variant: ButtonVariant::Outline,
                        onclick: move |_| open.set(false),
                        "Cancel"
                    }
                    Button {
                        variant: ButtonVariant::Destructive,
                        onclick: move |_| open.set(false),
                        "Delete"
                    }
                }
            }
        }
    }
}
