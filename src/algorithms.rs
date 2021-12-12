use rand::{thread_rng, Rng};

use crate::cell;
use crate::grid;
use crate::hash_grid;

pub fn binary_tree_cell(some_cell: &mut cell::Cell) -> &mut cell::Cell {
    let mut neighbors: Vec<(i32, i32)> = vec![];
    if some_cell.north.is_some() {
        neighbors.push(some_cell.north.expect("should be present"));
    }
    if some_cell.east.is_some() {
        neighbors.push(some_cell.east.expect("This shouldn't happen"));
    }
    if !neighbors.is_empty() {
        let mut rng = thread_rng();
        let pick = rng.gen_range(0..neighbors.len());
        let coords = neighbors[pick];
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
        let mut inner: Vec<cell::Cell> = Vec::new();
        let mut run_count = 1i32;
        let mut rng = thread_rng();

        for (col_num, cll) in row.iter().enumerate() {
            inner.push(cll.clone());
            let at_northern_boundary: bool = cll.north.is_none();
            let should_close_out =
                cll.east.is_none() || (!at_northern_boundary && rng.gen_range(0..2) == 0);

            if should_close_out {
                let idx = if run_count == 1 {
                    col_num as usize
                } else {
                    rng.gen_range(col_num + 1 - (run_count as usize)..col_num + 1)
                };
                run_count = 1;

                let member = &inner[idx];
                if member.north.is_some() {
                    let north_cell = member.north.expect("should be present");
                    let mut new_member = member.clone();
                    new_member.link(north_cell);
                    inner[idx] = new_member;
                }
            } else {
                let east_cell = cll.east.expect("should be some");
                let mut new_cll = cll.clone();
                new_cll.link(east_cell);
                inner[col_num as usize] = new_cll;
                run_count += 1;
            }
        }
        outer.push(inner);
    }
    grid::Grid::from_cells(outer)
}

// These algorithms only work on HashGrids: it was too hard to get them working with others
pub fn aldous_broder(hgrid: &mut hash_grid::HashGrid) -> &mut hash_grid::HashGrid {
    let mut unvisited = hgrid.len() - 1;
    let mut cll = hgrid.random_cell().unwrap().clone();
    while unvisited > 0 {
        let neighbor = cll.random_neighbor().unwrap();
        let mut ncell = hgrid.get_item((neighbor.0, neighbor.1)).unwrap().clone();
        if !ncell.has_links() {
            cll.link((ncell.row, ncell.column));
            ncell.link((cll.row, cll.column));
            let _ = hgrid.replace_cell(cll);
            let _ = hgrid.replace_cell(ncell.clone());
            unvisited -= 1;
        }
        cll = ncell;
    }

    hgrid
}
