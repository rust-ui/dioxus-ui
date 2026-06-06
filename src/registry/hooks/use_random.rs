use crate::markdown::converter::MdComponents;
use crate::registry::types::RegistryEntry;

pub static USE_RANDOM: RegistryEntry = RegistryEntry {
    slug: "use-random",
    raw: include_str!("../../../public/docs/hooks/use_random.md"),
    tags: &["utils"],
    components: use_random_components,
};

fn use_random_components() -> MdComponents {
    MdComponents::new()
}
