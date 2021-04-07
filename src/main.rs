mod cell;
mod grid;
mod algorithms;
mod graph;
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

    // let broder = algorithms::aldous_broder(&mut new_grid);
    // println!("{}", broder);
    // broder.to_png(50, "broder.png")
    let sidewinder_graph = graph::from_grid(sidewinder);
    graph::to_png(&sidewinder_graph, 50, 12, 12, "sidewinder_from_graph.png");
}