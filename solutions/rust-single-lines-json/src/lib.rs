use simd_json::borrowed::Value;
use simd_json::ValueAccess;
use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader, BufWriter, Write};
use std::sync::mpsc::channel;
use std::thread;

pub fn process_input_file_json(
    input_file_path: &str,
    output_file_path: &str,
) -> Result<(), Box<dyn Error>> {
    let (tx, rx) = channel::<Vec<u8>>();
    let output_path_clone = output_file_path.to_owned(); // Clone for the worker thread

    // Worker thread for JSON parsing and title extraction
    let handle = thread::spawn(move || -> Result<(), Box<std::io::Error>> {
        let output_file = File::create(output_path_clone)?;
        let mut output_buffered_writer = BufWriter::new(output_file);
        for mut json_data in rx.iter() {
            let parsed: Result<Value, _> = simd_json::from_slice(&mut json_data);
            if let Ok(json_value) = parsed {
                if let Some(title_str) = json_value.get("title").and_then(|t| t.as_str()) {
                    output_buffered_writer.write(format!("{}\n", title_str).as_bytes())?;
                }
            }
        }
        // Flush the writer
        output_buffered_writer.flush()?;
        Ok(())
    });

    // Main thread for reading input file
    let input_file = File::open(input_file_path)?;
    let mut reader = BufReader::new(input_file);
    let mut line_buffer = Vec::new();

    loop {
        line_buffer.clear();

        let bytes_read = reader.read_until(b'\n', &mut line_buffer)?;
        if bytes_read == 0 {
            break;
        }

        if let Some(json_start_index) = line_buffer.iter().position(|&x| x == b'{') {
            tx.send(line_buffer[json_start_index..].to_vec()).unwrap();
        }
    }

    drop(tx); // Close the channel
    handle.join().unwrap()?; // Wait for worker thread to finish
    Ok(())
}

#[cfg(test)]
mod tests {
    #[test]
    fn the_truth() {
        assert_eq!(true, true);
    }
}
