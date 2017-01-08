pub struct Conway {
    world: World,
}

impl Conway {
    pub fn new(row_count: usize) -> Self {
        Conway { world: World::new(20, row_count) }
    }

    pub fn to_string(&self) -> String {
        let mut r = String::from("");

        let mut row = 0;

        for location in self.world.locations.iter() {
            if location.y != row {
                row += 1;
                r.push('\n');
            }

            if let Some(cell) = self.world.cell_at(&location) {
                r.push(cell.state());
            }
        }
        r
    }

    pub fn add_living(&mut self, l: &Location) {
        self.world.add_cell(Cell::alive(), l);
    }

    pub fn tick(&self) -> Conway {
        let mut new_conway = Conway::new(self.world.height);

        for location in self.world.locations.iter() {
            if let Some(cell) = self.world.cell_at(&location) {
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
        self.world
            .neighbors_of(location)
            .into_iter()
            .filter(|&c| c.living)
            .collect::<Vec<&Cell>>()
            .len()
    }
}

#[derive(Debug)]
pub struct Location {
    x: usize,
    y: usize,
}

impl Location {
    pub fn new(x: usize, y: usize) -> Self {
        Location { x: x, y: y }
    }

    fn neighbors(&self) -> Vec<Location> {
        let mut n = Vec::new();

        if self.x > 0 && self.y > 0 {
            n.push(Location::new(self.x - 1, self.y - 1));
        }

        if self.x > 0 {
            n.push(Location::new(self.x - 1, self.y));
            n.push(Location::new(self.x - 1, self.y + 1));
        }

        if self.y > 0 {
            n.push(Location::new(self.x, self.y - 1));
            n.push(Location::new(self.x + 1, self.y - 1));
        }

        n.push(Location::new(self.x + 1, self.y));
        n.push(Location::new(self.x, self.y + 1));
        n.push(Location::new(self.x + 1, self.y + 1));
        n
    }
}

#[derive(Debug, Clone)]
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

struct World {
    height: usize,
    state: Vec<Vec<Cell>>,
    locations: Vec<Location>,
}

impl World {
    fn new(width: usize, height: usize) -> Self {
        let mut w = World {
            height: height.clone(),
            state: vec![vec![Cell::dead(); width]; height],
            locations: Vec::with_capacity(height * width),
        };

        for y_index in 0..height {
            for x_index in 0..width {
                let l = Location::new(x_index, y_index);
                w.add_cell(Cell::dead(), &l);
                w.add_location(l);
            }
        }

        w
    }

    fn add_cell(&mut self, cell: Cell, location: &Location) {
        self.state[location.y][location.x] = cell;
    }

    fn add_location(&mut self, location: Location) {
        self.locations.push(location)
    }

    fn cell_at(&self, location: &Location) -> Option<&Cell> {
        if let Some(_) = self.state.get(location.y) {
            if let Some(cell) = self.state[location.y].get(location.x) {
                Some(&cell)
            } else {
                None
            }
        } else {
            None
        }
    }

    fn neighbors_of(&self, location: &Location) -> Vec<&Cell> {
        location.neighbors()
            .iter()
            .filter_map(|neighbor| self.cell_at(neighbor))
            .collect::<Vec<&Cell>>()
    }
}
