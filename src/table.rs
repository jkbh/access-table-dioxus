use dioxus::{html::input_data::MouseButton, prelude::*};

use dioxus_primitives::context_menu::{
    ContextMenu, ContextMenuContent, ContextMenuItem, ContextMenuTrigger,
};

use crate::{GroupColumn, Row, User};

#[derive(Clone, Debug, Default)]
struct ContextMenuState {
    open: Signal<bool>,
    position: Signal<(f64, f64)>,
}

#[component]
pub fn Table(users: Vec<User>) -> Element {
    let group_columns = use_context_provider(|| Signal::new(get_group_columns(&users)));
    let rows = use_signal(|| get_rows(users));
    let mut selection = use_context_provider(|| Signal::new(Selection::default()));
    let mut context_menu_state = use_context_provider(|| ContextMenuState::default());

    rsx! {
        ContextMenu { open: *context_menu_state.open.read(),
            ContextMenuContent {
                class: "z-1000 p-3 bg-white border rounded shadow-lg",
                left: format!("{}px", context_menu_state.position.read().0),
                top: format!("{}px", context_menu_state.position.read().1),
                ContextMenuItem {
                    index: 0usize,
                    value: "0",
                    on_select: |value: String| {},
                    "Move element"
                }
            }
        }
        table {
            class: "whitespace-nowrap border-separate border-spacing-0",
            onmouseup: move |_| {
                selection.write().state = DragState::Idle;
            },
            onkeydown: move |evt: KeyboardEvent| {
                if evt.key() == Key::Escape {
                    selection.write().clear();
                }
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
    let group_columns = use_context::<Signal<Vec<GroupColumn>>>();
    let mut selection = use_context::<Signal<Selection>>();
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

#[derive(Clone, Copy, PartialEq)]
enum DragState {
    Idle,
    Cell,
    Row,
    Column,
}

impl Default for DragState {
    fn default() -> Self {
        DragState::Idle
    }
}

#[derive(Default, Clone)]
struct Selection {
    cells: Option<IndexRect>,
    rows: Option<IndexRange>,
    columns: Option<IndexRange>,
    state: DragState,
}

impl Selection {
    pub fn start_cell_selection(&mut self, row: usize, column: usize) {
        self.clear();
        self.cells = Some(IndexRect::new(
            IndexRange::new(row, row),
            IndexRange::new(column, column),
        ));
    }

    pub fn update_cell_selection(&mut self, row: usize, column: usize) {
        if let Some(rect) = &mut self.cells {
            rect.row.end = row;
            rect.column.end = column;
        } else {
            self.start_cell_selection(row, column);
        }
    }

    pub fn start_row_selection(&mut self, row: usize) {
        self.clear();
        self.rows = Some(IndexRange::new(row, row));
        self.cells = Some(IndexRect::new(
            IndexRange::new(row, row),
            IndexRange::new(0, usize::MAX),
        ));
    }

    pub fn update_row_selection(&mut self, row: usize) {
        if let Some(range) = &mut self.rows {
            range.end = row;
            if let Some(rect) = &mut self.cells {
                rect.row.end = row;
            }
        } else {
            self.start_row_selection(row);
        }
    }

    pub fn start_column_selection(&mut self, column: usize) {
        self.clear();
        self.columns = Some(IndexRange::new(column, column));
        self.cells = Some(IndexRect::new(
            IndexRange::new(0, usize::MAX),
            IndexRange::new(column, column),
        ));
    }

    pub fn update_column_selection(&mut self, column: usize) {
        if let Some(range) = &mut self.columns {
            range.end = column;
            if let Some(rect) = &mut self.cells {
                rect.column.end = column;
            }
        } else {
            self.start_column_selection(column);
        }
    }

    pub fn is_cell_selected(&self, row: usize, column: usize) -> bool {
        if let Some(rect) = self.cells {
            rect.contains(row, column)
        } else {
            false
        }
    }

    pub fn is_row_selected(&self, row: usize) -> bool {
        if let Some(range) = self.rows {
            range.contains(row)
        } else {
            false
        }
    }

    pub fn is_column_selected(&self, column: usize) -> bool {
        if let Some(range) = self.columns {
            range.contains(column)
        } else {
            false
        }
    }

    pub fn clear(&mut self) {
        self.cells = None;
        self.rows = None;
        self.columns = None;
        self.state = DragState::Idle;
    }
}

#[derive(Clone, Copy, PartialEq, Default, Debug)]
pub struct IndexRange {
    pub start: usize,
    pub end: usize,
}

impl IndexRange {
    pub fn new(start: usize, end: usize) -> Self {
        Self { start, end }
    }

    pub fn sorted(&self) -> IndexRange {
        IndexRange {
            start: self.start.min(self.end),
            end: self.start.max(self.end),
        }
    }

    pub fn contains(&self, index: usize) -> bool {
        let sorted = self.sorted();
        index >= sorted.start && index <= sorted.end
    }
}

#[derive(Clone, Copy, PartialEq, Default, Debug)]
pub struct IndexRect {
    pub row: IndexRange,
    pub column: IndexRange,
}

impl IndexRect {
    pub fn new(row: IndexRange, column: IndexRange) -> Self {
        Self { row, column }
    }

    pub fn sorted(&self) -> Self {
        Self {
            row: self.row.sorted(),
            column: self.column.sorted(),
        }
    }

    pub fn contains(&self, row: usize, column: usize) -> bool {
        self.row.contains(row) && self.column.contains(column)
    }
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
