use super::types::RegistryEntry;
use crate::demos::demo_bento_grid::DemoBentoGrid;
use crate::demos::demo_bento_grid6::DemoBentoGrid6;
use crate::markdown::converter::MdComponents;
use dioxus::prelude::*;

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
