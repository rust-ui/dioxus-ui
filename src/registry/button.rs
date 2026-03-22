use dioxus::prelude::*;

use crate::demos::demo_button::DemoButton;
use crate::demos::demo_button_disabled::DemoButtonDisabled;
use crate::demos::demo_button_group::DemoButtonGroup;
use crate::demos::demo_button_group_icon::DemoButtonGroupIcon;
use crate::demos::demo_button_href::DemoButtonHref;
use crate::demos::demo_button_override::DemoButtonOverride;
use crate::demos::demo_button_reactive::DemoButtonReactive;
use crate::demos::demo_button_sizes::DemoButtonSizes;
use crate::demos::demo_button_stateful::DemoButtonStateful;
use crate::demos::demo_button_variants::DemoButtonVariants;
use crate::markdown::converter::{MdComponents, MdNodeProps};
use crate::registry::RegistryEntry;

pub static BUTTON: RegistryEntry = RegistryEntry {
    slug: "button",
    raw: include_str!("../../public/docs/button.md"),
    tags: &["interactive", "form"],
    components: button_components,
};

fn button_components() -> MdComponents {
    let mut c = MdComponents::new();
    c.add("DemoButton", |_: MdNodeProps| rsx! { DemoButton {} });
    c.add("DemoButtonVariants", |_: MdNodeProps| rsx! { DemoButtonVariants {} });
    c.add("DemoButtonSizes", |_: MdNodeProps| rsx! { DemoButtonSizes {} });
    c.add("DemoButtonDisabled", |_: MdNodeProps| rsx! { DemoButtonDisabled {} });
    c.add("DemoButtonStateful", |_: MdNodeProps| rsx! { DemoButtonStateful {} });
    c.add("DemoButtonReactive", |_: MdNodeProps| rsx! { DemoButtonReactive {} });
    c.add("DemoButtonOverride", |_: MdNodeProps| rsx! { DemoButtonOverride {} });
    c.add("DemoButtonHref", |_: MdNodeProps| rsx! { DemoButtonHref {} });
    c.add("DemoButtonGroup", |_: MdNodeProps| rsx! { DemoButtonGroup {} });
    c.add("DemoButtonGroupIcon", |_: MdNodeProps| rsx! { DemoButtonGroupIcon {} });
    c
}
