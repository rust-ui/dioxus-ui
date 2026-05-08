use dioxus::prelude::*;
use icons::{House, ScrollText, Settings};

use crate::app::Route;
use crate::components::ui::bottom_nav::{BottomNav, BottomNavButton, BottomNavGrid, BottomNavLabel};
use crate::domain::home::routes::HomeRoutes;
use crate::domain::item::routing::ItemRoutes;
use crate::domain::settings::routes::SettingsRoutes;

#[component]
pub fn AppBottomNav() -> Element {
    let route: Route = use_route();
    let navigator = use_navigator();

    rsx! {
        BottomNav {
            BottomNavGrid {
                BottomNavButton {
                    active: matches!(route, Route::Home {}),
                    onclick: move |_| { navigator.push(Route::Home {}); },
                    House { class: "size-5" }
                    BottomNavLabel { {HomeRoutes::LABEL} }
                }
                BottomNavButton {
                    active: matches!(route, Route::ItemList {}),
                    onclick: move |_| { navigator.push(Route::ItemList {}); },
                    ScrollText { class: "size-5" }
                    BottomNavLabel { {ItemRoutes::LABEL} }
                }
                BottomNavButton {
                    active: matches!(route, Route::Settings {}),
                    onclick: move |_| { navigator.push(Route::Settings {}); },
                    Settings { class: "size-5" }
                    BottomNavLabel { {SettingsRoutes::LABEL} }
                }
            }
        }
    }
}
