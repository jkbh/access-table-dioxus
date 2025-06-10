use dioxus::{html::input_data::MouseButton, logger::tracing, prelude::*};
use dioxus_test::{create_mock_users, GroupColumn, Row, User};
use std::collections::HashSet;

fn main() {
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    let users = create_mock_users(50, 200);

    rsx! {
        document::Stylesheet { href: asset!("/assets/tailwind.css") }
        Table { users }
    }
}

#[derive(Clone, Copy, PartialEq, Default, Debug)]
struct IndexRange {
    start: usize,
    end: usize,
}

impl IndexRange {
    fn length(&self) -> usize {
        let sorted = self.sorted();
        sorted.end - sorted.start
    }

    fn sorted(&self) -> IndexRange {
        IndexRange {
            start: self.start.min(self.end),
            end: self.start.max(self.end),
        }
    }
}

#[derive(Clone, Copy, PartialEq, Default, Debug)]
struct IndexRect {
    row: IndexRange,
    column: IndexRange,
}

impl IndexRect {
    fn sorted(&self) -> Self {
        Self {
            row: self.row.sorted(),
            column: self.column.sorted(),
        }
    }
}

#[derive(PartialEq, Clone, Copy, Default)]
struct RangeSelectionState {
    is_dragging: bool,
    cells: Option<IndexRect>,
}

#[derive(Clone, PartialEq, Default)]
struct SelectionStore {
    rows: HashSet<usize>,
    columns: HashSet<usize>,
    cells: HashSet<(usize, usize)>,
    range: Option<IndexRect>,
}

impl SelectionStore {
    fn new() -> Self {
        Self::default()
    }

    fn clear(&mut self) {
        self.rows.clear();
        self.columns.clear();
        self.cells.clear();
        self.range = None;
    }

    fn set_range_selection(&mut self, rect: IndexRect) {
        self.clear();
        self.range = Some(rect);
    }

    fn add_row(&mut self, idx: usize) {
        self.range = None;
        self.rows.insert(idx);
    }

    fn add_column(&mut self, idx: usize) {
        self.range = None;
        self.columns.insert(idx);
    }

    fn add_cell(&mut self, row: usize, column: usize) {
        self.range = None;
        self.cells.insert((row, column));
    }

    fn is_cell_selected(&self, row_idx: usize, col_idx: usize) -> bool {
        if let Some(range) = self.range {
            let sorted = range.sorted();
            row_idx >= sorted.row.start
                && row_idx <= sorted.row.end
                && col_idx >= sorted.column.start
                && col_idx <= sorted.column.end
        } else {
            false
        }
    }

    fn is_row_selected(&self, idx: usize) -> bool {
        self.rows.contains(&idx)
    }

    fn is_column_selected(&self, idx: usize) -> bool {
        self.columns.contains(&idx)
    }
}

#[component]
fn Table(users: Vec<User>) -> Element {
    let rows = use_signal(|| {
        users
            .clone()
            .into_iter()
            .map(|u| Row {
                id: u.id.clone(),
                user: u,
            })
            .collect::<Vec<_>>()
    });

    let group_columns = use_context_provider(move || {
        let groups = users[0].groups.keys().cloned().collect::<Vec<String>>();
        Signal::new(
            groups
                .into_iter()
                .map(|g| GroupColumn::new(g.clone(), g.clone()))
                .collect::<Vec<GroupColumn>>(),
        )
    });

    let mut selection_store = use_context_provider(|| Signal::new(SelectionStore::new()));
    let mut range_selection_state =
        use_context_provider(|| Signal::new(RangeSelectionState::default()));

    rsx! {
        table {
            class: "whitespace-nowrap border-separate border-spacing-0",
            onmouseup: move |_| {
                range_selection_state.write().is_dragging = false;
            },
            thead {
                tr {
                    th { class: "sticky left-0 top-0 bg-white z-3 text-left align-bottom px-1 border-r border-b",
                        "Name"
                    }
                    for (column_index , group_column) in group_columns.read().iter().enumerate() {
                        GroupHeader {
                            column_index,
                            selected: selection_store.read().is_column_selected(column_index),
                        }
                    }
                }
            }
            tbody {
                for (row_index , row) in rows.read().iter().enumerate() {
                    tr {
                        RowHeader {
                            row_index,
                            selected: selection_store.read().is_row_selected(row_index),
                            "{row.user.name}"
                        }
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
fn RowHeader(row_index: usize, selected: bool, children: Element) -> Element {
    let mut selection_store = use_context::<Signal<SelectionStore>>();

    rsx! {
        th {
            class: "h-8 text-left px-1 border-r border-b sticky left-0 z-1",
            class: if selected { "bg-gray-200" } else { "bg-white" },
            onclick: move |_| {
                selection_store.write().clear();
                selection_store.write().add_row(row_index);
            },
            {children}
        }
    }
}

#[component]
fn GroupHeader(column_index: usize, selected: bool) -> Element {
    let mut selection_store = use_context::<Signal<SelectionStore>>();
    let group_columns = use_context::<Signal<Vec<GroupColumn>>>();

    rsx! {
        th {
            class: "wm-sideways-lr border-r w-8 border-b sticky z-1 top-0 px-1",
            class: if selected { "bg-gray-200" } else { "bg-white" },
            onclick: move |_| {
                selection_store.write().clear();
                selection_store.write().add_column(column_index);
            },
            "{group_columns.read()[column_index].id}"
        }
    }
}

#[component]
fn GroupCell(row_index: usize, column_index: usize, value: bool) -> Element {
    let mut selection_store = use_context::<Signal<SelectionStore>>();
    let mut range_selection = use_context::<Signal<RangeSelectionState>>();

    let selected = use_memo(move || {
        selection_store
            .read()
            .is_cell_selected(row_index, column_index)
    });

    rsx! {
        td {
            class: "min-w-8 h-8 border-r border-b",
            class: if value && selected() { "bg-gray-500" },
            class: if value { "bg-gray-400" },
            class: if selected() { "bg-gray-200" },
            onmousedown: move |evt: MouseEvent| {
                if let Some(MouseButton::Primary) = evt.trigger_button() {
                    let rect = IndexRect {
                        row: IndexRange {
                            start: row_index,
                            end: row_index,
                        },
                        column: IndexRange {
                            start: column_index,
                            end: column_index,
                        },
                    };
                    selection_store.write().set_range_selection(rect);
                    range_selection.write().is_dragging = true;
                }
            },
            onmouseenter: move |_| {
                if range_selection.read().is_dragging {
                    let new_rect = if let Some(current_range) = selection_store.read().range {
                        Some(IndexRect {
                            row: IndexRange {
                                start: current_range.row.start,
                                end: row_index,
                            },
                            column: IndexRange {
                                start: current_range.column.start,
                                end: column_index,
                            },
                        })
                    } else {
                        None
                    };
                    if let Some(new_rect) = new_rect {
                        selection_store.write().set_range_selection(new_rect);
                    }
                }
            },
        }
    }
}
