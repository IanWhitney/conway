pub struct Conway {
    rows: Vec<Vec<char>>,
}

impl Conway {
    pub fn new(row_count: usize) -> Self {
        Conway { rows: vec![vec!['O'; 20]; row_count] }
    }

    pub fn state(&self) -> String {
        let mut r = String::from("");

        for row in self.rows.iter() {
            for c in row.iter().cloned() {
                r.push(c);
            }
            r.push('\n');
        }
        r.pop();
        r
    }

    pub fn add_living(&mut self, l: &Location) {
        self.rows[l.y][l.x] = 'X'
    }

    pub fn tick(&self) -> Conway {
        let mut new_conway = Conway::new(self.rows.len());

        for (y_index, row) in self.rows.iter().enumerate() {
            for (x_index, cell) in row.iter().enumerate() {
                let location = Location::new(x_index, y_index);
                let living_cell = *cell == 'X';
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
            if x >= 1 && self.rows[y_index][x - 1] == 'X' {
                count += 1;
            }

            if x <= 18 && self.rows[y_index][x + 1] == 'X' {
                count += 1;
            }

            if y_index != y && self.rows[y_index][x] == 'X' {
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
