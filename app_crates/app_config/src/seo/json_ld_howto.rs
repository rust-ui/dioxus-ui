use dioxus::prelude::*;
use serde::Serialize;

#[derive(Clone, Debug, PartialEq)]
pub struct HowToStep {
    pub name: String,
    pub text: String,
}

#[derive(Serialize)]
struct HowToSchema {
    #[serde(rename = "@context")]
    context: String,
    #[serde(rename = "@type")]
    type_: String,
    name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    description: Option<String>,
    step: Vec<Step>,
}

#[derive(Serialize)]
struct Step {
    #[serde(rename = "@type")]
    type_: String,
    name: String,
    text: String,
}

#[component]
pub fn JsonLdHowTo(name: String, #[props(default)] description: Option<String>, steps: Vec<HowToStep>) -> Element {
    let schema_steps: Vec<Step> =
        steps.into_iter().map(|s| Step { type_: "HowToStep".to_string(), name: s.name, text: s.text }).collect();

    let schema = HowToSchema {
        context: "https://schema.org".to_string(),
        type_: "HowTo".to_string(),
        name,
        description,
        step: schema_steps,
    };

    let json_content = serde_json::to_string(&schema).unwrap_or_else(|_| "{}".to_string());

    rsx! {
        document::Script { r#type: "application/ld+json", "{json_content}" }
    }
}
