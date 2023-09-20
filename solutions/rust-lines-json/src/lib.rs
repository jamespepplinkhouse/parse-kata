use simd_json::borrowed::Value;
use simd_json::ValueAccess;
use std::error::Error;
use tokio::fs::File;
use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader, BufWriter};

pub async fn process_input_file_json(
    input_path: &str,
    output_path: &str,
) -> Result<(), Box<dyn Error>> {
    let input_file = File::open(input_path).await?;
    let output_file = File::create(output_path).await?;
    let buffer_size = 1 * 1024 * 1024; // 1MB
    let input_buffered_reader = BufReader::with_capacity(buffer_size, input_file);
    let mut output_buffered_writer = BufWriter::new(output_file);

    let mut line_stream = input_buffered_reader.lines();

    while let Ok(Some(line)) = line_stream.next_line().await {
        if let Some(start_index) = line.find('{') {
            let mut json_string = String::from(&line[start_index..]);
            let json_value: Result<Value, _>;

            unsafe {
                json_value = simd_json::from_str(json_string.as_mut_str());
            }

            if let Ok(json_value) = json_value {
                if let Some(title) = json_value.get("title") {
                    if let Some(title_str) = title.as_str() {
                        output_buffered_writer
                            .write_all(format!("{}\n", title_str).as_bytes())
                            .await?;
                    }
                }
            }
        }
    }

    // Flush the writer
    output_buffered_writer.flush().await?;

    Ok(())
}

#[cfg(test)]
mod tests {
    #[test]
    fn the_truth() {
        assert_eq!(true, true);
    }
}
