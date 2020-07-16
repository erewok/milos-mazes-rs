use crate::cell;
use rand::prelude::*;
use raqote::*;


pub struct Neighbors {
    north_cell: (i32, i32),
    east_cell: (i32, i32),
    south_cell: (i32, i32),
    west_cell: (i32, i32),
}

pub struct BoxCoords {
    x1: f32,
    x2: f32,
    y1: f32,
    y2: f32
}

pub fn get_neighbor_coords(current: (i32, i32)) -> Neighbors {
    Neighbors {
        north_cell: cell::next_cell(current, cell::Direction::North),
        east_cell: cell::next_cell(current, cell::Direction::East),
        south_cell: cell::next_cell(current, cell::Direction::South),
        west_cell: cell::next_cell(current, cell::Direction::West),
    }
}

pub fn draw_cell(dt: &mut DrawTarget, coords: BoxCoords, cll: cell::Cell) -> &mut DrawTarget {
    let mut pb = PathBuilder::new();

    if cll.west.is_none() {
        pb.move_to(coords.x1, coords.y1);
        pb.line_to(coords.x1, coords.y2);
    }
    if cll.south.is_none() {
        pb.move_to(coords.x1, coords.y2);
        pb.line_to(coords.x2, coords.y2);
    }
    if cll.east.is_none() {
        pb.move_to(coords.x2, coords.y2);
        pb.line_to(coords.x2, coords.y1);
    }
    if cll.north.is_none() {
        pb.move_to(coords.x2, coords.y1);
        pb.line_to(coords.x1, coords.y1);
    }
    if !cll.direction_has_link(cell::Direction::North) {
        pb.move_to(coords.x1, coords.y1);
        pb.line_to(coords.x2, coords.y1);
    }
    if !cll.direction_has_link(cell::Direction::East) {
        pb.move_to(coords.x2, coords.y2);
        pb.line_to(coords.x2, coords.y1);
    }    


    let path = pb.finish();

    dt.stroke(
        &path,
        &Source::Solid(SolidSource {
            r: 0x0,
            g: 0x0,
            b: 0x0,
            a: 0x99
        }),
        &StrokeStyle {
            cap: LineCap::Round,
            join: LineJoin::Round,
            width: 2.,
            miter_limit: 1.,
            dash_array: vec![],
            dash_offset: 0.,
        },
        &DrawOptions::new()
    );

    dt
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
                    Some(val) => Some(Box::new(val.clone())),
                    None => None,
                };
                let east = match self.get_item(neighbors.east_cell) {
                    Some(val) => Some(Box::new(val.clone())),
                    None => None,
                };
                let south = match self.get_item(neighbors.south_cell) {
                    Some(val) => Some(Box::new(val.clone())),
                    None => None,
                };
                let west = match self.get_item(neighbors.west_cell) {
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

    pub fn to_png(&self, cell_size: i32, filename: &str) -> () {
        let img_width = cell_size * &self.columns;
        let img_height = cell_size * &self.rows;
        let mut dt = DrawTarget::new(img_width + cell_size * 2, img_height + cell_size * 2);

        for rownum in 0..self.rows {
            for colnum in 0..self.columns {
                // we pad it an extra + cell_size to keep it off from the edges
                let some_cell = self.grid[rownum as usize][colnum as usize].clone();
                let coords = BoxCoords {
                    x1: (colnum * cell_size + cell_size) as f32,
                    x2: ((colnum + 1) * cell_size + cell_size) as f32,
                    y1: (rownum * cell_size + cell_size) as f32,
                    y2: ((rownum + 1) * cell_size + cell_size) as f32

                };
                draw_cell(&mut dt, coords, some_cell);
            }
        }

        dt.write_png(filename);
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