use std::collections::HashSet;

use dioxus::prelude::*;

use crate::ui::multi_select::{
    MultiSelect, MultiSelectContent, MultiSelectGroup, MultiSelectLabel, MultiSelectOption, MultiSelectTrigger,
    MultiSelectValue,
};

#[component]
pub fn DemoMultiSelectScrollable() -> Element {
    let selected_signal = use_signal(HashSet::<String>::new);

    rsx! {
        MultiSelect { values: selected_signal,
            MultiSelectTrigger { class: "w-[280px]",
                MultiSelectValue { placeholder: "Select timezones" }
            }

            MultiSelectContent { class: "w-[280px]",
                // North America
                MultiSelectGroup {
                    MultiSelectLabel { "North America" }
                    {[
                        ("est", "Eastern Standard Time (EST)"),
                        ("cst", "Central Standard Time (CST)"),
                        ("mst", "Mountain Standard Time (MST)"),
                        ("pst", "Pacific Standard Time (PST)"),
                        ("akst", "Alaska Standard Time (AKST)"),
                        ("hst", "Hawaii Standard Time (HST)"),
                    ].into_iter().map(|(value, label)| {
                        rsx! {
                            MultiSelectOption { value: value,
                                {label}
                            }
                        }
                    }).collect::<Vec<_>>()}
                }

                // Europe & Africa
                MultiSelectGroup {
                    MultiSelectLabel { "Europe & Africa" }
                    {[
                        ("gmt", "Greenwich Mean Time (GMT)"),
                        ("cet", "Central European Time (CET)"),
                        ("eet", "Eastern European Time (EET)"),
                        ("west", "Western European Summer Time (WEST)"),
                        ("cat", "Central Africa Time (CAT)"),
                        ("eat", "East Africa Time (EAT)"),
                    ].into_iter().map(|(value, label)| {
                        rsx! {
                            MultiSelectOption { value: value,
                                {label}
                            }
                        }
                    }).collect::<Vec<_>>()}
                }

                // Asia
                MultiSelectGroup {
                    MultiSelectLabel { "Asia" }
                    {[
                        ("msk", "Moscow Time (MSK)"),
                        ("ist", "India Standard Time (IST)"),
                        ("cst_china", "China Standard Time (CST)"),
                        ("jst", "Japan Standard Time (JST)"),
                        ("kst", "Korea Standard Time (KST)"),
                        ("wita", "Indonesia Central Standard Time (WITA)"),
                    ].into_iter().map(|(value, label)| {
                        rsx! {
                            MultiSelectOption { value: value,
                                {label}
                            }
                        }
                    }).collect::<Vec<_>>()}
                }

                // Australia & Pacific
                MultiSelectGroup {
                    MultiSelectLabel { "Australia & Pacific" }
                    {[
                        ("awst", "Australian Western Standard Time (AWST)"),
                        ("acst", "Australian Central Standard Time (ACST)"),
                        ("aest", "Australian Eastern Standard Time (AEST)"),
                        ("nzst", "New Zealand Standard Time (NZST)"),
                        ("fjt", "Fiji Time (FJT)"),
                    ].into_iter().map(|(value, label)| {
                        rsx! {
                            MultiSelectOption { value: value,
                                {label}
                            }
                        }
                    }).collect::<Vec<_>>()}
                }

                // South America
                MultiSelectGroup {
                    MultiSelectLabel { "South America" }
                    {[
                        ("art", "Argentina Time (ART)"),
                        ("bot", "Bolivia Time (BOT)"),
                        ("brt", "Brasilia Time (BRT)"),
                        ("clt", "Chile Standard Time (CLT)"),
                    ].into_iter().map(|(value, label)| {
                        rsx! {
                            MultiSelectOption { value: value,
                                {label}
                            }
                        }
                    }).collect::<Vec<_>>()}
                }
            }
        }
    }
}
