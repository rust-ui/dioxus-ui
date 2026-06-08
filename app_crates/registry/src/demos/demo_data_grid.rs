use std::collections::{HashMap, HashSet};
use std::sync::LazyLock;

use dioxus::prelude::*;
use icons::{Copy, Eraser, Plus, Scissors, Settings2, Trash2};
use serde::{Deserialize, Serialize};
use strum::{AsRefStr, Display, EnumIter, IntoEnumIterator};

use crate::hooks::use_copy_clipboard::use_copy_clipboard;
use crate::hooks::use_drag_selection::{UseDragSelection, use_drag_selection};
use crate::hooks::use_press_hold::use_press_hold;
use crate::ui::badge::{Badge, BadgeVariant};
use crate::ui::checkbox::Checkbox;
use crate::ui::data_grid::{
    DataGridColumn, DataGridToolbar, Grid, GridBody, GridCell, GridCellContent, GridCellWrapper, GridHeaderCell,
    GridPinnedCell, GridPinnedHeaderCell, GridRow, GridSelectCell, GridSelectHeaderCell, GridWrapper, PinnableColumn,
    PinnableSortableHeaderCell, SortDirection, SortableColumn, generate_grid_style, get_column_width,
    get_pinned_left_position, get_pinned_visible_columns,
};
use crate::ui::separator::Separator;

/* ========================================================== */
/*                     ✨ TYPES ✨                            */
/* ========================================================== */

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Display, AsRefStr, EnumIter)]
#[strum(serialize_all = "title_case")]
pub enum Column {
    Select,
    Name,
    Age,
    Email,
    Website,
    Notes,
    Salary,
    Department,
    Status,
    Skills,
    IsActive,
    StartDate,
    Attachments,
}

impl DataGridColumn for Column {
    fn colindex(self) -> i32 {
        self as i32 + 1
    }

    fn is_disabled(self) -> bool {
        matches!(self, Self::Select | Self::Name | Self::Email)
    }
}

