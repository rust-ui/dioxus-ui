use super::RegistryEntry;
use crate::demos::demo_sheet::DemoSheet;
use crate::markdown::converter::MdComponents;
use dioxus::prelude::*;

pub static SHEET: RegistryEntry = RegistryEntry {
    slug: "sheet",
    raw: include_str!("../../public/docs/sheet.md"),
    tags: &[],
    components: sheet_components,
};

fn sheet_components() -> MdComponents {
    let mut c = MdComponents::new();
    c.add("DemoSheet", |_| rsx! { DemoSheet {} });
    c
}
