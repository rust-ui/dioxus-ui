use dioxus::prelude::*;
use serde::Serialize;

use super::site_config::SiteConfig;

#[derive(Serialize)]
struct TechArticleSchema {
    #[serde(rename = "@context")]
    context: String,
    #[serde(rename = "@type")]
    type_: String,
    headline: String,
    description: String,
    author: Author,
    #[serde(rename = "datePublished")]
    date_published: String,
    #[serde(rename = "dateModified")]
    date_modified: String,
    url: String,
    #[serde(rename = "mainEntityOfPage")]
    main_entity_of_page: MainEntity,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    keywords: Vec<String>,
    #[serde(rename = "articleSection")]
    article_section: String,
}

#[derive(Serialize)]
struct Author {
    #[serde(rename = "@type")]
    type_: String,
    #[serde(rename = "@id")]
    id: String,
}

#[derive(Serialize)]
struct MainEntity {
    #[serde(rename = "@type")]
    type_: String,
    #[serde(rename = "@id")]
    id: String,
}

#[component]
pub fn JsonLdArticle(
    title: String,
    description: String,
    url: String,
    #[props(default)] date_published: Option<String>,
    #[props(default)] date_modified: Option<String>,
    #[props(default = Vec::new())] keywords: Vec<String>,
    #[props(default = "Documentation".to_string())] article_section: String,
) -> Element {
    let schema = TechArticleSchema {
        context: "https://schema.org".to_string(),
        type_: "TechArticle".to_string(),
        headline: title,
        description,
        author: Author { type_: "Organization".to_string(), id: format!("{}/#organization", SiteConfig::BASE_URL) },
        date_published: date_published.unwrap_or_else(|| "2024-01-01".to_string()),
        date_modified: date_modified.unwrap_or_else(|| "2025-11-08".to_string()),
        main_entity_of_page: MainEntity { type_: "WebPage".to_string(), id: url.clone() },
        url,
        keywords,
        article_section,
    };

    let json_content = serde_json::to_string(&schema).unwrap_or_else(|_| "{}".to_string());

    rsx! {
        document::Script { r#type: "application/ld+json", "{json_content}" }
    }
}
