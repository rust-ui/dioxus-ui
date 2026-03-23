use dioxus::prelude::*;

use crate::ui::alert_dialog::{
    AlertDialog, AlertDialogAction, AlertDialogClose, AlertDialogContent,
    AlertDialogDescription, AlertDialogFooter, AlertDialogHeader, AlertDialogTitle,
    AlertDialogTrigger,
};
use crate::ui::button::{Button, ButtonVariant};

#[component]
pub fn DemoAlertDialog() -> Element {
    rsx! {
        div { class: "flex justify-center",
            AlertDialog {
                AlertDialogTrigger {
                    Button { variant: ButtonVariant::Outline, "Show Dialog" }
                }
                AlertDialogContent {
                    AlertDialogHeader {
                        AlertDialogTitle { "Are you absolutely sure?" }
                        AlertDialogDescription {
                            "This action cannot be undone. This will permanently delete your account and remove your data from our servers."
                        }
                    }
                    AlertDialogFooter {
                        AlertDialogClose {
                            Button { variant: ButtonVariant::Outline, "Cancel" }
                        }
                        AlertDialogAction {
                            Button { variant: ButtonVariant::Destructive, "Delete" }
                        }
                    }
                }
            }
        }
    }
}
