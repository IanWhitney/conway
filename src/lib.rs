pub struct Conway {
    rows: Vec<Row>,
}

impl Conway {
    pub fn new(row_count: usize) -> Self {
        let mut rows: Vec<Row> = Vec::new();

        for _ in 0..row_count {
            let mut cell_row = Row::new();
            for _ in 0..20 {
                cell_row.add(Cell::dead());
            }
            rows.push(cell_row);
        }

        Conway { rows: rows }
    }

    pub fn state(&self) -> String {
        let mut s: Vec<String> = Vec::new();

        for row in self.rows.iter() {
            s.push(row.state());
        }

        s.join("\n")
    }

    pub fn add_living(&mut self, l: &Location) {
        self.rows[l.y].replace(l.x, Cell::alive()).unwrap();
    }

    pub fn tick(&self) -> Conway {
        let mut new_conway = Conway::new(self.rows.len());

        for (y_index, row) in self.rows.iter().enumerate() {
            for (x_index, cell) in row.cells().enumerate() {
                let location = Location::new(x_index, y_index);
                let living_cell = cell.living;
                let living_neighbors = self.living_neighbor_count(&location);

                if living_cell && (living_neighbors == 2 || living_neighbors == 3) {
                    new_conway.add_living(&location);
                }

                if !living_cell && living_neighbors == 3 {
                    new_conway.add_living(&location);
                }
            }
        }
        new_conway
    }

    fn living_neighbor_count(&self, location: &Location) -> usize {
        let y = location.y;
        let x = location.x;

        let low_y = if y < 1 { 0 } else { y - 1 };
        let high_y = if y == (self.rows.len() - 1) {
            self.rows.len()
        } else {
            y + 2
        };
        let mut count = 0;

        for y_index in low_y..high_y {
            if x >= 1 && self.rows[y_index].cell_at(x - 1).living {
                count += 1;
            }

            if x <= 18 && self.rows[y_index].cell_at(x + 1).living {
                count += 1;
            }

            if y_index != y && self.rows[y_index].cell_at(x).living {
                count += 1;
            }

        }
        count
    }
}

pub struct Location {
    x: usize,
    y: usize,
}

impl Location {
    pub fn new(x: usize, y: usize) -> Self {
        Location { x: x, y: y }
    }
}

struct Cell {
    living: bool,
}

impl Cell {
    pub fn dead() -> Self {
        Cell { living: false }
    }

    pub fn alive() -> Self {
        Cell { living: true }
    }

    pub fn state(&self) -> char {
        if self.living { 'X' } else { 'O' }
    }
}

struct Row {
    cells: Vec<Cell>,
}

impl Row {
    fn new() -> Self {
        Row { cells: Vec::new() }
    }

    fn add(&mut self, cell: Cell) {
        self.cells.push(cell)
    }

    fn replace(&mut self, column: usize, new_cell: Cell) -> Result<(), ()> {
        if let Some(_) = self.cells.get(column) {
            self.cells[column] = new_cell;
            Ok(())
        } else {
            Err(())
        }
    }

    fn cell_at<'a>(&'a self, column: usize) -> &'a Cell {
        &self.cells[column]
    }

    fn state(&self) -> String {
        let mut s = String::new();
        for cell in self.cells.iter() {
            s.push(cell.state());
        }
        s
    }

    fn cells(&self) -> std::slice::Iter<Cell> {
        self.cells.iter()
    }
}
