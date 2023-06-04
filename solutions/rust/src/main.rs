use parse_kata::process_input_file_bytes;
use parse_kata::process_input_file_json;
use std::error::Error;
use structopt::StructOpt;

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

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let opt = Opt::from_args();

    if opt.fast_mode {
        println!("Fast mode enabled: using custom byte parser");
        process_input_file_bytes(&opt.input_file, &opt.output_file).await?;
    } else {
        println!("Normal mode enabled: using stream of lines and JSON parser");
        process_input_file_json(&opt.input_file, &opt.output_file).await?;
    }

    println!("Processing complete!");
    Ok(())
}
