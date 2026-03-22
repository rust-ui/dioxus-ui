use dioxus::prelude::*;

use crate::markdown::converter::convert_md;
use crate::registry;

#[component]
pub fn ComponentPage(name: String) -> Element {
    let entry = registry::find(&name);

    rsx! {
        div { class: "flex flex-col gap-8 max-w-2xl",
            match entry {
                None => rsx! {
                    p { class: "text-muted-foreground", "Component not found: {name}" }
                },
                Some(e) => rsx! {
                    div {
                        h1 { class: "text-2xl font-bold mb-1", "{e.title()}" }
                        p { class: "text-muted-foreground text-sm", "{e.description()}" }
                    }
                    {convert_md(e.body_md(), &(e.components)())}
                },
            }
        }
    }
}
