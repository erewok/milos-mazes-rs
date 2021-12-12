use rand::prelude::*;
use raqote::DrawTarget;
use std::collections::HashMap;

use crate::cell;
use crate::distances;
use crate::render;

pub struct Neighbors {
    north_cell: (i32, i32),
    east_cell: (i32, i32),
    south_cell: (i32, i32),
    west_cell: (i32, i32),
}

pub fn get_neighbor_coords(current: (i32, i32)) -> Neighbors {
    Neighbors {
        north_cell: cell::next_cell(current, cell::Direction::North),
        east_cell: cell::next_cell(current, cell::Direction::East),
        south_cell: cell::next_cell(current, cell::Direction::South),
        west_cell: cell::next_cell(current, cell::Direction::West),
    }
}

#[derive(Eq, PartialEq, Debug)]
pub struct HashGrid {
    pub rows: i32,
    pub columns: i32,
    grid: HashMap<(i32, i32), cell::Cell>,
    distances: Option<distances::DistanceMap>,
}

impl HashGrid {
    pub fn new(rows: i32, columns: i32) -> Self {
        let mut grd_init = Self {
            rows,
            columns,
            grid: HashMap::new(),
            distances: None,
        };
        grd_init.prepare_grid().configure_cells();
        grd_init
    }

    pub fn len(&self) -> usize {
        self.grid.len()
    }

    pub fn from_cells(cells: Vec<Vec<cell::Cell>>) -> Self {
        let mut grd_init = Self {
            rows: cells.len() as i32,
            columns: cells[0].len() as i32,
            grid: HashMap::new(),
            distances: None,
        };
        for cll in cells.iter().flatten().into_iter() {
            grd_init.grid.insert((cll.row, cll.column), cll.to_owned());
        }
        grd_init
    }

    pub fn prepare_grid(&mut self) -> &mut Self {
        for rownum in 0..self.rows {
            for colnum in 0..self.columns {
                let new_cell = cell::Cell::new(rownum, colnum);
                self.grid.insert((new_cell.row, new_cell.column), new_cell);
            }
        }

        self
    }
    pub fn configure_cells(&mut self) -> &mut Self {
        let mut new_grid = HashMap::new();
        for (location, cll) in self.grid.iter() {
            let mut new_cell = cll.clone();
            let neighbors = get_neighbor_coords((cll.row, cll.column));
            if self.grid.contains_key(&neighbors.north_cell) {
                new_cell.north = Some(neighbors.north_cell);
            } else {
                new_cell.north = None;
            }
            if self.grid.contains_key(&neighbors.east_cell) {
                new_cell.east = Some(neighbors.east_cell);
            } else {
                new_cell.east = None;
            }
            if self.grid.contains_key(&neighbors.south_cell) {
                new_cell.south = Some(neighbors.south_cell);
            } else {
                new_cell.south = None;
            }
            if self.grid.contains_key(&neighbors.west_cell) {
                new_cell.west = Some(neighbors.west_cell);
            } else {
                new_cell.west = None;
            }
            new_grid.insert((location.0, location.1), new_cell);
        }
        self.grid = new_grid;
        self
    }

    pub fn get_item(&self, row_col: (i32, i32)) -> Option<&cell::Cell> {
        let (rownum, colnum) = row_col;
        if rownum >= self.rows || rownum < 0 {
            return None;
        }
        if colnum >= self.columns || colnum < 0 {
            return None;
        }
        self.grid.get(&row_col)
    }
    pub fn replace_cell(&mut self, cll: cell::Cell) -> Result<(), &str> {
        // bounds check
        let (row_num, col_num) = (cll.row, cll.column);
        if row_num >= self.rows || row_num < 0 {
            return Err("Row number must be within bounds of the grid");
        }
        if col_num >= self.columns || col_num < 0 {
            return Err("Column number must be within bounds of the grid");
        }
        self.grid.insert((row_num, col_num), cll);
        Ok(())
    }

    pub fn size(&self) -> i32 {
        self.rows * self.columns
    }

    pub fn random_cell(&self) -> Option<&cell::Cell> {
        let mut rng = thread_rng();
        let rownum = rng.gen_range(0..self.rows);
        let colnum = rng.gen_range(0..self.columns);
        self.grid.get(&(rownum, colnum))
    }

    pub fn to_png(&self, cell_size: i32, filename: &str) -> Result<(), String> {
        let img_width: i32 = cell_size * self.columns;
        let img_height: i32 = cell_size * self.rows;
        let mut dt = DrawTarget::new(
            (img_width + cell_size * 2i32) as i32,
            (img_height + cell_size * 2i32) as i32,
        );

        for rownum in 0..self.rows {
            for colnum in 0..self.columns {
                // we pad it an extra + cell_size to keep it off from the edges
                let some_cell = self.grid.get(&(rownum, colnum)).unwrap();
                render::draw_cell(&mut dt, cell_size, some_cell.to_owned());
            }
        }
        dt.write_png(filename)
            .map_err(|err| format!("Failed writing file {}", err))
    }

    pub fn build_distance_map(&mut self) {
        let start = (self.rows - 1, 0);
        self.distances = Some(distances::DistanceMap::from_hashgrid(start, self));
    }

    pub fn build_breadcrumbs_to_longest(&mut self) {
        let start = (self.rows - 1, 0);
        let mut maxval = 0u32;
        let dm = distances::DistanceMap::from_hashgrid(start, self);
        // get the spot furthest away from root
        let endpoint = dm.map.iter().fold((0i32, 0i32), |acc, val| {
            if val.1 > &maxval {
                maxval = *val.1;
                *val.0
            } else {
                acc
            }
        });
        // build a distance map with just those breadcrumbs
        let breadcrumbs: HashMap<(i32, i32), u32> = dm.path_to(endpoint, self);
        self.distances = Some(distances::DistanceMap::new((0i32, 0i32), breadcrumbs));
    }

    pub fn get_cell_body(&self, cell_loc: &(i32, i32)) -> String {
        match self.distances.as_ref() {
            None => "    ".to_string(),
            Some(dist) => match dist.map.get(cell_loc) {
                None => "    ".to_string(),
                Some(dist) => format!(" {number:>0width$} ", number = dist, width = 2),
            },
        }
    }
}

impl std::fmt::Display for HashGrid {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let line_separator = "----+".repeat(self.columns as usize);
        let corner = "+".to_string();
        for rownum in 0..self.rows {
            let mut body = "|".to_string();
            let mut top = "+".to_string();
            for colnum in 0..self.columns {
                let some_cell = self.grid.get(&(rownum, colnum)).unwrap();
                let north_boundary = if some_cell.direction_has_link(cell::Direction::North) {
                    "    "
                } else {
                    "----"
                };
                let east_boundary = if some_cell.direction_has_link(cell::Direction::East) {
                    " "
                } else {
                    "|"
                };

                top = format!("{}{}{}", top, north_boundary, corner);
                // body = format!("{}{}{}", west_boundary, self.get_cell_body(&(rownum, colnum)), east_boundary);
                body = format!(
                    "{}{}{}",
                    body,
                    self.get_cell_body(&(rownum, colnum)),
                    east_boundary
                );
            }
            let _ = writeln!(f, "{}", top);
            let _ = writeln!(f, "{}", body);
        }
        writeln!(f, "+{}", line_separator)
    }
}
