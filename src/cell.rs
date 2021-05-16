use rand::prelude::IteratorRandom;
use std::collections;

#[derive(Hash, PartialEq, Eq)]
pub enum Direction {
    North,
    East,
    South,
    West
}

pub fn next_cell(current: (i32, i32), way: Direction) -> (i32, i32) {
    let (row, col) = current;
    match way {
        Direction::North => (row - 1, col),
        Direction::East => (row, col + 1),
        Direction::South => (row + 1, col),
        Direction::West => (row, col - 1),
    }
}

pub fn neighbor_cells(coords: (i32, i32)) -> collections::HashMap<Direction, (i32, i32)> {
    let mut answers: collections::HashMap<Direction, (i32, i32)> = collections::HashMap::new();
    let (row, col) = coords;
    let north = (row - 1, col);
    if north.0 >= 0 && north.1 >= 0 {
        answers.insert(Direction::North, north);
    }
    let east = (row, col + 1);
    if east.0 >= 0 && east.1 >= 0 {
        answers.insert(Direction::East, east);
    }
    let south  = (row + 1, col);
    if south.0 >= 0 && south.1 >= 0 {
        answers.insert(Direction::South, south);
    }
    let west = (row, col - 1);
    if west.0 >= 0 && west.1 >= 0 {
        answers.insert(Direction::West, west);
    }
    answers
}

#[derive(Eq, PartialEq, Debug, Clone)]
pub struct Cell {
    pub row: i32,
    pub column: i32,
    pub links: collections::HashSet<(i32, i32)>,
    pub north: Option<(i32, i32)>,
    pub south: Option<(i32, i32)>,
    pub east: Option<(i32, i32)>,
    pub west: Option<(i32, i32)>,
}

impl Cell {
    pub fn new(row: i32, column: i32) -> Self {
        Cell {
            row,
            column,
            links: collections::HashSet::new(),
            north: None,
            south: None,
            east: None,
            west: None
        }
    }

    pub fn coords(&self) -> (i32, i32) {
        (self.row, self.column)
    }

    pub fn link(&mut self, other: (i32, i32)) -> () {
        &self.links.insert(other);
        ()
    }

    pub fn has_links(&self) -> bool {
        !self.links.is_empty()
    }

    // pub fn unlink(&mut self, other: (i32, i32)) -> () {
    //     &self.links.remove(&other);
    //     ()

    // }

    // pub fn is_linked(&self, cell: &Cell) ->  bool {
    //     self.links.contains_key(&(cell.row, cell.column))
    // }
    pub fn add_neighbor(&mut self, other: (i32, i32)) -> Option<()> {
        let way = self.neighbor_direction(other);
        let result = match way {
            Some(Direction::East) => Some(self.east = Some(other)),
            Some(Direction::South) => Some(self.south = Some(other)),
            Some(Direction::West) => Some(self.west = Some(other)),
            Some(Direction::North) => Some(self.north = Some(other)),
            None => None
        };
        if let Some(()) = result {
            self.link(other);
            return Some(());
        }
        None
    }

    pub fn match_direction(&self, way: &Direction) -> Option<(i32, i32)> {
        match way {
            Direction::East => self.east,
            Direction::South => self.south,
            Direction::West => self.west,
            Direction::North => self.north,
        }
    }
    pub fn neighbor_direction(&self, neighbor: (i32, i32)) -> Option<Direction> {
        if neighbor == (self.row - 1, self.column) {
            return Some(Direction::North);
        }
        else if neighbor == (self.row + 1, self.column) {
            return Some(Direction::South);
        }
        else if neighbor == (self.row, self.column - 1) {
            return Some(Direction::West);
        }
        else if neighbor == (self.row, self.column + 1) {
            return Some(Direction::East);
        } else {
            return None;
        }
    }

    pub fn direction_has_link(&self, way: Direction) -> bool {
        let result = match self.match_direction(&way) {
            Some(_cl) => {
                let way_coords = next_cell((self.row as i32, self.column as i32), way);
                self.links.contains(&way_coords)
            }
            _ => false,
        };
        result
    }

    pub fn neighbors(&self) -> Vec<&(i32, i32)> {
        let result: Vec<&(i32, i32)> = vec![
            &self.north,
            &self.east,
            &self.south,
            &self.west
        ]
        .iter()
        .filter(|&elem| elem.is_some())
        .map(|&elem| elem.as_ref().expect("This should have been filtered out!"))
        .collect();
        result
    }
    pub fn random_neighbor(&self) -> Option<&(i32, i32)> {
        vec![
            &self.north,
            &self.east,
            &self.south,
            &self.west
        ]
        .iter()
        .filter(|&elem| elem.is_some())
        .map(|&elem| elem.as_ref().expect("This should have been filtered out!"))
        .choose(&mut rand::thread_rng())

    }

}