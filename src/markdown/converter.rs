use std::collections::HashMap;

use dioxus::prelude::*;
use html_parser::{Dom, Element as HtmlElement, Node};

use crate::markdown::markdown_to_html;

// ---------------------------------------------------------------------------
// Types
// ---------------------------------------------------------------------------

pub struct MdNodeProps {
    pub id: Option<String>,
    pub classes: Vec<String>,
    pub attributes: HashMap<String, Option<String>>,
    pub children: Vec<Element>,
    pub text_content: String,
}

/// Function-pointer type for custom components.
/// Using fn pointers (not closures) so the registry can be stored statically.
pub type MdComponent = fn(MdNodeProps) -> Element;

#[derive(Default)]
pub struct MdComponents(HashMap<&'static str, MdComponent>);

impl MdComponents {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn add(&mut self, tag: &'static str, f: MdComponent) -> &mut Self {
        self.0.insert(tag, f);
        self
    }
}

// ---------------------------------------------------------------------------
// Entry point
// ---------------------------------------------------------------------------

/// Convert a markdown string to a Dioxus Element tree.
pub fn convert_md(md: &str, components: &MdComponents) -> Element {
    let html = markdown_to_html(md);
    let dom = match Dom::parse(&html) {
        Ok(d) => d,
        Err(_) => return rsx! {},
    };
    let children: Vec<Element> = dom.children.iter().map(|n| process_node(n, components)).collect();
    rsx! {
        {children.into_iter()}
    }
}

// ---------------------------------------------------------------------------
// Recursive processing
// ---------------------------------------------------------------------------

fn process_node(node: &Node, components: &MdComponents) -> Element {
    match node {
        Node::Text(t) => {
            let t = t.clone();
            rsx! { "{t}" }
        }
        Node::Element(el) => process_element(el, components),
        Node::Comment(_) => rsx! {},
    }
}

fn extract_text(nodes: &[Node]) -> String {
    nodes
        .iter()
        .map(|n| match n {
            Node::Text(t) => t.clone(),
            Node::Element(el) => extract_text(&el.children),
            _ => String::new(),
        })
        .collect::<Vec<_>>()
        .join("")
}

fn process_element(el: &HtmlElement, components: &MdComponents) -> Element {
    let children: Vec<Element> = el.children.iter().map(|n| process_node(n, components)).collect();

    // Check custom registry first
    if let Some(component) = components.0.get(el.name.as_str()) {
        return component(MdNodeProps {
            id: el.id.clone(),
            classes: el.classes.clone(),
            attributes: el.attributes.clone(),
            children,
            text_content: extract_text(&el.children),
        });
    }

    // Default element rendering with Tailwind classes
    match el.name.as_str() {
        "h1" => rsx! { h1 { class: "text-3xl font-bold mt-6 mb-4", {children.into_iter()} } },
        "h2" => rsx! { h2 { class: "text-2xl font-semibold mt-8 mb-3 pb-1 border-b border-border", {children.into_iter()} } },
        "h3" => rsx! { h3 { class: "text-xl font-semibold mt-6 mb-2", {children.into_iter()} } },
        "h4" => rsx! { h4 { class: "text-base font-semibold mt-4 mb-1", {children.into_iter()} } },
        "p" => rsx! { p { class: "mb-4 leading-7 text-foreground/90", {children.into_iter()} } },
        "ul" => rsx! { ul { class: "list-disc pl-6 mb-4 space-y-1", {children.into_iter()} } },
        "ol" => rsx! { ol { class: "list-decimal pl-6 mb-4 space-y-1", {children.into_iter()} } },
        "li" => rsx! { li { class: "leading-7", {children.into_iter()} } },
        "a" => {
            let href = el.attributes.get("href").and_then(|v| v.clone()).unwrap_or_default();
            rsx! { a { class: "text-primary underline underline-offset-4 hover:opacity-80", href: "{href}", {children.into_iter()} } }
        }
        "code" => rsx! { code { class: "bg-muted rounded px-1.5 py-0.5 font-mono text-sm", {children.into_iter()} } },
        "pre" => rsx! { pre { class: "bg-muted rounded-lg p-4 mb-4 overflow-x-auto text-sm", {children.into_iter()} } },
        "strong" | "b" => rsx! { strong { class: "font-semibold", {children.into_iter()} } },
        "em" | "i" => rsx! { em { {children.into_iter()} } },
        "blockquote" => rsx! { blockquote { class: "border-l-4 border-border pl-4 italic text-muted-foreground mb-4", {children.into_iter()} } },
        "table" => rsx! { div { class: "overflow-x-auto mb-4", table { class: "w-full text-sm border-collapse", {children.into_iter()} } } },
        "thead" => rsx! { thead { class: "border-b border-border", {children.into_iter()} } },
        "tbody" => rsx! { tbody { {children.into_iter()} } },
        "tr" => rsx! { tr { class: "border-b border-border last:border-0", {children.into_iter()} } },
        "th" => rsx! { th { class: "px-4 py-2 text-left font-semibold", {children.into_iter()} } },
        "td" => rsx! { td { class: "px-4 py-2", {children.into_iter()} } },
        "hr" => rsx! { hr { class: "border-border my-6" } },
        _ => rsx! { {children.into_iter()} },
    }
}
