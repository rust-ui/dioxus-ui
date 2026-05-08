use super::types::RegistryEntry;
use crate::demos::demo_callout::DemoCallout;
use crate::demos::demo_callout_info::DemoCalloutInfo;
use crate::demos::demo_callout_warning::DemoCalloutWarning;
use crate::markdown::converter::MdComponents;
use dioxus::prelude::*;

pub static CALLOUT: RegistryEntry = RegistryEntry {
    slug: "callout",
    raw: include_str!("../../public/docs/callout.md"),
    tags: &[],
    components: callout_components,
};

fn callout_components() -> MdComponents {
    let mut c = MdComponents::new();
    c.add("DemoCallout", |_| rsx! { DemoCallout {} });
    c.add("DemoCalloutInfo", |_| rsx! { DemoCalloutInfo {} });
    c.add("DemoCalloutWarning", |_| rsx! { DemoCalloutWarning {} });
    c
}
