mod algorithms;
mod cell;
mod grid;
mod graph;
mod hash_grid;
mod render;

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

    let sidewinder_graph = graph::from_grid(sidewinder);
    graph::to_png(&sidewinder_graph, 50, 12, 12, "sidewinder_from_graph.png");
}