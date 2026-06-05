use dioxus::prelude::*;

use crate::ui::select_native::{OverlappingLabel, SelectNative};

const TARGET_ID: &str = "select-native-timezone";

#[component]
pub fn DemoSelectNativeTimezone() -> Element {
    rsx! {
        div { class: "relative group min-w-[300px]",
            OverlappingLabel { html_for: TARGET_ID, label: "Select with native timezone" }
            SelectNative { id: TARGET_ID,
                option { value: "Pacific/Midway", "(GMT-11) Pacific/Midway" }
                option { value: "Pacific/Niue", "(GMT-11) Pacific/Niue" }
                option { value: "Pacific/Pago_Pago", "(GMT-11) Pacific/Pago Pago" }
                option { value: "Pacific/Honolulu", "(GMT-10) Pacific/Honolulu" }
                option { value: "Pacific/Rarotonga", "(GMT-10) Pacific/Rarotonga" }
                option { value: "Pacific/Tahiti", "(GMT-10) Pacific/Tahiti" }
                option { value: "America/Adak", "(GMT-9) America/Adak" }
                option { value: "Pacific/Gambier", "(GMT-9) Pacific/Gambier" }
                option { value: "Pacific/Marquesas", "(GMT-9:30) Pacific/Marquesas" }
                option { value: "America/Anchorage", "(GMT-8) America/Anchorage" }
                option { value: "America/Juneau", "(GMT-8) America/Juneau" }
                option { value: "America/Metlakatla", "(GMT-8) America/Metlakatla" }
                option { value: "America/Nome", "(GMT-8) America/Nome" }
                option { value: "America/Sitka", "(GMT-8) America/Sitka" }
                option { value: "America/Yakutat", "(GMT-8) America/Yakutat" }
                option { value: "Pacific/Pitcairn", "(GMT-8) Pacific/Pitcairn" }
                option { value: "America/Creston", "(GMT-7) America/Creston" }
                option { value: "America/Dawson", "(GMT-7) America/Dawson" }
                option { value: "America/Dawson_Creek", "(GMT-7) America/Dawson Creek" }
                option { value: "America/Fort_Nelson", "(GMT-7) America/Fort Nelson" }
                option { value: "America/Hermosillo", "(GMT-7) America/Hermosillo" }
                option { value: "America/Los_Angeles", "(GMT-7) America/Los Angeles" }
                option { value: "America/Mazatlan", "(GMT-7) America/Mazatlan" }
                option { value: "America/Phoenix", "(GMT-7) America/Phoenix" }
                option { value: "America/Tijuana", "(GMT-7) America/Tijuana" }
                option { value: "America/Vancouver", "(GMT-7) America/Vancouver" }
                option { value: "America/Whitehorse", "(GMT-7) America/Whitehorse" }
                option { value: "America/Bahia_Banderas", "(GMT-6) America/Bahia Banderas" }
                option { value: "America/Belize", "(GMT-6) America/Belize" }
                option { value: "America/Boise", "(GMT-6) America/Boise" }
                option { value: "America/Cambridge_Bay", "(GMT-6) America/Cambridge Bay" }
                option { value: "America/Chicago", "(GMT-5) America/Chicago" }
                option { value: "America/New_York", "(GMT-4) America/New York" }
                option { value: "America/Toronto", "(GMT-4) America/Toronto" }
                option { value: "America/Sao_Paulo", "(GMT-3) America/Sao Paulo" }
                option { value: "America/Buenos_Aires", "(GMT-3) America/Buenos Aires" }
                option { value: "Atlantic/South_Georgia", "(GMT-2) Atlantic/South Georgia" }
                option { value: "Atlantic/Azores", "(GMT+0) Atlantic/Azores" }
                option { value: "Europe/London", selected: true, "(GMT+1) Europe/London" }
                option { value: "Europe/Paris", "(GMT+2) Europe/Paris" }
                option { value: "Europe/Berlin", "(GMT+2) Europe/Berlin" }
                option { value: "Europe/Moscow", "(GMT+3) Europe/Moscow" }
                option { value: "Asia/Tehran", "(GMT+3:30) Asia/Tehran" }
                option { value: "Asia/Dubai", "(GMT+4) Asia/Dubai" }
                option { value: "Asia/Kabul", "(GMT+4:30) Asia/Kabul" }
                option { value: "Asia/Karachi", "(GMT+5) Asia/Karachi" }
                option { value: "Asia/Calcutta", "(GMT+5:30) Asia/Calcutta" }
                option { value: "Asia/Katmandu", "(GMT+5:45) Asia/Katmandu" }
                option { value: "Asia/Dhaka", "(GMT+6) Asia/Dhaka" }
                option { value: "Asia/Rangoon", "(GMT+6:30) Asia/Rangoon" }
                option { value: "Asia/Bangkok", "(GMT+7) Asia/Bangkok" }
                option { value: "Asia/Shanghai", "(GMT+8) Asia/Shanghai" }
                option { value: "Asia/Singapore", "(GMT+8) Asia/Singapore" }
                option { value: "Asia/Tokyo", "(GMT+9) Asia/Tokyo" }
                option { value: "Australia/Sydney", "(GMT+10) Australia/Sydney" }
                option { value: "Pacific/Auckland", "(GMT+12) Pacific/Auckland" }
            }
        }
    }
}
