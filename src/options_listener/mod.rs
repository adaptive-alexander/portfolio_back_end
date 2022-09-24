use rayon::prelude::*;
use std::path::PathBuf;
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
    println!("Started processing options at: {}", Utc::now());
    let mut opts = Options::from_file(&file, Box::new(black_scholes::BlackScholesModel::new()));

    // Chunk options
    let mut chunked_opts = utilities::chunk_opt(opts, 1000);

    // Parallel computation of options_old
    chunked_opts.par_iter_mut().for_each(|x| x.get_prices());
    chunked_opts.par_iter_mut().for_each(|x| x.get_greeks());

    // Collect Options
    opts = utilities::collect_chunks(chunked_opts);

    // Write and time output
    opts.write_csv(PathBuf::from(format!("{id}.csv")))
        .expect("Failed writing output to csv.");
}
