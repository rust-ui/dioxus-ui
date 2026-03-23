use dioxus::prelude::*;

use crate::ui::card::{Card, CardContent, CardHeader};
use crate::ui::collapsible::{Collapsible, CollapsibleContent, CollapsibleTrigger};
use crate::ui::tabs::{Tabs, TabsList, TabsTrigger};

#[derive(Clone, PartialEq)]
pub enum FileTreeItem {
    File { name: &'static str },
    Folder { name: &'static str, items: Vec<FileTreeItem> },
}

#[component]
fn FileTreeNode(item: FileTreeItem) -> Element {
    match item {
        FileTreeItem::File { name } => rsx! {
            div { class: "flex gap-2 items-center py-1 px-2 w-full text-sm rounded-md transition-colors text-foreground hover:bg-accent hover:text-accent-foreground",
                svg {
                    xmlns: "http://www.w3.org/2000/svg",
                    class: "size-4 shrink-0",
                    view_box: "0 0 24 24",
                    fill: "none",
                    stroke: "currentColor",
                    stroke_width: "2",
                    stroke_linecap: "round",
                    stroke_linejoin: "round",
                    path { d: "M15 2H6a2 2 0 0 0-2 2v16a2 2 0 0 0 2 2h12a2 2 0 0 0 2-2V7Z" }
                    path { d: "M14 2v4a2 2 0 0 0 2 2h4" }
                }
                span { {name} }
            }
        },
        FileTreeItem::Folder { name, items } => rsx! {
            Collapsible {
                CollapsibleTrigger { class: "flex gap-2 items-center py-1 px-2 w-full text-sm rounded-md transition-colors group hover:bg-accent hover:text-accent-foreground",
                    svg {
                        xmlns: "http://www.w3.org/2000/svg",
                        class: "transition-transform duration-200 size-4 shrink-0 group-data-[state=open]:rotate-90",
                        view_box: "0 0 24 24",
                        fill: "none",
                        stroke: "currentColor",
                        stroke_width: "2",
                        stroke_linecap: "round",
                        stroke_linejoin: "round",
                        path { d: "m9 18 6-6-6-6" }
                    }
                    svg {
                        xmlns: "http://www.w3.org/2000/svg",
                        class: "size-4 shrink-0",
                        view_box: "0 0 24 24",
                        fill: "none",
                        stroke: "currentColor",
                        stroke_width: "2",
                        stroke_linecap: "round",
                        stroke_linejoin: "round",
                        path { d: "M20 20a2 2 0 0 0 2-2V8a2 2 0 0 0-2-2h-7.9a2 2 0 0 1-1.69-.9L9.6 3.9A2 2 0 0 0 7.93 3H4a2 2 0 0 0-2 2v13a2 2 0 0 0 2 2Z" }
                    }
                    {name}
                }
                CollapsibleContent { class: "flex flex-col gap-1 mt-1 ml-5",
                    for child in items {
                        FileTreeNode { item: child }
                    }
                }
            }
        },
    }
}

#[component]
pub fn DemoCollapsibleFileTree() -> Element {
    let file_tree: Vec<FileTreeItem> = vec![
        FileTreeItem::Folder {
            name: "components",
            items: vec![
                FileTreeItem::Folder {
                    name: "ui",
                    items: vec![
                        FileTreeItem::File { name: "button.tsx" },
                        FileTreeItem::File { name: "card.tsx" },
                        FileTreeItem::File { name: "dialog.tsx" },
                        FileTreeItem::File { name: "input.tsx" },
                        FileTreeItem::File { name: "select.tsx" },
                    ],
                },
                FileTreeItem::File { name: "login-form.tsx" },
                FileTreeItem::File { name: "register-form.tsx" },
            ],
        },
        FileTreeItem::Folder {
            name: "lib",
            items: vec![
                FileTreeItem::File { name: "utils.ts" },
                FileTreeItem::File { name: "cn.ts" },
                FileTreeItem::File { name: "api.ts" },
            ],
        },
        FileTreeItem::Folder {
            name: "hooks",
            items: vec![
                FileTreeItem::File { name: "use-media-query.ts" },
                FileTreeItem::File { name: "use-debounce.ts" },
                FileTreeItem::File { name: "use-local-storage.ts" },
            ],
        },
        FileTreeItem::File { name: "app.tsx" },
        FileTreeItem::File { name: "layout.tsx" },
        FileTreeItem::File { name: "globals.css" },
        FileTreeItem::File { name: "package.json" },
    ];

    rsx! {
        Card { class: "gap-2 mx-auto w-full max-w-[16rem]",
            CardHeader {
                Tabs { default_value: "explorer",
                    TabsList { class: "w-full",
                        TabsTrigger { value: "explorer", "Explorer" }
                        TabsTrigger { value: "outline", "Outline" }
                    }
                }
            }
            CardContent {
                div { class: "flex flex-col gap-1",
                    for item in file_tree {
                        FileTreeNode { item: item }
                    }
                }
            }
        }
    }
}
