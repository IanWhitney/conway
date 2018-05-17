pub trait CellLike {
    fn tick(&self, n: u8) -> Cell;
}

#[derive(Debug)]
pub enum Cell {
    LiveCell(LiveCell),
    DeadCell(DeadCell),
}

impl CellLike for Cell {
    fn tick(&self, n: u8) ->  Cell {
        match *self {
            Cell::LiveCell(ref live_cell) => live_cell.tick(n),
            Cell::DeadCell(ref dead_cell) => dead_cell.tick(n),
        }
    }
}

#[derive(Debug)]
pub struct LiveCell {}

#[derive(Debug)]
pub struct DeadCell {}

impl CellLike for LiveCell {
    fn tick(&self, n: u8) -> Cell {
        if n > 4 {
            Cell::living()
        } else {
            Cell::dead()
        }
    }
}

impl CellLike for DeadCell {
    fn tick(&self, n: u8) -> Cell {
        if n > 4 {
            Cell::living()
        } else {
            Cell::dead()
        }
    }
}

impl Cell {
    pub fn living() -> Self {
        Cell::LiveCell(LiveCell {})
    }

    pub fn dead() -> Self {
        Cell::DeadCell(DeadCell {})
    }
}

fn main() {
    let live = Cell::living();
    let dead = Cell::dead();
    
    live.tick(4);
    dead.tick(4);
}