impl SortableColumn<RowData> for Column {
    fn compare(self, a: &RowData, b: &RowData) -> Option<std::cmp::Ordering> {
        match self {
            Self::Name => Some(a.name.cmp(&b.name)),
            Self::Age => Some(a.age.cmp(&b.age)),
            Self::Email => Some(a.email.cmp(&b.email)),
            Self::Website => Some(a.website.cmp(&b.website)),
            Self::Notes => Some(a.notes.cmp(&b.notes)),
            Self::Salary => Some(a.salary.cmp(&b.salary)),
            Self::Department => Some(a.department.cmp(&b.department)),
            Self::Status => Some(a.status.cmp(&b.status)),
            Self::Skills => Some(a.skills.len().cmp(&b.skills.len())),
            Self::IsActive => Some(a.is_active.cmp(&b.is_active)),
            Self::StartDate => Some(a.start_date.cmp(&b.start_date)),
            Self::Select | Self::Attachments => None,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct RowData {
    pub name: String,
    pub age: i32,
    pub email: String,
    pub website: String,
    pub notes: String,
    pub salary: i32,
    pub department: String,
    pub status: String,
    pub skills: Vec<String>,
    pub is_active: bool,
    pub start_date: String,
}

/// Pinnable columns with their widths in order they should appear when pinned
const PINNABLE_COLUMNS: &[(Column, i32)] = &[
    (Column::Name, 180),
    (Column::Age, 150),
    (Column::Email, 240),
    (Column::Website, 240),
    (Column::Notes, 200),
    (Column::Salary, 180),
    (Column::Department, 180),
    (Column::Status, 180),
    (Column::Skills, 240),
    (Column::IsActive, 110),
    (Column::StartDate, 150),
    (Column::Attachments, 240),
];

impl PinnableColumn for Column {
    fn pinnable_columns() -> &'static [(Self, i32)] {
        PINNABLE_COLUMNS
    }
}

impl Column {
    fn wrapper_class(self) -> &'static str {
        match self {
            Self::IsActive => "flex justify-center",
            _ => "",
        }
    }

    fn get_value(self, row: &RowData) -> String {
        match self {
            Self::Name => row.name.clone(),
            Self::Age => row.age.to_string(),
            Self::Email => row.email.clone(),
            Self::Website => row.website.clone(),
            Self::Notes => row.notes.clone(),
            Self::Salary => row.salary.to_string(),
            Self::Department => row.department.clone(),
            Self::Status => row.status.clone(),
            Self::Skills => row.skills.join(", "),
            Self::IsActive => row.is_active.to_string(),
            Self::StartDate => row.start_date.clone(),
            Self::Select | Self::Attachments => String::new(),
        }
    }
}

/// CSS custom properties for column sizes, generated from PINNABLE_COLUMNS.
static GRID_STYLE: LazyLock<String> = LazyLock::new(generate_grid_style::<Column>);

/* ========================================================== */
/*                     ✨ DATA ✨                             */
/* ========================================================== */

fn initial_rows() -> Vec<RowData> {
    vec![
        RowData {
            name: "Toby Deckow".to_string(),
            age: 50,
            email: "toby.deckow32@yahoo.com".to_string(),
            website: "https://young-plain.net".to_string(),
            notes: "Relocated from the Seattle office last month. Adjusting well to the new team dynamics and company culture.".to_string(),
            salary: 42953,
            department: "Finance".to_string(),
            status: "In Office".to_string(),
            skills: vec!["Docker".to_string(), "JavaScript".to_string(), "SQL".to_string()],
            is_active: true,
            start_date: "12/13/2020".to_string(),
        },
        RowData {
            name: "Montserrat Kutch".to_string(),
            age: 52,
            email: "montserrat_kutch@yahoo.com".to_string(),
            website: "https://major-footrest.net".to_string(),
            notes: "Transferred from the marketing department. Bringing valuable cross-functional experience to the team.".to_string(),
            salary: 64820,
            department: "Marketing".to_string(),
            status: "In Office".to_string(),
            skills: vec!["AWS".to_string(), "Git".to_string(), "SQL".to_string(), "React".to_string()],
            is_active: true,
            start_date: "6/28/2023".to_string(),
        },
        RowData {
            name: "Alice Chen".to_string(),
            age: 34,
            email: "alice.chen@gmail.com".to_string(),
            website: "https://devblog-alice.io".to_string(),
            notes: "Senior developer with expertise in distributed systems. Leading the backend migration project.".to_string(),
            salary: 95000,
            department: "Engineering".to_string(),
            status: "Remote".to_string(),
            skills: vec!["Rust".to_string(), "Kubernetes".to_string(), "PostgreSQL".to_string(), "Go".to_string()],
            is_active: true,
            start_date: "3/15/2019".to_string(),
        },
        RowData {
            name: "James Rodriguez".to_string(),
            age: 28,
            email: "j.rodriguez@company.com".to_string(),
            website: "https://jamesux.design".to_string(),
            notes: "UI/UX designer focused on accessibility and user research. Recently completed design system overhaul.".to_string(),
            salary: 72500,
            department: "Design".to_string(),
            status: "Hybrid".to_string(),
            skills: vec!["Figma".to_string(), "CSS".to_string(), "TypeScript".to_string()],
            is_active: false,
            start_date: "9/01/2022".to_string(),
        },
    ]
}

/* ========================================================== */
/*                     ✨ FUNCTIONS ✨                        */
/* ========================================================== */

#[component]
pub fn DemoDataGrid() -> Element {
    rsx! { DataGridFull {} }
}

/* ========================================================== */
/*                     ✨ FUNCTIONS ✨                        */
/* ========================================================== */

#[component]
pub fn DataGridFull() -> Element {
    let mut rows_signal = use_signal(initial_rows);

    let mut selected_indices_signal = use_signal(HashSet::<usize>::new);

    // Sort signals per column
    let sort_signals: Signal<HashMap<Column, Signal<SortDirection>>> = use_signal(|| {
        PINNABLE_COLUMNS.iter().map(|(col, _)| (*col, Signal::new(SortDirection::None))).collect()
    });

    let mut pinned_columns_signal: Signal<HashSet<Column>> = use_signal(HashSet::new);
    let mut visible_columns_signal: Signal<HashSet<String>> =
        use_signal(|| Column::iter().map(|c| c.to_string()).collect());

    // Cell selection state
    let mut active_cell_signal: Signal<Option<(usize, Column)>> = use_signal(|| None);
    let mut context_menu_cell_signal: Signal<Option<(usize, Column)>> = use_signal(|| None);

    // Drag selection
    let mut drag_selection = use_drag_selection::<Column>();

    // Copy to clipboard
    let (copy_to_clipboard, _) = use_copy_clipboard(None);
    let mut copy_value_signal = use_signal(String::new);

    // Create sorted rows memo
    let sorted_rows_signal = use_memo(move || {
        let mut rows: Vec<RowData> = rows_signal();

        // Apply sorting in order (last one with a value takes precedence)
        let signals = sort_signals.read();
        for (col, _) in PINNABLE_COLUMNS {
            if let Some(signal) = signals.get(col) {
                col.sort_rows(&mut rows, signal());
            }
        }

        rows
    });

    let row_count_signal = use_memo(move || rows_signal.read().len());

    // Dynamic height: (row_count + 1) * 36px to include add row button
    let grid_body_height = use_memo(move || {
        let row_count = row_count_signal();
        format!("height: {}px", (row_count + 1) * 36)
    });

    let selected_count_signal = use_memo(move || selected_indices_signal.read().len());

    let handle_select_all = move |checked: bool| {
        let row_count = row_count_signal();
        selected_indices_signal.with_mut(|selected| {
            if checked {
                for index in 0..row_count {
                    selected.insert(index);
                }
            } else {
                selected.clear();
            }
        });
    };

    // Delete rows handler
    let handle_delete_rows = move |names: Vec<String>| {
        rows_signal.with_mut(|rows| {
            rows.retain(|r| !names.contains(&r.name));
        });
    };

    rsx! {
        div { class: "container flex flex-col py-4",
            ToolbarDataGrid { visible_columns_signal }

            div {
                onmouseup: move |_| drag_selection.stop_dragging(),
                onmouseleave: move |_| drag_selection.stop_dragging(),
                GridWrapper {
                    button {
                        r#type: "button",
                        id: "radix-_R_mlubsqlb_",
                        "aria-haspopup": "menu",
                        "aria-expanded": "false",
                        "data-state": "closed",
                        "data-slot": "dropdown-menu-trigger",
                        style: "position:fixed;left:0px;top:0px;width:1px;height:1px;padding:0;margin:0;border:none;background:transparent;pointer-events:none;opacity:0",
                    }
                    Grid {
                        rowcount: Signal::new(
                            i32::try_from(row_count_signal()).unwrap_or(0) + 1,
                        ),
                        colcount: i32::try_from(Column::iter().count()).unwrap_or(0),
                        style: GRID_STYLE.as_str(),

                        // ------------- GRID HEADER ----------- //
                        GridHeaderDataGrid {
                            row_count_signal: Signal::new(row_count_signal()),
                            selected_count_signal: Signal::new(selected_count_signal()),
                            handle_select_all,
                            sort_signals,
                            pinned_columns_signal,
                            visible_columns_signal,
                        }

                        // ------------- GRID BODY ----------- //
                        GridBody { style: Signal::new(grid_body_height()),
                            for row in sorted_rows_signal() {
                                {
                                    let row_name = row.name.clone();
                                    let index = Signal::new(
                                        sorted_rows_signal()
                                            .iter()
                                            .position(|r| r.name == row_name)
                                            .unwrap_or(0),
                                    );
                                    let rowindex = index() + 2;
                                    let mut is_active_signal = use_signal(|| row.is_active);
                                    let is_selected = selected_indices_signal.read().contains(&index());
                                    let row_clone = row.clone();
                                    let row_for_render = row_clone.clone();

                                    let render_cell_content = move |col: Column| -> Element {
                                        match col {
                                            Column::Website => {
                                                let url = row_for_render.website.clone();
                                                let url_display = url.clone();
                                                rsx! {
                                                    div {
                                                        "data-slot": "grid-cell-content",
                                                        class: "overflow-hidden size-full",
                                                        a {
                                                            href: "{url}",
                                                            target: "_blank",
                                                            rel: "noopener noreferrer",
                                                            class: "underline truncate text-primary decoration-primary/30 underline-offset-2 hover:decoration-primary/60",
                                                            "{url_display}"
                                                        }
                                                    }
                                                }
                                            }
                                            Column::Skills => {
                                                let skills = row_for_render.skills.clone();
                                                rsx! {
                                                    div { class: "flex overflow-hidden flex-wrap gap-1 items-center",
                                                        for skill in skills {
                                                            Badge { variant: BadgeVariant::Secondary, "{skill}" }
                                                        }
                                                    }
                                                }
                                            }
                                            Column::IsActive => {
                                                rsx! {
                                                    Checkbox {
                                                        checked: is_active_signal(),
                                                        on_checked_change: move |checked| is_active_signal.set(checked),
                                                    }
                                                }
                                            }
                                            Column::Select | Column::Attachments => rsx! { div {} },
                                            _ => {
                                                let value = col.get_value(&row_for_render);
                                                rsx! { GridCellContent { "{value}" } }
                                            }
                                        }
                                    };

                                    rsx! {
                                        GridRow { rowindex, index,
                                            GridSelectCell {
                                                div { class: "py-1.5 px-3 size-full",
                                                    Checkbox {
                                                        checked: is_selected,
                                                        on_checked_change: move |checked| {
                                                            let idx = index();
                                                            selected_indices_signal.with_mut(|selected| {
                                                                if checked { selected.insert(idx); } else { selected.remove(&idx); }
                                                            });
                                                        },
                                                    }
                                                }
                                            }
                                            // Pinned cells - only show if both pinned AND visible
                                            for (col, _width) in get_pinned_visible_columns(pinned_columns_signal, visible_columns_signal) {
                                                GridPinnedCell { col, pinned_columns_signal,
                                                    GridCellWrapper { class: col.wrapper_class(),
                                                        {render_cell_content(col)}
                                                    }
                                                }
                                            }
                                            // Non-pinned cells
                                            for (col, _width) in PINNABLE_COLUMNS.iter().copied() {
                                                {
                                                    let is_active = active_cell_signal().map(|(r, c)| r == index() && c == col).unwrap_or(false);
                                                    let is_context = context_menu_cell_signal().map(|(r, c)| r == index() && c == col).unwrap_or(false);
                                                    let in_range = drag_selection.is_cell_in_range(index(), col);
                                                    let is_visible = col.is_visible(pinned_columns_signal, visible_columns_signal);
                                                    let value_for_copy = col.get_value(&row_clone);
                                                    rsx! {
                                                        GridCell {
                                                            colindex: col.colindex(),
                                                            column: col.css_safe_name(),
                                                            visible: is_visible,
                                                            active: is_active,
                                                            current: is_context,
                                                            in_range,
                                                            on_click: move |_| {
                                                                active_cell_signal.set(Some((index(), col)));
                                                                context_menu_cell_signal.set(None);
                                                                drag_selection.clear_selection();
                                                            },
                                                            on_contextmenu: move |_| {
                                                                if drag_selection.handle_contextmenu(index(), col) {
                                                                    active_cell_signal.set(Some((index(), col)));
                                                                }
                                                                context_menu_cell_signal.set(Some((index(), col)));
                                                                copy_value_signal.set(value_for_copy.clone());
                                                            },
                                                            on_mousedown: move |_| {
                                                                active_cell_signal.set(Some((index(), col)));
                                                                drag_selection.start_drag(index(), col);
                                                            },
                                                            on_mouseenter: move |_| {
                                                                drag_selection.update_drag(index(), col);
                                                            },
                                                            GridCellWrapper { class: col.wrapper_class(),
                                                                {render_cell_content(col)}
                                                            }
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                            // Add row button at the bottom
                            GridAddRow { rows_signal, row_count_signal: Signal::new(row_count_signal()) }
                        }
                    }
                }
            }
        }
    }
}

/* ========================================================== */
/*                     ✨ FUNCTIONS ✨                        */
/* ========================================================== */

#[component]
pub fn ToolbarDataGrid(visible_columns_signal: Signal<HashSet<String>>) -> Element {
    rsx! {
        DataGridToolbar { class: "justify-end",
            // Column visibility — simplified toggle button (multi_select not yet ported)
            button {
                r#type: "button",
                class: "px-3 py-2 h-9 inline-flex gap-2 items-center text-sm font-medium rounded-md border bg-background border-input hover:bg-accent hover:text-accent-foreground",
                "data-name": "ToolbarColumnsToggle",
                Settings2 { class: "text-muted-foreground" }
                span { "View" }
            }
        }
    }
}

#[component]
pub fn GridHeaderDataGrid(
    row_count_signal: Signal<usize>,
    selected_count_signal: Signal<usize>,
    handle_select_all: EventHandler<bool>,
    sort_signals: Signal<HashMap<Column, Signal<SortDirection>>>,
    pinned_columns_signal: Signal<HashSet<Column>>,
    visible_columns_signal: Signal<HashSet<String>>,
) -> Element {
    let all_checked = {
        let row_count = row_count_signal();
        row_count > 0 && selected_count_signal() == row_count
    };

    rsx! {
        div { role: "rowgroup", "data-slot": "grid-header", class: "grid sticky top-0 z-10 border-b bg-background",
            div { role: "row", "aria-rowindex": "1", "data-slot": "grid-header-row", tabindex: "-1", class: "flex w-full",
                GridSelectHeaderCell {
                    div { class: "py-1.5 px-3 size-full",
                        Checkbox {
                            checked: all_checked,
                            on_checked_change: move |checked| handle_select_all.call(checked),
                        }
                    }
                }
                // Pinned headers - only show if both pinned AND visible
                for (col, _width) in get_pinned_visible_columns(pinned_columns_signal, visible_columns_signal) {
                    {
                        let sort_signal = sort_signals.read().get(&col).copied();
                        let left = get_pinned_left_position(col, &pinned_columns_signal.read());
                        let width = get_column_width(col);
                        if let Some(sort_signal) = sort_signal {
                            rsx! {
                                GridPinnedHeaderCell { left, width,
                                    PinnableSortableHeaderCell {
                                        column: col,
                                        label: col.to_string(),
                                        sort_signal,
                                        pinned_columns_signal,
                                        visible_columns_signal,
                                        is_pinned: true,
                                    }
                                }
                            }
                        } else {
                            rsx! { div {} }
                        }
                    }
                }
                // Non-pinned headers
                for (col, _width) in PINNABLE_COLUMNS.iter().copied() {
                    {
                        let sort_signal = sort_signals.read().get(&col).copied();
                        let is_visible = col.is_visible(pinned_columns_signal, visible_columns_signal);
                        if let Some(sort_signal) = sort_signal {
                            rsx! {
                                GridHeaderCell {
                                    colindex: col.colindex(),
                                    column: col.css_safe_name(),
                                    visible: is_visible,
                                    PinnableSortableHeaderCell {
                                        column: col,
                                        label: col.to_string(),
                                        sort_signal,
                                        pinned_columns_signal,
                                        visible_columns_signal,
                                    }
                                }
                            }
                        } else {
                            rsx! { div {} }
                        }
                    }
                }
            }
        }
    }
}

#[component]
fn PressHoldDeleteRow(
    index: Signal<usize>,
    mut drag_selection: UseDragSelection<Column>,
    sorted_rows_signal: Signal<Vec<RowData>>,
    handle_delete_rows: EventHandler<Vec<String>>,
    mut active_cell_signal: Signal<Option<(usize, Column)>>,
    mut context_menu_cell_signal: Signal<Option<(usize, Column)>>,
) -> Element {
    let disabled = use_signal(|| false);

    let on_delete = Callback::new(move |_: ()| {
        let idx = index();
        let (min_row, max_row) =
            drag_selection.get_selection_bounds().map(|(min, max, _, _)| (min, max)).unwrap_or((idx, idx));

        let sorted = sorted_rows_signal();
        let names_to_delete: Vec<String> =
            (min_row..=max_row).filter_map(|i| sorted.get(i).map(|r| r.name.clone())).collect();

        handle_delete_rows.call(names_to_delete);
        drag_selection.clear_selection();
        active_cell_signal.set(None);
        context_menu_cell_signal.set(None);
    });

    let press_hold = use_press_hold(1500, on_delete, false);

    let ph1 = press_hold.clone();
    let ph2 = press_hold.clone();
    let ph3 = press_hold.clone();
    let ph4 = press_hold.clone();

    let progress_style = move || {
        let width_percent = (press_hold.progress_signal)() * 100.0;
        format!(
            "position: absolute; left: 0; top: 0; bottom: 0; width: {width_percent:.1}%; background: rgba(239, 68, 68, 0.3); pointer-events: none; border-radius: inherit;"
        )
    };

    let is_multi_row =
        move || drag_selection.get_selection_bounds().is_some_and(|(min_row, max_row, _, _)| max_row > min_row);

    rsx! {
        button {
            class: "flex relative gap-2 items-center py-1.5 px-2 w-full text-sm rounded-sm transition-colors select-none text-destructive hover:bg-destructive/10",
            onpointerdown: move |_| ph1.on_pointer_down(),
            onpointerup: move |_| ph2.on_pointer_up(),
            onpointerleave: move |_| ph3.on_pointer_up(),
            onpointercancel: move |_| ph4.on_pointer_up(),
            span { style: "{progress_style()}" }
            span { class: "flex relative z-10 gap-2 items-center",
                Trash2 { class: "size-4" }
                if is_multi_row() { "Hold to delete rows" } else { "Hold to delete row" }
            }
        }
    }
}

/* ========================================================== */
/*                     ✨ FUNCTIONS ✨                        */
/* ========================================================== */

/// Add row button at the bottom of the grid using absolute positioning.
#[component]
fn GridAddRow(mut rows_signal: Signal<Vec<RowData>>, row_count_signal: Signal<usize>) -> Element {
    let rowindex = row_count_signal() + 2;
    let translate_y = row_count_signal() * 36;

    rsx! {
        div {
            role: "row",
            "aria-rowindex": "{rowindex}",
            "data-slot": "grid-add-row",
            tabindex: "-1",
            class: "flex absolute w-full border-b transition-colors cursor-pointer bg-muted/30 hover:bg-muted/50",
            style: "height: 36px; transform: translateY({translate_y}px);",
            onclick: move |_| {
                rows_signal.with_mut(|rows| {
                    let new_row_num = rows.len() + 1;
                    let new_name = format!("New Row {new_row_num}");
                    rows.push(RowData { name: new_name, ..Default::default() });
                });
            },
            div { class: "flex sticky left-0 gap-2 items-center px-3 text-muted-foreground",
                Plus { class: "size-3.5" }
                span { class: "text-sm", "Add row" }
            }
        }
    }
}
