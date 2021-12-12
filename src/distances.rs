use crate::cell;
use crate::grid;
use crate::hash_grid;
use std::collections::HashMap;

#[derive(Debug, Eq, PartialEq)]
pub struct DistanceMap {
    root: (i32, i32),
    pub map: HashMap<(i32, i32), u32>,
}

impl DistanceMap {
    pub fn new(root: (i32, i32), map: HashMap<(i32, i32), u32>) -> Self {
        Self { root, map }
    }

    pub fn from_hashgrid(start: (i32, i32), hgrid: &hash_grid::HashGrid) -> Self {
        let mut distance_map = HashMap::new();
        distance_map.insert(start, 0);
        let cell = hgrid.get_item(start).unwrap();
        let mut frontier: Vec<&cell::Cell> = vec![cell];
        while !frontier.is_empty() {
            let mut new_frontier: Vec<&cell::Cell> = vec![];
            for cell in frontier {
                for key in cell.links.iter() {
                    if !distance_map.contains_key(key) {
                        let current_weight = *distance_map.get(&(cell.row, cell.column)).unwrap();
                        distance_map.insert(*key, current_weight + 1);
                        new_frontier.push(hgrid.get_item(*key).unwrap());
                    }
                }
            }
            frontier = new_frontier;
        }

        Self {
            root: start,
            map: distance_map,
        }
    }
    pub fn from_grid(start: (i32, i32), grid: &grid::Grid) -> Self {
        let mut distance_map = HashMap::new();
        distance_map.insert(start, 0);
        let cell = grid.get_item(start).unwrap();
        let mut frontier: Vec<&cell::Cell> = vec![cell];
        while !frontier.is_empty() {
            let mut new_frontier: Vec<&cell::Cell> = vec![];
            for cell in frontier {
                for key in cell.links.iter() {
                    if !distance_map.contains_key(key) {
                        let current_weight = *distance_map.get(&(cell.row, cell.column)).unwrap();
                        distance_map.insert(*key, current_weight + 1);
                        new_frontier.push(grid.get_item(*key).unwrap());
                    }
                }
            }
            frontier = new_frontier;
        }

        Self {
            root: start,
            map: distance_map,
        }
    }

    pub fn path_to(
        &self,
        goal: (i32, i32),
        hgrid: &hash_grid::HashGrid,
    ) -> HashMap<(i32, i32), u32> {
        let mut path = HashMap::new();
        path.insert(goal, *self.map.get(&goal).unwrap());
        let mut current = goal;
        while current != goal {
            let cell = hgrid.get_item(current).unwrap();
            for key in cell.links.iter() {
                let neighbor_dist = *self.map.get(key).unwrap();
                if neighbor_dist < *self.map.get(&current).unwrap() {
                    path.insert(*key, neighbor_dist);
                    current = *key;
                }
            }
        }
        path.to_owned()
    }
}
