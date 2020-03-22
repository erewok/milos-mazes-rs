use crate::cell;
use crate::grid;
use rand::prelude::*;
use rand::seq::SliceRandom;

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
        let coords = (cll.row, cll.column);
        some_cell.link(coords);
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


pub fn sidewinder(some_grid: &grid::Grid) -> grid::Grid {
    let mut outer: Vec<Vec<cell::Cell>> = Vec::new();
    for row in some_grid.each_row() {
        let mut run: Vec<cell::Cell> = Vec::new();
        let mut inner: Vec<cell::Cell> = Vec::new();

        for mut cll in row.clone() {
            run.push(cll.clone());
            let mut rng = thread_rng();
            let at_eastern_boundary: bool = cll.column == some_grid.columns;
            let at_northern_boundary: bool = cll.row == 0;
            let should_close_out = at_eastern_boundary || (!at_northern_boundary && rng.gen_range(0, 2) == 0);

            if should_close_out {
                let member = run.choose(&mut rng);
                match member {
                    Some(member_cll) => if member_cll.north.is_some() {
                        let north_cell = member_cll.north.as_ref().expect("should be present");
                        let mut new_member = member_cll.clone();
                        new_member.link((north_cell.row, north_cell.column));
                        inner.push(new_member)
                        },
                    _ => {}
                }
            } else {
                if cll.east.is_some() {
                    let east_cell = cll.east.as_ref().expect("should be some");
                    cll.link((east_cell.row, east_cell.column));
                }
                inner.push(cll.clone())
            }
        }
        outer.push(inner);
    }
    grid::Grid::from_cells(outer)
}