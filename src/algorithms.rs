use crate::cell;
use crate::grid;
use rand::prelude::*;

pub fn binary_tree_cell(some_cell: &mut cell::Cell) -> &mut cell::Cell {
    let mut neighbors: Vec<&Box<cell::Cell>> = vec![];
    if some_cell.north.is_some() {
        neighbors.push(some_cell.north.as_ref().expect("This shouldn't happen"));
    }
    if some_cell.east.is_some() {
        neighbors.push(some_cell.east.as_ref().expect("This shouldn't happen"));
    }
    if neighbors.len() > 0 {
        let mut rng = thread_rng();
        let pick = rng.gen_range(0, neighbors.len());
        let cll = neighbors[pick];
        let released = move || *cll.clone();
        some_cell.link(&mut released(), true);
    }
    some_cell
}

pub fn binary_tree(some_grid: &grid::Grid) -> grid::Grid {
    let mut outer: Vec<Vec<cell::Cell>> = Vec::new();
    for row in some_grid.each_row() {
        let mut inner: Vec<cell::Cell> = Vec::new();
        for mut cll in row.clone() {
            let new_cell = binary_tree_cell(&mut cll);
            inner.push(new_cell.clone());
        }
        outer.push(inner);
    }
    grid::Grid::from_cells(outer)
}