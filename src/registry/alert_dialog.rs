use dioxus::prelude::*;

use crate::demos::demo_alert_dialog::DemoAlertDialog;
use crate::markdown::converter::{MdComponents, MdNodeProps};
use crate::registry::RegistryEntry;

pub static ALERT_DIALOG: RegistryEntry = RegistryEntry {
    slug: "alert-dialog",
    raw: include_str!("../../public/docs/alert_dialog.md"),
    tags: &["overlay"],
    components: alert_dialog_components,
};

fn alert_dialog_components() -> MdComponents {
    let mut c = MdComponents::new();
    c.add("DemoAlertDialog", |_: MdNodeProps| rsx! { DemoAlertDialog {} });
    c
}
