use dioxus::prelude::*;

use crate::demos::demo_skeleton::DemoSkeleton;
use crate::demos::demo_skeleton_avatar::DemoSkeletonAvatar;
use crate::demos::demo_skeleton_form::DemoSkeletonForm;
use crate::demos::demo_skeleton_image::DemoSkeletonImage;
use crate::demos::demo_skeleton_table::DemoSkeletonTable;
use crate::demos::demo_skeleton_text::DemoSkeletonText;
use crate::markdown::converter::{MdComponents, MdNodeProps};
use crate::registry::RegistryEntry;

pub static SKELETON: RegistryEntry = RegistryEntry {
    slug: "skeleton",
    raw: include_str!("../../public/docs/skeleton.md"),
    components: skeleton_components,
};

fn skeleton_components() -> MdComponents {
    let mut c = MdComponents::new();
    c.add("demo-skeleton", |_: MdNodeProps| rsx! { DemoSkeleton {} });
    c.add("demo-skeleton-text", |_: MdNodeProps| rsx! { DemoSkeletonText {} });
    c.add("demo-skeleton-avatar", |_: MdNodeProps| rsx! { DemoSkeletonAvatar {} });
    c.add("demo-skeleton-form", |_: MdNodeProps| rsx! { DemoSkeletonForm {} });
    c.add("demo-skeleton-image", |_: MdNodeProps| rsx! { DemoSkeletonImage {} });
    c.add("demo-skeleton-table", |_: MdNodeProps| rsx! { DemoSkeletonTable {} });
    c
}
