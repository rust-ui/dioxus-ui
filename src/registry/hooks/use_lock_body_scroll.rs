use crate::markdown::converter::MdComponents;
use crate::registry::types::RegistryEntry;

pub static USE_LOCK_BODY_SCROLL: RegistryEntry = RegistryEntry {
    slug: "use-lock-body-scroll",
    raw: include_str!("../../../public/docs/hooks/use_lock_body_scroll.md"),
    tags: &["utils", "dialog"],
    components: use_lock_body_scroll_components,
};

fn use_lock_body_scroll_components() -> MdComponents {
    MdComponents::new()
}
