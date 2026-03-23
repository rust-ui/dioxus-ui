use dioxus::prelude::*;

use crate::demos::demo_theme_toggle::DemoThemeToggle;
use crate::markdown::converter::{MdComponents, MdNodeProps};
use crate::registry::RegistryEntry;

pub static THEME_TOGGLE: RegistryEntry = RegistryEntry {
    slug: "theme-toggle",
    raw: include_str!("../../public/docs/theme_toggle.md"),
    tags: &["utility"],
    components: theme_toggle_components,
};

fn theme_toggle_components() -> MdComponents {
    let mut c = MdComponents::new();
    c.add("DemoThemeToggle", |_: MdNodeProps| rsx! { DemoThemeToggle {} });
    c
}
