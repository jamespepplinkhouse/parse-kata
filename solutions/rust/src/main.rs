use parse_kata::process_input_file_bytes;
use parse_kata::process_input_file_json;
use std::error::Error;
use structopt::StructOpt;

/// A basic example
#[derive(StructOpt, Debug)]
#[structopt(
    name = "parse-kata",
    about = "A parse-kata implementation written in Rust."
)]
struct Opt {
    /// Input file
    #[structopt(short = "i", long = "input")]
    input_file: String,

    /// Output file
    #[structopt(short = "o", long = "output")]
    output_file: String,

    /// Fast mode
    #[structopt(short = "f", long = "fast")]
    fast_mode: bool,
}

fn main() -> Result<(), Box<dyn Error>> {
    let opt = Opt::from_args();
    println!("{:?}", opt);

    if opt.fast_mode {
        process_input_file_bytes(&opt.input_file, &opt.output_file)
            .map_err(|err| println!("Error processing file: {}", err))
            .ok();
    } else {
        process_input_file_json(&opt.input_file, &opt.output_file)
            .map_err(|err| println!("Error processing file: {}", err))
            .ok();
    }

    println!("Processing complete.");

    Ok(())
}
