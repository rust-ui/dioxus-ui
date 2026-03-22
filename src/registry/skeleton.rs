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
    c.add("DemoSkeleton", |_: MdNodeProps| rsx! { DemoSkeleton {} });
    c.add("DemoSkeletonText", |_: MdNodeProps| rsx! { DemoSkeletonText {} });
    c.add("DemoSkeletonAvatar", |_: MdNodeProps| rsx! { DemoSkeletonAvatar {} });
    c.add("DemoSkeletonForm", |_: MdNodeProps| rsx! { DemoSkeletonForm {} });
    c.add("DemoSkeletonImage", |_: MdNodeProps| rsx! { DemoSkeletonImage {} });
    c.add("DemoSkeletonTable", |_: MdNodeProps| rsx! { DemoSkeletonTable {} });
    c
}
