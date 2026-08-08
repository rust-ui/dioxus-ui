use dioxus::prelude::*;
use registry::demos::demo_button_rtl::DemoButtonRtl;

use super::RegistryEntry;
use crate::markdown::converter::MdComponents;

pub static INTRODUCTION: RegistryEntry = RegistryEntry {
    slug: "introduction",
    raw: include_str!("../../public/docs/introduction.md"),
    tags: &[],
    components: MdComponents::new,
};

pub static INSTALLATION: RegistryEntry = RegistryEntry {
    slug: "installation",
    raw: include_str!("../../public/docs/installation.md"),
    tags: &[],
    components: MdComponents::new,
};

pub static CHANGELOG: RegistryEntry = RegistryEntry {
    slug: "changelog",
    raw: include_str!("../../public/docs/changelog.md"),
    tags: &[],
    components: MdComponents::new,
};

pub static FIGMA: RegistryEntry = RegistryEntry {
    slug: "figma",
    raw: include_str!("../../public/docs/figma.md"),
    tags: &[],
    components: MdComponents::new,
};

pub static RTL: RegistryEntry = RegistryEntry {
    slug: "rtl",
    raw: include_str!("../../public/docs/rtl.md"),
    tags: &[],
    components: rtl_components,
};

fn rtl_components() -> MdComponents {
    let mut c = MdComponents::new();
    c.add("DemoButtonRtl", |_| rsx! { DemoButtonRtl {} });
    c
}

pub static GET_STARTED: &[&RegistryEntry] = &[&INTRODUCTION, &INSTALLATION, &CHANGELOG, &FIGMA, &RTL];

pub fn find(slug: &str) -> Option<&'static RegistryEntry> {
    GET_STARTED.iter().copied().find(|e| e.slug == slug)
}

pub fn prev_next(slug: &str) -> (Option<&'static RegistryEntry>, Option<&'static RegistryEntry>) {
    let pos = GET_STARTED.iter().position(|e| e.slug == slug);
    match pos {
        None => (None, None),
        Some(i) => (
            if i > 0 { Some(GET_STARTED[i - 1]) } else { None },
            if i + 1 < GET_STARTED.len() { Some(GET_STARTED[i + 1]) } else { None },
        ),
    }
}
