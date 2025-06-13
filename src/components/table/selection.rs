#[derive(Debug, Clone, Copy, PartialEq)]
pub enum DragState {
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

#[derive(Debug, Default, Clone, PartialEq)]
pub struct Selection {
    cell_rect: Option<IndexRect>,
    rows: Option<IndexRange>,
    pub columns: Option<IndexRange>,
    pub state: DragState,
}

impl Selection {
    pub fn start_cell_selection(&mut self, row: usize, column: usize) {
        self.clear();
        self.cell_rect = Some(IndexRect::new(
            IndexRange::new(row, row),
            IndexRange::new(column, column),
        ));
    }

    pub fn update_cell_selection(&mut self, row: usize, column: usize) {
        if let Some(rect) = &mut self.cell_rect {
            rect.row.end = row;
            rect.column.end = column;
        } else {
            self.start_cell_selection(row, column);
        }
    }

    pub fn start_row_selection(&mut self, row: usize) {
        self.clear();
        self.rows = Some(IndexRange::new(row, row));
        self.cell_rect = Some(IndexRect::new(
            IndexRange::new(row, row),
            IndexRange::new(0, usize::MAX),
        ));
    }

    pub fn update_row_selection(&mut self, row: usize) {
        if let Some(range) = &mut self.rows {
            range.end = row;
            if let Some(rect) = &mut self.cell_rect {
                rect.row.end = row;
            }
        } else {
            self.start_row_selection(row);
        }
    }

    pub fn start_column_selection(&mut self, column: usize) {
        self.clear();
        self.columns = Some(IndexRange::new(column, column));
        self.cell_rect = Some(IndexRect::new(
            IndexRange::new(0, usize::MAX),
            IndexRange::new(column, column),
        ));
    }

    pub fn update_column_selection(&mut self, column: usize) {
        if let Some(range) = &mut self.columns {
            range.end = column;
            if let Some(rect) = &mut self.cell_rect {
                rect.column.end = column;
            }
        } else {
            self.start_column_selection(column);
        }
    }

    pub fn shift_column_selection_to(&mut self, to: usize) {
        if let Some(range) = &mut self.columns {
            let length = range.length();
            range.start = to;
            range.end = to + length;

            self.cell_rect = Some(IndexRect::new(
                self.cell_rect.unwrap_or_default().row,
                IndexRange::new(to, to + length),
            ))
        }
    }

    pub fn is_cell_selected(&self, row: usize, column: usize) -> bool {
        if let Some(rect) = self.cell_rect {
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
        self.cell_rect = None;
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

    pub fn length(&self) -> usize {
        let sorted = self.sorted();
        return sorted.end - sorted.start;
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

    pub fn contains(&self, row: usize, column: usize) -> bool {
        self.row.contains(row) && self.column.contains(column)
    }
}
