mod cell;
mod grid;
mod algorithms;

fn main() {
    let mut new_grid = grid::Grid::new(12, 12);
    let binary_grid = algorithms::binary_tree(&mut new_grid);
    println!("Binary tree");
    println!("{}", binary_grid);
    println!("");
    println!("{}", "---".repeat(20));
    println!("Sidewinder");
    let sidewinder = algorithms::sidewinder(&mut new_grid);
    println!("{}", sidewinder);
    
    sidewinder.to_png(50, "maze.png")
}