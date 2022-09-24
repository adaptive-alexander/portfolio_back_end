use rayon::prelude::*;
use std::path::PathBuf;
use std::time::{Duration, Instant};
use std::{thread};
use chrono::offset::Utc;
use options::options_struct::Options;
use options::pricing_models::black_scholes;
use options::utilities;

/// # run_opt_calc_dir
/// Run function for listener
///
/// # args:
/// *`inp_path` - Path for input file.
/// *`out_path` - Path for output file.
/// *`move_path` - Path to move processed input files into.
pub fn run_api_calc(file: PathBuf, id: String) {
    // Initialize data
    thread::sleep(Duration::from_millis(1)); // Avoids OS sometimes still using file when moved into "input"

    // Timing initialization
    let mut start = Instant::now();
    println!("Started processing options at: {}", Utc::now());
    let mut opts = Options::from_file(&file, Box::new(black_scholes::BlackScholesModel::new()));
    println!(
        "Time to parse inputs: {} ms (OS bound)",
        start.elapsed().as_millis()
    );

    // Timing computation and chunking
    start = Instant::now();

    // Chunk options
    let mut chunked_opts = utilities::chunk_opt(opts, 1000);

    // Timing computation
    let start_comp = Instant::now();

    // Parallel computation of options_old
    chunked_opts.par_iter_mut().for_each(|x| x.get_prices());
    chunked_opts.par_iter_mut().for_each(|x| x.get_greeks());
    println!(
        "Time for computation: {} µs",
        start_comp.elapsed().as_micros()
    );

    // Collect Options
    opts = utilities::collect_chunks(chunked_opts);
    println!(
        "Time to compute including chunking and collecting: {} µs",
        start.elapsed().as_micros()
    );

    // Write and time output
    start = Instant::now();
    opts.write_csv(PathBuf::from(format!("{id}.csv")))
        .expect("Failed writing output to csv.");
    println!(
        "Time to write result: {} ms (OS bound)",
        start.elapsed().as_millis()
    );
}
