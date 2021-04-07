use rand::prelude::*;
use raqote::{DrawTarget};
use crate::cell;
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
pub struct Grid {
    pub rows: i32,
    pub columns: i32,
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
    pub fn iter(&self) -> IterGrid {
        IterGrid::new(self)
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
                let north = match self.get_item(neighbors.north_cell) {
                    Some(val) => Some(val.coords()),
                    None => None,
                };
                let east = match self.get_item(neighbors.east_cell) {
                    Some(val) => Some(val.coords()),
                    None => None,
                };
                let south = match self.get_item(neighbors.south_cell) {
                    Some(val) => Some(val.coords()),
                    None => None,
                };
                let west = match self.get_item(neighbors.west_cell) {
                    Some(val) => Some(val.coords()),
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
    pub fn replace_cell(&mut self, cll: cell::Cell) -> Result<(), &str> {
        // bounds check
        let (row_num, col_num) = (cll.row, cll.column);
        if row_num >= self.rows || row_num < 0 {
            return Err("Row number must be within bounds of the grid");
        }
        if col_num >= self.columns || col_num < 0 {
            return Err("Column number must be within bounds of the grid");
        }
        self.grid[row_num as usize][col_num as usize] = cll;
        Ok(())
    }

    pub fn size(&self) -> i32 {
        self.rows * self.columns
    }

    pub fn random_cell(&self) -> &cell::Cell {
        let mut rng = thread_rng();
        let rownum = rng.gen_range(0..self.rows);
        let colnum = rng.gen_range(0..self.columns);
        &self.grid[rownum as usize][colnum as usize]
    }
    pub fn random_cell_cloned(&self) -> cell::Cell {
        let mut rng = thread_rng();
        let rownum = rng.gen_range(0..self.rows);
        let colnum = rng.gen_range(0..self.columns);
        self.grid[rownum as usize][colnum as usize].clone()
    }
    pub fn random_cell_mut(&mut self) -> &mut cell::Cell {
        let mut rng = thread_rng();
        let rownum = rng.gen_range(0..self.rows);
        let colnum = rng.gen_range(0..self.columns);
        &mut self.grid[rownum as usize][colnum as usize]
    }

    pub fn each_row(&self) -> std::slice::Iter<Vec<cell::Cell>> {
        self.grid.iter()
    }

    pub fn to_png(&self, cell_size: i32, filename: &str) -> Result<(), String> {
        let img_width: i32 = cell_size * &self.columns;
        let img_height: i32 = cell_size * &self.rows;
        let mut dt = DrawTarget::new((img_width + cell_size * 2i32) as i32, (img_height + cell_size * 2i32) as i32);

        for rownum in 0..self.rows {
            for colnum in 0..self.columns {
                // we pad it an extra + cell_size to keep it off from the edges
                let some_cell = self.grid[rownum as usize][colnum as usize].clone();
                render::draw_cell(&mut dt, cell_size, some_cell);
            }
        }
        dt.write_png(filename).map_err(|err| format!("Failed writing file {}", err))
    }
}

impl std::fmt::Display for Grid {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let line_separator = "---+".repeat(self.columns as usize).to_string();
        let corner = "+".to_string();
        for rownum in 0..self.rows {
            let mut body = "|".to_owned();
            let mut top = "+".to_owned();
            for colnum in 0..self.columns {
                let some_cell = self.grid[rownum as usize][colnum as usize].clone();
                let north_boundary =
                    if some_cell.direction_has_link(cell::Direction::North) { "   " }
                    else {"---"};
                let east_boundary =
                    if some_cell.direction_has_link(cell::Direction::East) { " " }
                    else {"|"};
                top = format!("{}{}{}", top, north_boundary, corner);
                body = format!("{}   {}", body, east_boundary);
            }
            let _ = write!(f, "{}\n", top);
            let _ = write!(f, "{}\n", body);
        }
        write!(f, "+{}\n", line_separator)
    }
}
pub struct IterGrid<'a> {
    grid: &'a Grid,
    row_col: Option<(i32, i32)>,
    next: Option<&'a cell::Cell>,
}

impl<'a> IterGrid<'a> {
    fn new(grid: &'a Grid) -> IterGrid<'a> {
        IterGrid { grid, row_col: Some((0, 0)), next: None }
    }
}

impl<'a> Iterator for IterGrid<'a> {
    type Item = &'a cell::Cell;

    fn next(&mut self) -> Option<Self::Item> {
        if self.row_col.is_none() {
            return None;
        }
        let (mut row, mut col) = self.row_col.unwrap();
        let next_cell: Option<&cell::Cell> = self.grid.get_item((row, col));
        if col < self.grid.columns - 1 {
            col += 1;
        } else {
            col = 0;
            row += 1
        }
        if row < self.grid.rows {
            self.row_col = Some((row, col));
        } else {
            self.row_col = None;
        }
        self.next = next_cell;
        next_cell
    }
}

#[cfg(test)]
mod test {
    use super::Grid;
    #[test]
    fn iter() {
        let new_grid = Grid::new(2, 2);
        let mut iter = new_grid.iter();
        assert_eq!(iter.next().map(|cl| (cl.row, cl.column)), Some((0, 0)));
        assert_eq!(iter.next().map(|cl| (cl.row, cl.column)), Some((0, 1)));
        assert_eq!(iter.next().map(|cl| (cl.row, cl.column)), Some((1, 0)));
        assert_eq!(iter.next().map(|cl| (cl.row, cl.column)), Some((1, 1)));
        assert_eq!(iter.next().map(|cl| (cl.row, cl.column)), None);
    }
}