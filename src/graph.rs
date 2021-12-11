use crate::cell;
use crate::grid;
use crate::render;
use petgraph::graph::{NodeIndex, UnGraph};
use petgraph::visit::IntoNodeIdentifiers;
use raqote;
use std::collections::HashMap;

pub fn from_grid(some_grid: grid::Grid) -> UnGraph<(i32, i32), ()> {
    let size = (some_grid.columns * some_grid.rows) as usize;
    let mut graph = UnGraph::with_capacity(size, size * 4);
    let mut node_point_map: HashMap<NodeIndex, (i32, i32)> = HashMap::new();
    let mut point_node_reverse_map: HashMap<(i32, i32), NodeIndex> = HashMap::new();
    for cell in some_grid.iter() {
        // this is pretty wasteful, but :shrugs:
        let cell_node = graph.add_node((cell.row, cell.column));
        node_point_map.insert(cell_node.clone(), (cell.row, cell.column));
        point_node_reverse_map.insert((cell.row, cell.column), cell_node.clone());
    }
    for (node_index, coords) in node_point_map.into_iter() {
        if let Some(cell) = some_grid.get_item(coords) {
            for neighbor in cell.neighbors().iter() {
                let neighbor_direction = cell
                    .neighbor_direction(**neighbor)
                    .map(|dir| cell.direction_has_link(dir))
                    .unwrap_or(false);
                if let Some(neighbor_node) = point_node_reverse_map.get(*neighbor) {
                    if neighbor_direction && !graph.contains_edge(node_index, *neighbor_node) {
                        graph.add_edge(node_index, *neighbor_node, ());
                    }
                }
            }
        }
    }
    graph
}

pub fn base_from_coords(rows: i32, columns: i32) -> UnGraph<(i32, i32), ()> {
    let size = (columns * rows) as usize;
    let mut graph = UnGraph::with_capacity(size, size * 4);
    for row in 0..rows {
        for column in 0..columns {
            graph.add_node((row, column));
        }
    }
    graph
}

pub fn to_png(
    graph: &UnGraph<(i32, i32), ()>,
    cell_size: i32,
    columns: i32,
    rows: i32,
    filename: &str,
) -> Result<(), String> {
    let img_width = cell_size * columns;
    let img_height = cell_size * rows;
    let mut dt = raqote::DrawTarget::new(img_width + cell_size * 2, img_height + cell_size * 2);

    for node in graph.node_identifiers() {
        let weight = graph.node_weight(node); // could also use the index: graph[node]
        if let Some((rownum, colnum)) = weight {
            let mut new_cell = cell::Cell::new(*rownum, *colnum);
            let neighbor_iter = graph.neighbors(node);
            for neighbor in neighbor_iter {
                if let Some(neighbor_weight) = graph.node_weight(neighbor) {
                    new_cell.add_neighbor(*neighbor_weight);
                }
            }
            render::draw_cell(&mut dt, cell_size, new_cell);
        }
    }
    dt.write_png(filename)
        .map_err(|err| format!("Failed writing file {}", err))
}

#[cfg(test)]
mod test {
    use super::{base_from_coords, from_grid};
    use crate::grid::Grid;
    use petgraph::dot::{Config, Dot};
    #[test]
    fn test_from_coords() {}
    #[test]
    fn test_from_grid() {
        let mut new_grid = Grid::new(2, 2);
        let mut c1 = new_grid.get_item((0, 0)).unwrap().clone();
        c1.link((0, 1));
        new_grid.replace_cell(c1);
        let mut c2 = new_grid.get_item((0, 1)).unwrap().clone();
        c2.link((1, 1));
        new_grid.replace_cell(c2);
        let mut c3 = new_grid.get_item((1, 1)).unwrap().clone();
        c3.link((1, 0));
        new_grid.replace_cell(c3);
        let graph = from_grid(new_grid);
        assert_eq!(graph.edge_count(), 3);
    }
}
