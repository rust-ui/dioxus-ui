use dioxus::prelude::*;
use registry::demos::demo_scroll_area::DemoScrollArea;
use registry::demos::demo_scroll_area_horizontal::DemoScrollAreaHorizontal;
use registry::demos::demo_scroll_area_rtl::DemoScrollAreaRtl;

use super::types::RegistryEntry;
use crate::markdown::converter::MdComponents;

pub static SCROLL_AREA: RegistryEntry = RegistryEntry {
    slug: "scroll-area",
    raw: include_str!("../../public/docs/scroll_area.md"),
    tags: &[],
    components: scroll_area_components,
};

fn scroll_area_components() -> MdComponents {
    let mut c = MdComponents::new();
    c.add("DemoScrollArea", |_| rsx! { DemoScrollArea {} });
    c.add("DemoScrollAreaHorizontal", |_| rsx! { DemoScrollAreaHorizontal {} });
    c.add("DemoScrollAreaRtl", |_| rsx! { DemoScrollAreaRtl {} });
    c
}
