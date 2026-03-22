use dioxus::prelude::*;

use crate::markdown::converter::convert_md;
use crate::registry::{self, prev_next};
use crate::ui::doc_header::DocHeader;

#[component]
pub fn ComponentPage(name: String) -> Element {
    let entry = registry::find(&name);
    let (prev, next) = prev_next(&name);

    rsx! {
        div { class: "flex flex-col max-w-2xl",
            match entry {
                None => rsx! {
                    p { class: "text-muted-foreground", "Component not found: {name}" }
                },
                Some(e) => rsx! {
                    DocHeader {
                        title: e.title(),
                        description: e.description(),
                        tags: e.tags.to_vec(),
                        prev,
                        next,
                    }
                    {convert_md(e.body_md(), &(e.components)())}
                },
            }
        }
    }
}
