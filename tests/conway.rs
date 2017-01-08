// # Conway's Game of Life: Tests
//
// - You can start off with no cells
// - You can add cells
// - You can check if a cell is alive or dead
// - Your cells follow the rules when moving to the next generation
//

extern crate conway;
use conway::*;

#[test]
fn new_is_empty() {
    let c = Conway::new(1);
    let expected = "OOOOOOOOOOOOOOOOOOOO";
    assert_eq!(expected, c.to_string());
}

#[test]
fn adding_a_cell() {
    let mut c = Conway::new(1);
    c.add_living(&Location::new(0, 0));
    let expected = "XOOOOOOOOOOOOOOOOOOO";
    assert_eq!(expected, c.to_string());
}

#[test]
fn empty_worlds_stay_empty() {
    let c = Conway::new(1);
    let c = c.tick();
    let expected = "OOOOOOOOOOOOOOOOOOOO";
    assert_eq!(expected, c.to_string());
}

#[test]
fn cell_with_no_neigbors_dies() {
    let mut c = Conway::new(1);
    c.add_living(&Location::new(2, 0));
    let expected = "OOXOOOOOOOOOOOOOOOOO";
    assert_eq!(expected, c.to_string());
    let c = c.tick();
    let expected = "OOOOOOOOOOOOOOOOOOOO";
    assert_eq!(expected, c.to_string());
}

#[test]
fn cell_with_one_neighbor_dies() {
    let mut c = Conway::new(1);
    c.add_living(&Location::new(2, 0));
    c.add_living(&Location::new(3, 0));
    let expected = "OOXXOOOOOOOOOOOOOOOO";
    assert_eq!(expected, c.to_string());
    let c = c.tick();
    let expected = "OOOOOOOOOOOOOOOOOOOO";
    assert_eq!(expected, c.to_string());
}


#[test]
fn cell_with_two_neighbors_lives() {
    let mut c = Conway::new(1);
    c.add_living(&Location::new(2, 0));
    c.add_living(&Location::new(3, 0));
    c.add_living(&Location::new(4, 0));
    let expected = "OOXXXOOOOOOOOOOOOOOO";
    assert_eq!(expected, c.to_string());
    let c = c.tick();
    let expected = "OOOXOOOOOOOOOOOOOOOO";
    assert_eq!(expected, c.to_string());
}

#[test]
fn conway_can_be_created_with_more_than_one_row() {
    let c = Conway::new(2);
    let expected = "OOOOOOOOOOOOOOOOOOOO\n".to_string() + "OOOOOOOOOOOOOOOOOOOO";
    assert_eq!(expected, c.to_string());
}

#[test]
#[cfg_attr(rustfmt, rustfmt_skip)]
fn stable_block() {
    let mut c = Conway::new(4);
    c.add_living(&Location::new(2, 1));
    c.add_living(&Location::new(3, 1));
    c.add_living(&Location::new(2, 2));
    c.add_living(&Location::new(3, 2));
    let expected = "OOOOOOOOOOOOOOOOOOOO\n".to_string() +
                   "OOXXOOOOOOOOOOOOOOOO\n" +
                   "OOXXOOOOOOOOOOOOOOOO\n" +
                   "OOOOOOOOOOOOOOOOOOOO";
    assert_eq!(expected, c.to_string());
    let c = c.tick();
    assert_eq!(expected, c.to_string());
}

#[test]
#[cfg_attr(rustfmt, rustfmt_skip)]
fn stable_beehive() {
    let mut c = Conway::new(5);
    c.add_living(&Location::new(2, 1));
    c.add_living(&Location::new(3, 1));
    c.add_living(&Location::new(1, 2));
    c.add_living(&Location::new(4, 2));
    c.add_living(&Location::new(2, 3));
    c.add_living(&Location::new(3, 3));
    let expected = "OOOOOOOOOOOOOOOOOOOO\n".to_string() +
                   "OOXXOOOOOOOOOOOOOOOO\n" +
                   "OXOOXOOOOOOOOOOOOOOO\n" +
                   "OOXXOOOOOOOOOOOOOOOO\n" +
                   "OOOOOOOOOOOOOOOOOOOO";
    assert_eq!(expected, c.to_string());
    let c = c.tick();
    assert_eq!(expected, c.to_string());
}

