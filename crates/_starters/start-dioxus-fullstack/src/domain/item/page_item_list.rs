use dioxus::prelude::*;

use crate::app::Route;
use crate::components::ui::button::Button;
use crate::components::ui::card::{CardDescription, CardHeader, CardTitle};
use crate::components::ui::empty::{Empty, EmptyDescription, EmptyTitle};
use crate::components::ui::input::Input;
use crate::components::ui::skeleton::Skeleton;
use crate::domain::item::server_fns::{create_item, get_items};

#[component]
pub fn ItemListPage() -> Element {
    let mut items = use_server_future(get_items)?;
    let navigator = use_navigator();

    let mut new_title = use_signal(String::new);
    let mut creating = use_signal(|| false);

    rsx! {
        div { class: "flex flex-col gap-6 p-6 pt-[calc(env(safe-area-inset-top)+4rem)] sm:pt-6",
            div { class: "flex items-center justify-between",
                h1 { class: "text-2xl font-bold tracking-tight", "Items" }
            }

            // Create form
            form {
                class: "flex gap-2",
                onsubmit: move |e| {
                    e.prevent_default();
                    let title = new_title().trim().to_string();
                    if title.is_empty() { return; }
                    creating.set(true);
                    spawn(async move {
                        if create_item(title, None).await.is_ok() {
                            new_title.set(String::new());
                            items.restart();
                        }
                        creating.set(false);
                    });
                },
                Input {
                    placeholder: "New item title...",
                    value: new_title(),
                    oninput: move |e: FormEvent| new_title.set(e.value()),
                }
                Button {
                    disabled: creating(),
                    "Add"
                }
            }

            // List
            match &*items.read() {
                Some(Ok(list)) if list.is_empty() => rsx! {
                    Empty {
                        EmptyTitle { "No items yet" }
                        EmptyDescription { "Add your first item above." }
                    }
                },
                Some(Ok(list)) => rsx! {
                    div { class: "flex flex-col gap-3",
                        for item in list.iter() {
                            {
                                let id = item.id.to_string();
                                let id2 = id.clone();
                                rsx! {
                                    div {
                                        key: "{id}",
                                        class: "cursor-pointer rounded-xl border bg-card text-card-foreground shadow hover:bg-accent/50 transition-colors",
                                        onclick: move |_| { navigator.push(Route::ItemDetails { id: id2.clone() }); },
                                        CardHeader {
                                            CardTitle { "{item.title}" }
                                            if let Some(desc) = &item.description {
                                                CardDescription { "{desc}" }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                },
                Some(Err(e)) => rsx! {
                    p { class: "text-destructive text-sm", "Error: {e}" }
                },
                None => rsx! {
                    div { class: "flex flex-col gap-3",
                        Skeleton { class: "h-20 w-full" }
                        Skeleton { class: "h-20 w-full" }
                        Skeleton { class: "h-20 w-full" }
                    }
                },
            }
        }
    }
}
