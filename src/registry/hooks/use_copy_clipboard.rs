use crate::markdown::converter::MdComponents;
use crate::registry::types::RegistryEntry;

pub static USE_COPY_CLIPBOARD: RegistryEntry = RegistryEntry {
    slug: "use-copy-clipboard",
    raw: include_str!("../../../public/docs/hooks/use_copy_clipboard.md"),
    tags: &["utils"],
    components: use_copy_clipboard_components,
};

fn use_copy_clipboard_components() -> MdComponents {
    MdComponents::new()
}
