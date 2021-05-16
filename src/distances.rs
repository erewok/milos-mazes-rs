use std::collections::HashMap;
use crate::hash_grid;
use crate::cell;


#[derive(Debug, Eq, PartialEq)]
pub struct DistanceMap {
    root: (i32, i32),
    pub map: HashMap<(i32, i32), u32>
}

impl DistanceMap {
    pub fn from_hashgrid(start: (i32, i32), hgrid: &hash_grid::HashGrid) -> Self {
        let mut distance_map = HashMap::new();
        distance_map.insert(start, 0);
        let cell = hgrid.get_item(start).unwrap();
        let mut frontier: Vec<&cell::Cell> = vec![cell];
        while frontier.len() > 0 {
            let mut new_frontier: Vec<&cell::Cell> = vec![];
            for cell in frontier {
                for key in cell.links.iter() {
                    if !distance_map.contains_key(key) {
                        let current_weight = distance_map.get(&(cell.row, cell.column)).unwrap();
                        distance_map.insert(*key, current_weight + 1);
                        new_frontier.push(hgrid.get_item(*key).unwrap());
                    }
                }
            }
            frontier = new_frontier;
        }

        Self {root: start, map: distance_map}

    }
    pub fn from_grid(start: (i32, i32), hgrid: &hash_grid::HashGrid) -> Self {
        let mut distance_map = HashMap::new();
        distance_map.insert(start, 0);
        let cell = hgrid.get_item(start).unwrap();
        let mut frontier: Vec<&cell::Cell> = vec![cell];
        while frontier.len() > 0 {
            let mut new_frontier: Vec<&cell::Cell> = vec![];
            for cell in frontier {
                for key in cell.links.iter() {
                    if !distance_map.contains_key(key) {
                        let current_weight = distance_map.get(&(cell.row, cell.column)).unwrap();
                        distance_map.insert(*key, current_weight + 1);
                        new_frontier.push(hgrid.get_item(*key).unwrap());
                    }
                }
            }
            frontier = new_frontier;
        }

        Self {root: start, map: distance_map}

    }
}