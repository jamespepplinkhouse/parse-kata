use serde_json::Value;
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

#[cfg(test)]
mod tests {
    #[test]
    fn the_truth() {
        assert_eq!(true, true);
    }
}
