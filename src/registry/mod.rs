use crate::markdown::{markdown_to_html, parse_md};

pub struct RegistryEntry {
    pub slug: &'static str,
    pub raw: &'static str,
}

impl RegistryEntry {
    pub fn title(&self) -> String {
        parse_md(self.raw).0.title
    }

    pub fn description(&self) -> String {
        parse_md(self.raw).0.description
    }

    pub fn body_html(&self) -> String {
        let (_, body) = parse_md(self.raw);
        markdown_to_html(body)
    }
}

pub static REGISTRY: &[RegistryEntry] = &[
    RegistryEntry {
        slug: "button",
        raw: include_str!("../../assets/docs/button.md"),
    },
];

pub fn find(slug: &str) -> Option<&'static RegistryEntry> {
    REGISTRY.iter().find(|e| e.slug == slug)
}
