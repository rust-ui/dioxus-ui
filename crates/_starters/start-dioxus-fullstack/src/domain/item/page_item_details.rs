use dioxus::prelude::*;
use uuid::Uuid;

use crate::app::Route;
use crate::components::layout::back_button::BackButton;
use crate::components::ui::button::{Button, ButtonVariant};
use crate::components::ui::card::{Card, CardContent, CardHeader, CardTitle};
use crate::components::ui::skeleton::Skeleton;
use crate::domain::item::server_fns::{delete_item, get_item};
use crate::utils::param::parse_uuid;

#[component]
pub fn ItemDetailsPage(id: String) -> Element {
    let uuid = parse_uuid(&id);
    let navigator = use_navigator();

    let item = use_server_future(move || {
        let uuid = uuid;
        async move {
            match uuid {
                Some(id) => get_item(id).await,
                None => Ok(None),
            }
        }
    })?;

    rsx! {
        div { class: "flex flex-col gap-6 p-6 pt-[calc(env(safe-area-inset-top)+4rem)] sm:pt-6",
            div { class: "flex items-center gap-3",
                BackButton {}
                h1 { class: "text-2xl font-bold tracking-tight", "Item Details" }
            }

            match &*item.read() {
                Some(Ok(Some(item))) => {
                    let item_id = item.id;
                    rsx! {
                        Card {
                            CardHeader {
                                CardTitle { "{item.title}" }
                            }
                            CardContent { class: "flex flex-col gap-4",
                                if let Some(desc) = &item.description {
                                    p { class: "text-muted-foreground", "{desc}" }
                                } else {
                                    p { class: "text-muted-foreground italic", "No description." }
                                }

                                Button {
                                    variant: ButtonVariant::Destructive,
                                    onclick: move |_| {
                                        let nav = navigator.clone();
                                        spawn(async move {
                                            if delete_item(item_id).await.is_ok() {
                                                nav.push(Route::ItemList {});
                                            }
                                        });
                                    },
                                    "Delete"
                                }
                            }
                        }
                    }
                },
                Some(Ok(None)) => rsx! {
                    p { class: "text-muted-foreground", "Item not found." }
                },
                Some(Err(e)) => rsx! {
                    p { class: "text-destructive text-sm", "Error: {e}" }
                },
                None => rsx! {
                    Skeleton { class: "h-40 w-full" }
                },
            }
        }
    }
}
