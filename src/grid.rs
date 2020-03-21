use crate::cell;
use rand::prelude::*;


pub struct Neighbors {
    NorthCell: (i32, i32),
    EastCell: (i32, i32),
    SouthCell: (i32, i32),
    WestCell: (i32, i32),
}

pub fn get_neighbor_coords(current: (i32, i32)) -> Neighbors {
    Neighbors {
        NorthCell: cell::next_cell(current, cell::Direction::North),
        EastCell: cell::next_cell(current, cell::Direction::East),
        SouthCell: cell::next_cell(current, cell::Direction::South),
        WestCell: cell::next_cell(current, cell::Direction::West),
    }
}


#[derive(Eq, PartialEq, Debug)]
pub struct Grid {
    rows: i32,
    columns: i32,
    grid: Vec<Vec<cell::Cell>>
}

impl Grid {

    pub fn new(rows: i32, columns: i32) -> Self {
        let mut grd_init = Grid {
            rows,
            columns,
            grid: Vec::new()
        };
        grd_init
            .prepare_grid()
            .configure_cells();
        grd_init
    }

    pub fn from_cells(cells: Vec<Vec<cell::Cell>>) -> Self {
        let grd_init = Grid {
            rows: cells.len() as i32,
            columns: cells[0].len() as i32,
            grid: cells
        };
        grd_init
    }

    pub fn prepare_grid(&mut self) -> &mut Self {
        let mut outer: Vec<Vec<cell::Cell>> = Vec::new();
        for rownum in 0..self.rows {
            let mut inner: Vec<cell::Cell> = Vec::new();
            for colnum in 0..self.columns {
                let new_cell = cell::Cell::new(rownum, colnum);
                inner.push(new_cell);
            }
            outer.push(inner);
        }
        self.grid = outer;
        self
    }
    pub fn configure_cells(&mut self) -> &mut Self {
        let mut outer: Vec<Vec<cell::Cell>> = Vec::new();
        for rownum in 0..self.rows {
            let mut inner: Vec<cell::Cell> = Vec::new();
            for colnum in 0..self.columns {
                let mut new_cell = self.grid[rownum as usize][colnum as usize].clone();
                let neighbors = get_neighbor_coords((*&new_cell.row, *&new_cell.column));
                let north = match self.get_item(neighbors.NorthCell) {
                    Some(val) => Some(Box::new(val.clone())),
                    None => None,
                };
                let east = match self.get_item(neighbors.EastCell) {
                    Some(val) => Some(Box::new(val.clone())),
                    None => None,
                };
                let south = match self.get_item(neighbors.SouthCell) {
                    Some(val) => Some(Box::new(val.clone())),
                    None => None,
                };
                let west = match self.get_item(neighbors.WestCell) {
                    Some(val) => Some(Box::new(val.clone())),
                    None => None,
                };
                new_cell.north = north;
                new_cell.east = east;
                new_cell.south = south;
                new_cell.west = west;
                inner.push(new_cell);
            }
            outer.push(inner);
        }
        self.grid = outer;
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
        Some(&self.grid[rownum as usize][colnum as usize])
    }

    pub fn size(&self) -> i32 {
        *&self.rows & *&self.columns
    }

    pub fn random_cell(&self) -> &cell::Cell {
        let mut rng = thread_rng();
        let rownum = rng.gen_range(0, &self.rows);
        let colnum = rng.gen_range(0, &self.columns);
        &self.grid[rownum as usize][colnum as usize]
    }

    pub fn each_row(&self) -> std::slice::Iter<Vec<cell::Cell>> {
        self.grid.iter()
    }
    pub fn each_cell(&self) -> Vec<cell::Cell> {
        let mut outer: Vec<cell::Cell> = Vec::new();
        for rownum in 0..self.rows {
            for colnum in 0..self.columns {
                let some_cell = self.grid[rownum as usize][colnum as usize].clone();
                outer.push(some_cell);
            }
        }
        outer
    }

}

impl std::fmt::Display for Grid {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let line_separator = "---+".repeat(self.columns as usize).to_string();
        let corner = "+".to_string();
        let _ = write!(f, "+{}\n", line_separator);
        for rownum in 0..self.rows {
            let mut body = "|".to_owned();
            let mut bottom = "+".to_owned();
            for colnum in 0..self.columns {
                let some_cell = self.grid[rownum as usize][colnum as usize].clone();
                let east_boundary =
                    if some_cell.direction_has_link(cell::Direction::East) { " " }
                    else {"|"};
                let south_boundary =
                    if some_cell.direction_has_link(cell::Direction::South) { " " }
                    else {"---"};
                body = format!("{}   {}", body, east_boundary);
                // println!("{}", body);
                // println!("{}", bottom);
                bottom = format!("{}{}{}", bottom, south_boundary, corner);
            }
            let _ = write!(f, "{}\n", body);
            let _ = write!(f, "{}\n", bottom);
        }
        write!(f, "\n")
    }
}