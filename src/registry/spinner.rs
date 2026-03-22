use dioxus::prelude::*;

use crate::demos::demo_spinner::DemoSpinner;
use crate::demos::demo_spinner_button::DemoSpinnerButton;
use crate::markdown::converter::{MdComponents, MdNodeProps};
use crate::registry::RegistryEntry;

pub static SPINNER: RegistryEntry = RegistryEntry {
    slug: "spinner",
    raw: include_str!("../../public/docs/spinner.md"),
    components: spinner_components,
};

fn spinner_components() -> MdComponents {
    let mut c = MdComponents::new();
    c.add("demo-spinner", |_: MdNodeProps| rsx! { DemoSpinner {} });
    c.add("demo-spinner-button", |_: MdNodeProps| rsx! { DemoSpinnerButton {} });
    c
}
