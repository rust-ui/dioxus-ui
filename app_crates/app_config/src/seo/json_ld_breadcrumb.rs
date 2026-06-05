use dioxus::prelude::*;
use serde::Serialize;

#[derive(Clone, Debug, PartialEq)]
pub struct BreadcrumbItem {
    pub name: String,
    pub url: Option<String>,
}

#[derive(Serialize)]
struct BreadcrumbListSchema {
    #[serde(rename = "@context")]
    context: String,
    #[serde(rename = "@type")]
    type_: String,
    #[serde(rename = "itemListElement")]
    item_list_element: Vec<ListItem>,
}

#[derive(Serialize)]
struct ListItem {
    #[serde(rename = "@type")]
    type_: String,
    position: usize,
    name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    item: Option<String>,
}

#[component]
pub fn JsonLdBreadcrumb(items: Vec<BreadcrumbItem>) -> Element {
    let list_items: Vec<ListItem> = items
        .into_iter()
        .enumerate()
        .map(|(idx, crumb)| ListItem { type_: "ListItem".to_string(), position: idx + 1, name: crumb.name, item: crumb.url })
        .collect();

    let schema = BreadcrumbListSchema {
        context: "https://schema.org".to_string(),
        type_: "BreadcrumbList".to_string(),
        item_list_element: list_items,
    };

    let json_content = serde_json::to_string(&schema).unwrap_or_else(|_| "{}".to_string());

    rsx! {
        document::Script { r#type: "application/ld+json", "{json_content}" }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn breadcrumbs() -> Vec<BreadcrumbItem> {
        vec![
            BreadcrumbItem { name: "Home".to_string(), url: Some("https://dioxus-ui.com".to_string()) },
            BreadcrumbItem { name: "Components".to_string(), url: Some("https://dioxus-ui.com/components".to_string()) },
            BreadcrumbItem { name: "Button".to_string(), url: None },
        ]
    }

    #[test]
    fn serializes_to_json() {
        let items: Vec<ListItem> = breadcrumbs()
            .into_iter()
            .enumerate()
            .map(|(i, c)| ListItem { type_: "ListItem".to_string(), position: i + 1, name: c.name, item: c.url })
            .collect();
        let schema = BreadcrumbListSchema {
            context: "https://schema.org".to_string(),
            type_: "BreadcrumbList".to_string(),
            item_list_element: items,
        };
        let json = serde_json::to_string(&schema).unwrap();
        let v: serde_json::Value = serde_json::from_str(&json).unwrap();
        assert_eq!(v["@type"], "BreadcrumbList");
        assert_eq!(v["itemListElement"].as_array().unwrap().len(), 3);
    }

    #[test]
    fn last_item_has_no_url() {
        let items: Vec<ListItem> = breadcrumbs()
            .into_iter()
            .enumerate()
            .map(|(i, c)| ListItem { type_: "ListItem".to_string(), position: i + 1, name: c.name, item: c.url })
            .collect();
        let schema = BreadcrumbListSchema {
            context: "https://schema.org".to_string(),
            type_: "BreadcrumbList".to_string(),
            item_list_element: items,
        };
        let json = serde_json::to_value(&schema).unwrap();
        let last = json["itemListElement"].as_array().unwrap().last().unwrap();
        assert!(last.get("item").is_none());
    }
}
