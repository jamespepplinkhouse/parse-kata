use parse_kata::process_input_file_bytes;
use parse_kata::process_input_file_json;
use std::env;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();
    if args.len() != 4 {
        println!(
            "Usage: {} <input_file> <output_file> <fast_mode_boolean>",
            args[0]
        );
        return Ok(());
    }

    let input_path = &args[1];
    let output_path = &args[2];
    let fast_mode = &args[3];

    if fast_mode == "true" {
        process_input_file_bytes(input_path, output_path)
            .map_err(|err| println!("Error processing file: {}", err))
            .ok();
    } else {
        process_input_file_json(input_path, output_path)
            .map_err(|err| println!("Error processing file: {}", err))
            .ok();
    }

    println!("Processing complete.");

    Ok(())
}
