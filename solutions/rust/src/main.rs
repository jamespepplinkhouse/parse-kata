use boyer_moore_magiclen::BMByte;
use serde_json::Value;
use std::env;
use std::error::Error;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;
use std::io::BufWriter;
use std::io::Read;
use std::io::Write;

fn process_input_file_json(input_path: &str, output_path: &str) -> Result<(), Box<dyn Error>> {
    let input_file = File::open(input_path)?;
    let output_file = File::create(output_path)?;
    let buffer_size = 100 * 1024 * 1024; // 100MB
    let input_buffered_reader = BufReader::with_capacity(buffer_size, input_file);
    let mut output_buffered_writer = BufWriter::new(output_file);

    let mut line_stream = input_buffered_reader.lines();

    while let Some(line_result) = line_stream.next() {
        let line = line_result?;
        if let Some(json_string) = line.find('{').map(|start_index| &line[start_index..]) {
            let json_value: Value = serde_json::from_str(json_string).map_err(|e| e.to_string())?;
            if let Some(title) = json_value.get("title") {
                if let Some(title_str) = title.as_str() {
                    output_buffered_writer.write_all(format!("{}\n", title_str).as_bytes())?;
                }
            }
        }
    }

    // Flush the writer to ensure all output is written to the file
    output_buffered_writer.flush()?;

    Ok(())
}

fn process_input_file_bytes(input_path: &str, output_path: &str) -> Result<(), Box<dyn Error>> {
    let input_file = File::open(input_path)?;
    let output_file = File::create(output_path)?;
    let buffer_size = 100 * 1024 * 1024; // 100MB

    let title_len = b"\"title\": \"".len();
    let bmb_title = BMByte::from("\"title\": \"").unwrap();
    let quote_bytes = b"\"";
    let newline_bytes = b"\n";
    let bmb_newline = BMByte::from("\n").unwrap();

    let mut reader = BufReader::new(input_file);
    let mut writer = BufWriter::new(output_file);
    let mut buffer = vec![0; buffer_size];

    // Read the file in chunks of buffer_size
    let mut last_tail: Option<Vec<u8>> = None;
    while let Ok(bytes_read) = reader.read(&mut buffer) {
        if bytes_read == 0 {
            break; // End of file reached
        }

        if last_tail.is_some() {
            // If there is a tail from the last chunk, prepend it to the buffer
            buffer.splice(..0, last_tail.take().unwrap());
        }

        // Find the tail, which is any bytes after the last newline character
        let last_newline_index = match bmb_newline.rfind_first_in(&buffer) {
            Some(index) => Some(index + 1),
            None => None,
        };

        last_tail = match last_newline_index {
            Some(last_newline_index) => Some(buffer[last_newline_index..].to_vec()),
            None => None,
        };

        // Boyer-Moore-MagicLen, a fast string search algorithm
        let title_indexes: Vec<usize> = bmb_title.find_in(&buffer, 0);

        // Do this part in parallel?
        // Or move this to another thread and move on
        for title_index in title_indexes {
            let title_start_index = title_index + title_len;
            let title_end_index = buffer[title_start_index..]
                .iter()
                .position(|&b| b == quote_bytes[0])
                .map(|end_index| title_start_index + end_index)
                .unwrap_or(buffer.len());

            let title_bytes = &buffer[title_start_index..title_end_index];
            writer.write(title_bytes)?;
            writer.write(newline_bytes.as_slice())?;
        }
    }

    // Flush the writer to ensure all output is written to the file
    writer.flush()?;

    Ok(())
}

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
