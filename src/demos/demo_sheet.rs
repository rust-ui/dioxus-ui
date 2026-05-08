use dioxus::prelude::*;

use crate::ui::button::{Button, ButtonVariant};
use crate::ui::sheet::{
    Sheet, SheetBody, SheetClose, SheetContent, SheetDescription, SheetFooter, SheetHeader,
    SheetSide, SheetTitle, SheetTrigger,
};

#[component]
pub fn DemoSheet() -> Element {
    rsx! {
        Sheet {
            SheetTrigger { "Open Sheet" }
            SheetContent { side: SheetSide::Right,
                SheetHeader {
                    SheetTitle { "Edit Profile" }
                    SheetDescription { "Make changes to your profile here. Click save when done." }
                }
                SheetBody {
                    p { class: "text-sm text-muted-foreground",
                        "Profile settings and preferences will appear here."
                    }
                }
                SheetFooter {
                    SheetClose { "Cancel" }
                    Button { "Save Changes" }
                }
            }
        }
    }
}
