mod cell;
mod grid;
mod algorithms;

fn main() {
    let mut new_grid = grid::Grid::new(4, 4);
    let binary_grid = algorithms::binary_tree(&mut new_grid);
    println!("{}", binary_grid);
    println!("");
    println!("{}", "---".repeat(10));
    println!("");
    let sidewinder = algorithms::sidewinder(&mut new_grid);
    println!("{}", sidewinder);
}