use async_std::fs::File;
use async_std::io::{BufReader, BufWriter};
use async_std::prelude::*;
use serde_json::Value;
use std::env;
use std::error::Error;

async fn _process_input_file_json(
    input_path: &str,
    output_path: &str,
) -> Result<(), Box<dyn Error>> {
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

async fn process_input_file_bytes(
    input_path: &str,
    output_path: &str,
) -> Result<(), Box<dyn Error>> {
    let input_file = File::open(input_path).await?;
    let output_file = File::create(output_path).await?;
    let buffer_size = 100 * 1024 * 1024; // 100MB

    let mut reader = BufReader::new(input_file);
    let mut writer = BufWriter::new(output_file);
    let mut buffer = vec![0; buffer_size];

    // Thread 1: Read the file in chunks
    // Thread 1: Find title marker bytes
    // Thread 1: Write title to output file

    while let Ok(bytes_read) = reader.read(&mut buffer).await {
        if bytes_read == 0 {
            break; // End of file reached
        }

        // Loop over the buffer, looking for JSON objects
        let mut start_indices = Vec::new();
        let mut end_indices = Vec::new();
        let mut stack = Vec::new();

        let len = buffer.len();
        for i in 0..len {
            // Check for start condition: tab character followed by '{'
            if i < len - 1 && buffer[i] == b'\t' && buffer[i + 1] == b'{' {
                stack.push(i + 1);
            }
            // Check for end condition: '}' followed by newline character
            else if i < len - 1 && buffer[i] == b'}' && buffer[i + 1] == b'\n' {
                if let Some(start_index) = stack.pop() {
                    start_indices.push(start_index);
                    end_indices.push(i);
                }
            }
        }

        // for i in 0..start_indices.len() {
        //     let start_index = start_indices[i];
        //     let end_index = end_indices[i];
        //     let json_string = &buffer[start_index..=end_index];
        //     let json_value: Value =
        //         serde_json::from_slice(json_string).map_err(|e| e.to_string())?;
        //     if let Some(title) = json_value.get("title") {
        //         if let Some(title_str) = title.as_str() {
        //             writer
        //                 .write_all(format!("{}\n", title_str).as_bytes())
        //                 .await?;
        //         }
        //     }
        // }

        // println!("Start indices: {:?}", start_indices);
        // println!("End indices: {:?}", end_indices);

        // let _chunk = &buffer[..bytes_read].find('{');
        // buffer.as_slice().find('{');
        // println!("Read {} bytes", bytes_read);
        // chunk.find
    }

    // let input_buffered_reader = BufReader::with_capacity(buffer_size, input_file);
    // let mut output_buffered_writer = BufWriter::new(output_file);

    // let mut stream = input_buffered_reader.chunks(buffer_size);

    // while let Some(chunk_result) = stream.next().await {
    //     let chunk = chunk_result?.slice();
    //     if let Some(json_string) = chunk.find('{').map(|start_index| &chunk[start_index..]) {
    //         let json_value: Value = serde_json::from_str(json_string).map_err(|e| e.to_string())?;
    //         if let Some(title) = json_value.get("title") {
    //             if let Some(title_str) = title.as_str() {
    //                 output_buffered_writer
    //                     .write_all(format!("{}\n", title_str).as_bytes())
    //                     .await?;
    //             }
    //         }
    //     }
    // }

    // Flush the writer to ensure all output is written to the file
    writer.flush().await?;

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

    // process_input_file_json(input_path, output_path)
    //     .await
    //     .map_err(|err| println!("Error processing file: {}", err))
    //     .ok();

    process_input_file_bytes(input_path, output_path)
        .await
        .map_err(|err| println!("Error processing file: {}", err))
        .ok();

    println!("Processing complete.");

    Ok(())
}
