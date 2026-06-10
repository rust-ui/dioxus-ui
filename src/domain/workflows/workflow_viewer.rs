use dioxus::prelude::*;
use icons::Fullscreen;

use crate::domain::workflows::workflow_entry::WorkflowEntry;

#[component]
pub fn WorkflowViewer(workflow_entry: WorkflowEntry) -> Element {
    let workflow_id = workflow_entry.workflow_id_kebab;
    let workflow_id_str = workflow_entry.workflow_id_str;
    let meta = workflow_id.meta();
    let iframe_height = meta.iframe_height;

    rsx! {
        div {
            id: workflow_id_str,
            "data-name": "__WorkflowViewer",
            class: "flex flex-col gap-4 scroll-mt-20",
            div { class: "flex gap-2 items-center",
                a {
                    href: "#{workflow_id_str}",
                    class: "flex-1 text-sm font-medium hover:underline underline-offset-2",
                    {workflow_id.to_title()}
                }
                a {
                    href: workflow_id.to_full_view_url(),
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
                {workflow_id.to_component()}
            }
        }
    }
}
