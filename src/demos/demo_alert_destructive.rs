use dioxus::prelude::*;

use crate::ui::alert::{Alert, AlertDescription, AlertTitle, AlertVariant};

#[component]
pub fn DemoAlertDestructive() -> Element {
    rsx! {
        Alert { variant: AlertVariant::Destructive,
            AlertTitle { "Error" }
            AlertDescription { "Your session has expired. Please log in again." }
        }
    }
}
