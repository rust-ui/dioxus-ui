use dioxus::prelude::*;
use icons::Fullscreen;

use crate::domain::blocks::block_entry::BlockEntry;

#[component]
pub fn BlockViewer(block_entry: BlockEntry) -> Element {
    let block_id = block_entry.block_id_kebab;
    let block_id_str = block_entry.block_id_str;
    let meta = block_id.meta();
    let iframe_height = meta.iframe_height;

    rsx! {
        div {
            id: block_id_str,
            "data-name": "__BlockViewer",
            class: "flex flex-col gap-4 scroll-mt-20",
            div { class: "flex gap-2 items-center",
                a {
                    href: "#{block_id_str}",
                    class: "flex-1 text-sm font-medium hover:underline underline-offset-2",
                    {block_id.to_title()}
                }
                a {
                    href: block_id.to_full_view_url(),
                    target: "_blank",
                    rel: "noopener noreferrer",
                    class: "inline-flex items-center gap-1.5 text-xs text-muted-foreground hover:text-foreground",
                    title: "Open in new tab",
                    Fullscreen { class: "size-4" }
                }
            }
            div {
                class: "overflow-hidden rounded-lg border bg-background w-full",
                style: "height: {iframe_height}; overflow-y: auto;",
                {block_id.to_component()}
            }
        }
    }
}