#[test]
#[cfg_attr(rustfmt, rustfmt_skip)]
fn stable_loaf() {
    let mut c = Conway::new(6);
    c.add_living(&Location::new(2, 1));
    c.add_living(&Location::new(3, 1));
    c.add_living(&Location::new(1, 2));
    c.add_living(&Location::new(4, 2));
    c.add_living(&Location::new(2, 3));
    c.add_living(&Location::new(4, 3));
    c.add_living(&Location::new(3, 4));
    let expected = "OOOOOOOOOOOOOOOOOOOO\n".to_string() +
                   "OOXXOOOOOOOOOOOOOOOO\n" +
                   "OXOOXOOOOOOOOOOOOOOO\n" +
                   "OOXOXOOOOOOOOOOOOOOO\n" +
                   "OOOXOOOOOOOOOOOOOOOO\n" +
                   "OOOOOOOOOOOOOOOOOOOO";
    assert_eq!(expected, c.to_string());
    let c = c.tick();
    assert_eq!(expected, c.to_string());
}

#[test]
#[cfg_attr(rustfmt, rustfmt_skip)]
fn oscillating_blinker() {
    let mut c = Conway::new(5);
    c.add_living(&Location::new(2, 1));
    c.add_living(&Location::new(2, 2));
    c.add_living(&Location::new(2, 3));
    let period_one = "OOOOOOOOOOOOOOOOOOOO\n".to_string() +
                     "OOXOOOOOOOOOOOOOOOOO\n" +
                     "OOXOOOOOOOOOOOOOOOOO\n" +
                     "OOXOOOOOOOOOOOOOOOOO\n" +
                     "OOOOOOOOOOOOOOOOOOOO";
    assert_eq!(period_one, c.to_string());
    let c = c.tick();
    let period_two = "OOOOOOOOOOOOOOOOOOOO\n".to_string() +
                     "OOOOOOOOOOOOOOOOOOOO\n" +
                     "OXXXOOOOOOOOOOOOOOOO\n" +
                     "OOOOOOOOOOOOOOOOOOOO\n" +
                     "OOOOOOOOOOOOOOOOOOOO";
    assert_eq!(period_two, c.to_string());
    let c = c.tick();
    assert_eq!(period_one, c.to_string());
}

#[test]
#[cfg_attr(rustfmt, rustfmt_skip)]
fn oscillating_toad() {
    let mut c = Conway::new(6);
    c.add_living(&Location::new(2, 2));
    c.add_living(&Location::new(3, 2));
    c.add_living(&Location::new(4, 2));
    c.add_living(&Location::new(1, 3));
    c.add_living(&Location::new(2, 3));
    c.add_living(&Location::new(3, 3));
    let period_one = "OOOOOOOOOOOOOOOOOOOO\n".to_string() +
                     "OOOOOOOOOOOOOOOOOOOO\n" +
                     "OOXXXOOOOOOOOOOOOOOO\n" +
                     "OXXXOOOOOOOOOOOOOOOO\n" +
                     "OOOOOOOOOOOOOOOOOOOO\n" +
                     "OOOOOOOOOOOOOOOOOOOO";
    assert_eq!(period_one, c.to_string());
    let period_two = "OOOOOOOOOOOOOOOOOOOO\n".to_string() +
                     "OOOXOOOOOOOOOOOOOOOO\n" +
                     "OXOOXOOOOOOOOOOOOOOO\n" +
                     "OXOOXOOOOOOOOOOOOOOO\n" +
                     "OOXOOOOOOOOOOOOOOOOO\n" +
                     "OOOOOOOOOOOOOOOOOOOO";
    let c = c.tick();
    assert_eq!(period_two, c.to_string());
    let c = c.tick();
    assert_eq!(period_one, c.to_string());
}
