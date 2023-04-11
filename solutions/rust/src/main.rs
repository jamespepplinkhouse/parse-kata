use async_std::fs::File;
use async_std::io::{BufReader, BufWriter};
use async_std::prelude::*;
use serde_json::Value;
use std::env;
use std::error::Error;

async fn process_input_file(input_path: &str, output_path: &str) -> Result<(), Box<dyn Error>> {
    let input_file = File::open(input_path).await?;
    let output_file = File::create(output_path).await?;
    let buffer_size = 100 * 1024 * 1024; // 100MB
    let input_buffered_reader = BufReader::with_capacity(buffer_size, input_file);
    let mut output_buffered_writer = BufWriter::new(output_file);

    let mut line_stream = input_buffered_reader.lines();

    while let Some(line_result) = line_stream.next().await {
        let line = line_result?;
        if let Some(json_string) = line.find('{').map(|start_index| &line[start_index..]) {
            let json_value: Value = serde_json::from_str(json_string).map_err(|e| e.to_string())?;
            if let Some(title) = json_value.get("title") {
                if let Some(title_str) = title.as_str() {
                    output_buffered_writer
                        .write_all(format!("{}\n", title_str).as_bytes())
                        .await?;
                }
            }
        }
    }

    // Flush the writer to ensure all output is written to the file
    output_buffered_writer.flush().await?;

    Ok(())
}

#[async_std::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();
    if args.len() != 3 {
        println!("Usage: {} <input_file> <output_file>", args[0]);
        return Ok(());
    }

    let input_path = &args[1];
    let output_path = &args[2];

    process_input_file(input_path, output_path)
        .await
        .map_err(|err| println!("Error processing file: {}", err))
        .ok();

    println!("Processing complete.");

    Ok(())
}
