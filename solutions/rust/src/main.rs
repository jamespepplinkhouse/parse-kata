use csv::ReaderBuilder;
use futures::stream::FuturesUnordered;
use futures::StreamExt;
use serde_json::{self, Value};
use std::env;
use std::error::Error;
use std::fs::File;
use std::io::{BufReader, BufWriter, Write};

async fn process_input_file(input_path: &str, output_path: &str) -> Result<(), Box<dyn Error>> {
    let input_file = File::open(input_path)?;
    let output_file = File::create(output_path)?;
    let input_buffered_reader = BufReader::new(input_file);
    let mut output_buffered_writer = BufWriter::new(output_file);

    let mut reader = ReaderBuilder::new()
        .delimiter(b'\t')
        .has_headers(false)
        .from_reader(input_buffered_reader);

    let mut records = reader.byte_records();
    let mut futures = FuturesUnordered::new();

    while let Some(record_result) = records.next() {
        let record = record_result?;
        futures.push(async_std::task::spawn_blocking(move || {
            let fields: Vec<_> = record.iter().collect();
            if fields.len() == 5 {
                let json_value: Value = serde_json::from_slice(&fields[4])
                    .map_err(|e| format!("Error parsing JSON: {}", e))?;
                if let Value::Object(map) = json_value {
                    if let Some(Value::String(title_str)) = map.get("title") {
                        return Ok(Some(title_str.to_string()));
                    }
                }
            }
            Ok::<Option<String>, String>(None)
        }));
    }

    while let Some(result) = futures.next().await {
        let title_opt = result.map_err(|e| format!("Error processing line: {}", e));
        if let Ok(Some(title)) = title_opt {
            writeln!(output_buffered_writer, "{}", title)?;
        }
    }

    // Flush the writer to ensure all output is written to the file
    output_buffered_writer.flush()?;

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
