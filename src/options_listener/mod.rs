use std::fs::File;
use std::io;
use std::io::BufRead;
use rayon::prelude::*;
use std::path::{Path, PathBuf};
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

    // Parallel computation of options
    chunked_opts.par_iter_mut().for_each(|x| x.get_prices());
    chunked_opts.par_iter_mut().for_each(|x| x.get_greeks());

    // Collect Options
    opts = utilities::collect_chunks(chunked_opts);

    println!("Writing output file {id}");
    // Write and time output
    opts.write_csv(PathBuf::from(format!("{id}.csv")))
        .expect("Failed writing output to csv.");
}

pub fn opt_file_healthy(file_path: String) -> bool {
    let lines = read_lines(Path::new(&file_path));
    let mut ok_lines;
    match lines {
        Ok(l) => ok_lines = l,
        Err(_) => return false
    }
    let headers = vec!["settle", "maturity", "ticker", "opt_type", "underlying", "volatility", "rfr", "dividend", "strike"];

    match ok_lines.next() {
        Some(l) => {
            let split = l.unwrap().split(',').map(|s| s.to_lowercase().to_string()).collect::<Vec<String>>();
            for header in headers {
                match split.contains(&header.to_string()) {
                    true => {}
                    false => {
                        println!("Header not found in file");
                        return false;
                    }
                }
            }
        }
        None => return false
    }

    for line in ok_lines.flatten() {
        let split = line.split(',').map(|s| s.to_string()).collect::<Vec<String>>();
        if split.len() < 9 {
            println!("File data incomplete");
            return false;
        }
    }
    true
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
    where
        P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
