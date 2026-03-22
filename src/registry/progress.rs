use dioxus::prelude::*;

use crate::demos::demo_progress::DemoProgress;
use crate::demos::demo_progress_animated::DemoProgressAnimated;
use crate::markdown::converter::{MdComponents, MdNodeProps};
use crate::registry::RegistryEntry;

pub static PROGRESS: RegistryEntry = RegistryEntry {
    slug: "progress",
    raw: include_str!("../../public/docs/progress.md"),
    tags: &["feedback", "display"],
    components: progress_components,
};

fn progress_components() -> MdComponents {
    let mut c = MdComponents::new();
    c.add("DemoProgress", |_: MdNodeProps| rsx! { DemoProgress {} });
    c.add("DemoProgressAnimated", |_: MdNodeProps| rsx! { DemoProgressAnimated {} });
    c
}
