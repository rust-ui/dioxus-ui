use dioxus::prelude::*;

use crate::demos::demo_alert::DemoAlert;
use crate::demos::demo_alert_destructive::DemoAlertDestructive;
use crate::markdown::converter::{MdComponents, MdNodeProps};
use crate::registry::RegistryEntry;

pub static ALERT: RegistryEntry = RegistryEntry {
    slug: "alert",
    raw: include_str!("../../public/docs/alert.md"),
    tags: &["feedback", "display"],
    components: alert_components,
};

fn alert_components() -> MdComponents {
    let mut c = MdComponents::new();
    c.add("DemoAlert", |_: MdNodeProps| rsx! { DemoAlert {} });
    c.add("DemoAlertDestructive", |_: MdNodeProps| rsx! { DemoAlertDestructive {} });
    c
}
