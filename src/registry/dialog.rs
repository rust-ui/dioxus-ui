use dioxus::prelude::*;

use crate::demos::demo_dialog::DemoDialog;
use crate::demos::demo_dialog_scrollable::DemoDialogScrollable;
use crate::markdown::converter::{MdComponents, MdNodeProps};
use crate::registry::RegistryEntry;

pub static DIALOG: RegistryEntry = RegistryEntry {
    slug: "dialog",
    raw: include_str!("../../public/docs/dialog.md"),
    tags: &["overlay"],
    components: dialog_components,
};

fn dialog_components() -> MdComponents {
    let mut c = MdComponents::new();
    c.add("DemoDialog", |_: MdNodeProps| rsx! { DemoDialog {} });
    c.add("DemoDialogScrollable", |_: MdNodeProps| rsx! { DemoDialogScrollable {} });
    c
}
