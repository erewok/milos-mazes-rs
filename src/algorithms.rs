use std::collections::HashSet;
use petgraph::graph::{UnGraph, NodeIndex};
use rand::{thread_rng, Rng};
use rand::seq::SliceRandom;

use crate::cell;
use crate::grid;

pub fn binary_tree_cell(some_cell: &mut cell::Cell) -> &mut cell::Cell {
    let mut neighbors: Vec<(i32, i32)> = vec![];
    if some_cell.north.is_some() {
        neighbors.push(some_cell.north.expect("should be present"));
    }
    if some_cell.east.is_some() {
        neighbors.push(some_cell.east.expect("This shouldn't happen"));
    }
    if neighbors.len() > 0 {
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
        let mut col_num = 0i32;
        let mut run_count = 1i32;
        let mut rng = thread_rng();


        for cll in row {
            inner.push(cll.clone());
            let at_eastern_boundary: bool = !cll.east.is_some();
            let at_northern_boundary: bool = !cll.north.is_some();
            let should_close_out = at_eastern_boundary || (!at_northern_boundary && rng.gen_range(0..2) == 0);

            if should_close_out {
                let idx = if run_count == 1 {
                    col_num as usize
                } else {
                    rng.gen_range(col_num + 1 - run_count..col_num + 1) as usize
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
            col_num += 1;

        }
        outer.push(inner);
    }
    grid::Grid::from_cells(outer)
}

// These algorithms only work on graphs: it was too hard to get them working on our custom
// data structure. The original cell/graph data structure _could_ work with association
// matrices or arrays that operate on copyable (u32, u32) or something
pub fn aldous_broder(graph: &mut UnGraph<(i32, i32), ()>) -> &mut UnGraph<(i32, i32), ()> {
    let all_node_indices: Vec<NodeIndex> = graph.node_indices().map(|ni| ni.clone()).collect();
    let mut visited: HashSet<NodeIndex> = HashSet::new();

    while visited.len() < all_node_indices.len() {
        let mut node: Option<&NodeIndex> = all_node_indices.choose(&mut rand::thread_rng());
        if let Some(nd) = node {
            if !visited.contains(nd) {
                visited.insert(nd.clone());
                let weight = graph.node_weight(*nd);
            }

            // this doesn't work because neighbors means they're already connected. We have _no_ edges yet
            // let neighbors: Vec<&NodeIndex> = graph.neighbors(*nd).collect();
            // let mut neighbor = neighbors.choose(&mut rand::thread_rng());
        }
    }

    // while unvisited > 0 {
    //     let neighbors = cll.neighbors();
    //     let neighbor_cnt = neighbors.len();
    //     let pick = rng.gen_range(0..neighbor_cnt);
    //     let neighbor = some_grid.get_item(*neighbors[pick]);
    //     if neighbor.is_some() && neighbor.expect("should be present").links.is_empty() {
    //         let mut new_cll = cll.clone();
    //         new_cll.link(neighbor.coords());
    //         some_grid.replace_cell(new_cll.to_owned());
    //         unvisited -= 1;
    //     }
    //     cll = *neighbor.clone();

    // }

    graph
}