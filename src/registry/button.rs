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
    components: button_components,
};

fn button_components() -> MdComponents {
    let mut c = MdComponents::new();
    c.add("demo-button", |_: MdNodeProps| rsx! { DemoButton {} });
    c.add("demo-button-variants", |_: MdNodeProps| rsx! { DemoButtonVariants {} });
    c.add("demo-button-sizes", |_: MdNodeProps| rsx! { DemoButtonSizes {} });
    c.add("demo-button-disabled", |_: MdNodeProps| rsx! { DemoButtonDisabled {} });
    c.add("demo-button-stateful", |_: MdNodeProps| rsx! { DemoButtonStateful {} });
    c.add("demo-button-reactive", |_: MdNodeProps| rsx! { DemoButtonReactive {} });
    c.add("demo-button-override", |_: MdNodeProps| rsx! { DemoButtonOverride {} });
    c.add("demo-button-href", |_: MdNodeProps| rsx! { DemoButtonHref {} });
    c.add("demo-button-group", |_: MdNodeProps| rsx! { DemoButtonGroup {} });
    c.add("demo-button-group-icon", |_: MdNodeProps| rsx! { DemoButtonGroupIcon {} });
    c
}
