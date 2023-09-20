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
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let opt = Opt::from_args();
    println!("Processing started!");
    process_input_file_json(&opt.input_file, &opt.output_file).await?;
    println!("Processing complete!");
    Ok(())
}
