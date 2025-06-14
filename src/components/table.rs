use std::ops::RangeBounds;

use dioxus::{
    desktop::tao::{
        event::{ElementState, RawKeyEvent},
        keyboard::KeyCode as TaoKeyCode,
    },
    html::input_data::MouseButton,
    prelude::*,
};

use crate::user::User;
use crate::utils::use_key_event;
use dioxus_primitives::context_menu::{ContextMenu, ContextMenuContent, ContextMenuItem};

use selection::{DragState, IndexRange, Selection};

mod selection;

#[component]
pub fn Table(users: Vec<User>) -> Element {
    let group_columns = use_context_provider(|| Signal::new(get_group_columns(&users)));
    let rows = use_signal(|| get_rows(users));
    let mut selection = use_context_provider(|| Signal::new(Selection::default()));
    let mut context_menu_state = use_context_provider(ContextMenuState::default);

    use_key_event(move |event: &RawKeyEvent| {
        if event.physical_key == TaoKeyCode::Escape && event.state == ElementState::Released {
            selection.write().clear();
            context_menu_state.open.set(false);
        }
    });

    rsx! {
        ContextMenu { open: (context_menu_state.open)(),
            ContextMenuContent {
                class: "z-1000 p-3 bg-white border rounded shadow-lg",
                left: format!("{}px", context_menu_state.position.read().0),
                top: format!("{}px", context_menu_state.position.read().1),
                ContextMenuItem {
                    index: 0usize,
                    value: "0",
                    on_select: |value: String| {
                        println!("Selected: {}", value);
                    },
                    "Move element"
                }
            }
        }
        div { class: "border rounded overscroll-none w-full h-full max-w-fit max-h-fit overflow-auto",
            table {
                class: "whitespace-nowrap border-separate border-spacing-0",
                onmouseup: move |_| {
                    selection.write().state = DragState::Idle;
                },
                oncontextmenu: move |evt| {
                    evt.prevent_default();
                    context_menu_state.position.set(evt.client_coordinates().to_tuple());
                    context_menu_state.open.set(true);
                },
                thead {
                    tr {
                        th { class: "sticky left-0 top-0 bg-white z-3 text-left align-bottom px-1 border-r border-b",
                            "Name"
                        }
                        for (column_index , _) in group_columns.read().iter().enumerate() {
                            GroupHeader { column_index }
                        }
                    }
                }
                tbody {
                    for (row_index , row) in rows.read().iter().enumerate() {
                        tr {
                            RowHeader { row_index, "{row.user.name}" }
                            for (column_index , column) in group_columns.read().iter().enumerate() {
                                GroupCell {
                                    row_index,
                                    column_index,
                                    value: column.access(row),
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

#[component]
fn RowHeader(row_index: usize, children: Element) -> Element {
    let mut selection = use_context::<Signal<Selection>>();
    let selected = use_memo(move || selection().is_row_selected(row_index));

    rsx! {
        th {
            class: "h-8 text-left px-1 border-r border-b sticky left-0 z-1",
            class: if selected() { "bg-gray-200" } else { "bg-white" },
            onmousedown: move |evt: MouseEvent| {
                if let Some(MouseButton::Primary) = evt.trigger_button() {
                    evt.prevent_default();
                    let mut selection = selection.write();
                    selection.start_row_selection(row_index);
                    selection.state = DragState::Row;
                }
            },
            onmouseenter: move |_| {
                if selection().state == DragState::Row {
                    selection.write().update_row_selection(row_index);
                }
            },
            {children}
        }
    }
}

#[component]
fn GroupHeader(column_index: usize) -> Element {
    let mut group_columns: Signal<Vec<GroupColumn>> = use_context();
    let mut selection: Signal<Selection> = use_context();
    let selected = use_memo(move || selection().is_column_selected(column_index));

    rsx! {
        th {
            class: "wm-sideways-lr border-r w-8 border-b sticky z-1 top-0 px-1 text-left",
            class: if selected() { "bg-gray-200" } else { "bg-white" },
            onmousedown: move |evt: MouseEvent| {
                if let Some(MouseButton::Primary) = evt.trigger_button() {
                    evt.prevent_default();
                    let mut selection = selection.write();
                    selection.start_column_selection(column_index);
                    selection.state = DragState::Column;
                }
            },
            onmouseenter: move |_| {
                if selection().state == DragState::Column {
                    selection.write().update_column_selection(column_index);
                }
            },
            oncontextmenu: move |evt: MouseEvent| {
                evt.prevent_default();
                evt.stop_propagation();
                let mut selection = selection.write();
                if let Some(range) = selection.columns {
                    let IndexRange { start, end } = range.sorted();
                    if range.contains(column_index) {
                        return;
                    }
                    let insertion_index = if column_index > end {
                        column_index - range.length()
                    } else {
                        column_index
                    };
                    let mut columns = group_columns.write();
                    reorder_elements(&mut columns, start..=end, insertion_index);
                    selection.shift_column_selection_to(insertion_index);
                }
            },
            "{group_columns.read()[column_index].id}"
        }
    }
}

#[component]
fn GroupCell(row_index: usize, column_index: usize, value: bool) -> Element {
    let mut selection = use_context::<Signal<Selection>>();
    let selected = use_memo(move || selection().is_cell_selected(row_index, column_index));

    rsx! {
        td {
            class: "min-w-8 h-8 border-r border-b",
            class: if value && selected() { "bg-gray-500" },
            class: if value { "bg-gray-400" },
            class: if selected() { "bg-gray-200" },
            onmousedown: move |evt: MouseEvent| {
                if let Some(MouseButton::Primary) = evt.trigger_button() {
                    evt.prevent_default();
                    let mut selection = selection.write();
                    selection.start_cell_selection(row_index, column_index);
                    selection.state = DragState::Cell;
                }
            },
            onmouseenter: move |_| {
                if selection().state == DragState::Cell {
                    selection.write().update_cell_selection(row_index, column_index);
                }
            },
        }
    }
}

#[derive(Clone, Debug, Default)]
struct ContextMenuState {
    open: Signal<bool>,
    position: Signal<(f64, f64)>,
}

fn get_rows(users: Vec<User>) -> Vec<Row> {
    users
        .into_iter()
        .map(|u| Row {
            id: u.id.clone(),
            user: u,
        })
        .collect()
}

fn get_group_columns(users: &[User]) -> Vec<GroupColumn> {
    if users.is_empty() {
        return vec![];
    }
    let groups = users[0].groups.keys().cloned().collect::<Vec<String>>();
    groups
        .into_iter()
        .map(|g| GroupColumn::new(g.clone(), g.clone()))
        .collect()
}

fn reorder_elements<T, R: RangeBounds<usize>>(vec: &mut Vec<T>, range: R, insertion_index: usize) {
    let elements: Vec<T> = vec.drain(range).collect();
    vec.splice(insertion_index..insertion_index, elements);
}

#[allow(unused)]
#[derive(Debug, Clone)]
pub struct Row {
    pub id: String,
    pub user: User,
}

#[allow(unused)]
pub struct PropertyColumn {
    pub id: String,
    pub property: String,
}

#[allow(unused)]
impl PropertyColumn {
    pub fn new(id: String, property: String) -> Self {
        Self { id, property }
    }

    pub fn access(&self, row: &Row) -> String {
        row.user
            .properties
            .get(&self.id)
            .cloned()
            .unwrap_or_default()
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct GroupColumn {
    pub id: String,
    pub group: String,
}

impl GroupColumn {
    pub fn new(id: String, group: String) -> Self {
        Self { id, group }
    }
    pub fn access(&self, row: &Row) -> bool {
        row.user.groups.get(&self.id).cloned().unwrap_or_default()
    }
}
