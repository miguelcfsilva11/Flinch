struct Table {
    // Table fields
}

struct Row {
    // Row fields
}

struct Cell {
    // Cell fields
}

impl Table {
    fn new() -> Self {
        Table {
            // Initialize table fields
        }
    }

    fn add_row(&mut self, row: Row) {
        // Logic to add a row to the table
    }
}

impl Row {
    fn new() -> Self {
        Row {
            // Initialize row fields
        }
    }

    fn add_cell(&mut self, cell: Cell) {
        // Logic to add a cell to the row
    }
}

impl Cell {
    fn new() -> Self {
        Cell {
            // Initialize cell fields
        }
    }
}


fn main() {
    let mut table = Table::new();
    let mut row = Row::new();
    let cell = Cell::new();

    row.add_cell(cell);
    table.add_row(row);

    // Further logic to work with the table
}