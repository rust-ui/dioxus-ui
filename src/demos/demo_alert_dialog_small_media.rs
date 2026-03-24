use dioxus::prelude::*;

use crate::ui::alert_dialog::{
    AlertDialog, AlertDialogBody, AlertDialogClose, AlertDialogContent, AlertDialogDescription,
    AlertDialogFooter, AlertDialogHeader, AlertDialogTitle, AlertDialogTrigger,
};
use crate::ui::button::Button;

#[component]
pub fn DemoAlertDialogSmallMedia() -> Element {
    rsx! {
        AlertDialog {
            AlertDialogTrigger { "Show Dialog" }
            AlertDialogContent { class: "w-[300px]",
                AlertDialogBody {
                    AlertDialogHeader { class: "items-center sm:items-center sm:text-center",
                        div { class: "flex justify-center items-center mb-2 rounded-md size-10 bg-muted",
                            svg {
                                class: "size-5",
                                xmlns: "http://www.w3.org/2000/svg",
                                view_box: "0 0 24 24",
                                fill: "none",
                                stroke: "currentColor",
                                stroke_width: "2",
                                stroke_linecap: "round",
                                stroke_linejoin: "round",
                                path { d: "M12 20h.01" }
                                path { d: "M2 8.82a15 15 0 0 1 20 0" }
                                path { d: "M5 12.859a10 10 0 0 1 14 0" }
                                path { d: "M8.5 16.429a5 5 0 0 1 7 0" }
                            }
                        }
                        AlertDialogTitle { "Allow accessory to connect?" }
                        AlertDialogDescription {
                            "Do you want to allow the Bluetooth accessory to connect to this device?"
                        }
                    }
                    AlertDialogFooter { class: "flex-row",
                        AlertDialogClose { class: "flex-1", "Don't allow" }
                        Button { r#type: "submit", class: "flex-1", "Allow" }
                    }
                }
            }
        }
    }
}
