#[derive(Debug, Clone, PartialEq)]
pub enum SelectionType {
    Cells(CellRange),
    Rows(IndexRange),
    Columns(IndexRange),
}

#[derive(Debug, Clone, PartialEq, Default)]
pub enum DragState {
    #[default]
    Idle,
    Dragging(SelectionType),
}

#[derive(Debug, Default, Clone, PartialEq)]
pub struct Selection {
    pub current: Option<SelectionType>,
    pub drag_state: DragState,
}

impl Selection {
    pub fn start_cell_selection(&mut self, row: usize, column: usize) {
        let range = CellRange::new(row, column, row, column);
        self.current = Some(SelectionType::Cells(range));
        self.drag_state = DragState::Dragging(SelectionType::Cells(range));
    }

    pub fn update_cell_selection(&mut self, row: usize, column: usize) {
        if let Some(SelectionType::Cells(range)) = &mut self.current {
            range.end_row = row;
            range.end_column = column;
            if let DragState::Dragging(SelectionType::Cells(drag_range)) = &mut self.drag_state {
                *drag_range = *range;
            }
        }
    }

    pub fn start_row_selection(&mut self, row: usize) {
        let range = IndexRange::new(row, row);
        self.current = Some(SelectionType::Rows(range));
        self.drag_state = DragState::Dragging(SelectionType::Rows(range));
    }

    pub fn update_row_selection(&mut self, row: usize) {
        if let Some(SelectionType::Rows(range)) = &mut self.current {
            range.end = row;
            if let DragState::Dragging(SelectionType::Rows(drag_range)) = &mut self.drag_state {
                *drag_range = *range;
            }
        }
    }

    pub fn start_column_selection(&mut self, column: usize) {
        let range = IndexRange::new(column, column);
        self.current = Some(SelectionType::Columns(range));
        self.drag_state = DragState::Dragging(SelectionType::Columns(range));
    }

    pub fn update_column_selection(&mut self, column: usize) {
        if let Some(SelectionType::Columns(range)) = &mut self.current {
            range.end = column;
            if let DragState::Dragging(SelectionType::Columns(drag_range)) = &mut self.drag_state {
                *drag_range = *range;
            }
        }
    }

    pub fn finish_drag(&mut self) {
        self.drag_state = DragState::Idle;
    }

    pub fn get_column_range(&self) -> Option<IndexRange> {
        match &self.current {
            Some(SelectionType::Columns(range)) => Some(*range),
            _ => None,
        }
    }

    pub fn shift_column_selection_to(&mut self, to: usize) {
        if let Some(SelectionType::Columns(range)) = &mut self.current {
            let length = range.length();
            range.start = to;
            range.end = to + length - 1;
        }
    }

    pub fn is_cell_selected(&self, row: usize, column: usize) -> bool {
        match &self.current {
            Some(SelectionType::Cells(range)) => range.contains(row, column),
            Some(SelectionType::Rows(range)) => range.contains(row),
            Some(SelectionType::Columns(range)) => range.contains(column),
            None => false,
        }
    }

    pub fn is_row_selected(&self, row: usize) -> bool {
        matches!(&self.current, Some(SelectionType::Rows(range)) if range.contains(row))
    }

    pub fn is_column_selected(&self, column: usize) -> bool {
        matches!(&self.current, Some(SelectionType::Columns(range)) if range.contains(column))
    }

    pub fn is_dragging(&self) -> bool {
        !matches!(self.drag_state, DragState::Idle)
    }

    pub fn is_dragging_rows(&self) -> bool {
        matches!(self.drag_state, DragState::Dragging(SelectionType::Rows(_)))
    }

    pub fn is_dragging_columns(&self) -> bool {
        matches!(self.drag_state, DragState::Dragging(SelectionType::Columns(_)))
    }

    pub fn is_dragging_cells(&self) -> bool {
        matches!(self.drag_state, DragState::Dragging(SelectionType::Cells(_)))
    }

    pub fn clear(&mut self) {
        self.current = None;
        self.drag_state = DragState::Idle;
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
        sorted.end - sorted.start + 1
    }

    pub fn contains(&self, index: usize) -> bool {
        let sorted = self.sorted();
        index >= sorted.start && index <= sorted.end
    }
}

#[derive(Clone, Copy, PartialEq, Default, Debug)]
pub struct CellRange {
    pub start_row: usize,
    pub start_column: usize,
    pub end_row: usize,
    pub end_column: usize,
}

impl CellRange {
    pub fn new(start_row: usize, start_column: usize, end_row: usize, end_column: usize) -> Self {
        Self { start_row, start_column, end_row, end_column }
    }

    pub fn sorted(&self) -> CellRange {
        CellRange {
            start_row: self.start_row.min(self.end_row),
            end_row: self.start_row.max(self.end_row),
            start_column: self.start_column.min(self.end_column),
            end_column: self.start_column.max(self.end_column),
        }
    }

    pub fn contains(&self, row: usize, column: usize) -> bool {
        let sorted = self.sorted();
        row >= sorted.start_row && row <= sorted.end_row &&
        column >= sorted.start_column && column <= sorted.end_column
    }
}
