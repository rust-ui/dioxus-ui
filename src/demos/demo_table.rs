use dioxus::prelude::*;

use crate::ui::badge::{Badge, BadgeVariant};
use crate::ui::table::{Table, TableBody, TableCaption, TableCell, TableHead, TableHeader, TableRow};

#[component]
pub fn DemoTable() -> Element {
    rsx! {
        Table {
            TableCaption { "A list of recent invoices." }
            TableHeader {
                TableRow {
                    TableHead { "Invoice" }
                    TableHead { "Status" }
                    TableHead { "Method" }
                    TableHead { class: "text-right", "Amount" }
                }
            }
            TableBody {
                TableRow {
                    TableCell { class: "font-medium", "INV001" }
                    TableCell { Badge { variant: BadgeVariant::Success, "Paid" } }
                    TableCell { "Credit Card" }
                    TableCell { class: "text-right", "$250.00" }
                }
                TableRow {
                    TableCell { class: "font-medium", "INV002" }
                    TableCell { Badge { variant: BadgeVariant::Outline, "Pending" } }
                    TableCell { "PayPal" }
                    TableCell { class: "text-right", "$150.00" }
                }
                TableRow {
                    TableCell { class: "font-medium", "INV003" }
                    TableCell { Badge { variant: BadgeVariant::Destructive, "Overdue" } }
                    TableCell { "Bank Transfer" }
                    TableCell { class: "text-right", "$350.00" }
                }
                TableRow {
                    TableCell { class: "font-medium", "INV004" }
                    TableCell { Badge { variant: BadgeVariant::Success, "Paid" } }
                    TableCell { "Credit Card" }
                    TableCell { class: "text-right", "$450.00" }
                }
            }
        }
    }
}
