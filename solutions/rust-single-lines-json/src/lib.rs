use simd_json::borrowed::Value;
use simd_json::ValueAccess;
use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader, BufWriter, Write};

pub fn process_input_file_json(
    input_file_path: &str,
    output_file_path: &str,
) -> Result<(), Box<dyn Error>> {
    // Input file
    let input_file = File::open(input_file_path)?;
    let mut reader = BufReader::new(input_file);

    // Output file
    let output_file = File::create(output_file_path)?;
    let mut output_buffered_writer = BufWriter::new(output_file);

    // Buffer for each line
    let mut line_buffer = Vec::new();

    loop {
        line_buffer.clear();

        // Read line into buffer
        let bytes_read = reader.read_until(b'\n', &mut line_buffer)?;

        // Exit loop if EOF
        if bytes_read == 0 {
            break;
        }

        // Find the start of the JSON document
        let json_start_index = line_buffer.iter().position(|&x| x == b'{');

        let json_value: Result<Value, _>;
        if let Some(json_start_index) = json_start_index {
            json_value = simd_json::from_slice(line_buffer[json_start_index..].as_mut());

            if let Some(title_str) = json_value.get("title").and_then(|t| t.as_str()) {
                output_buffered_writer.write(format!("{}\n", title_str).as_bytes())?;
            }
        }
    }

    // Flush the writer
    output_buffered_writer.flush()?;

    Ok(())
}

#[cfg(test)]
mod tests {
    #[test]
    fn the_truth() {
        assert_eq!(true, true);
    }
}
