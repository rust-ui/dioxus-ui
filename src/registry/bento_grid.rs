use dioxus::prelude::*;
use registry::demos::demo_bento_grid::DemoBentoGrid;
use registry::demos::demo_bento_grid6::DemoBentoGrid6;

use super::types::RegistryEntry;
use crate::markdown::converter::MdComponents;

pub static BENTO_GRID: RegistryEntry = RegistryEntry {
    slug: "bento-grid",
    raw: include_str!("../../public/docs/bento_grid.md"),
    tags: &[],
    components: bento_grid_components,
};

fn bento_grid_components() -> MdComponents {
    let mut c = MdComponents::new();
    c.add("DemoBentoGrid", |_| rsx! { DemoBentoGrid {} });
    c.add("DemoBentoGrid6", |_| rsx! { DemoBentoGrid6 {} });
    c
}
