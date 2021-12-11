use clap::Parser;

mod algorithms;
mod cell;
mod distances;
mod graph;
mod grid;
mod hash_grid;
mod render;

#[derive(Parser)]
#[clap(version = "0.3.0", author = "Erik Aker <eraker@gmail.com> & Milo Aker")]
struct Opts {
    #[clap(short, long, default_value = "12")]
    rows: u8,
    #[clap(short, long, default_value = "12")]
    columns: u8,
    #[clap(short, long, default_value = "aldous-broder")]
    algorithm: String,
    #[clap(short, long)]
    outfile: Option<String>,
    #[clap(long)]
    with_distance_map: bool,
    #[clap(long)]
    with_breadcrumbs: bool,
}

fn main() {
    let opts: Opts = Opts::parse();
    let mut new_hgrid = hash_grid::HashGrid::new(opts.rows as i32, opts.columns as i32);
    let mut hgrid = match opts.algorithm.as_str() {
        "aldous-broder" => algorithms::aldous_broder(&mut new_hgrid),
        _ => panic!("Unimplemented algorithm for hash grid"),
    };
    if opts.with_distance_map {
        hgrid.build_distance_map();
    }
    if opts.with_breadcrumbs {
        hgrid.build_breadcrumbs_to_longest();
    }
    println!("Aldous Broder");
    println!("{}", hgrid);

    match opts.outfile {
        None =>  (),
        Some(fname) => hgrid
            .to_png(30, fname.as_str())
            .expect("Couldn't write file"),
    };
}
