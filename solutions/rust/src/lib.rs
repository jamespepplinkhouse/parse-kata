use serde_json::Value;
use std::error::Error;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;
use std::io::BufWriter;
use std::io::Read;
use std::io::Write;

pub fn process_input_file_bytes(input_path: &str, output_path: &str) -> Result<(), Box<dyn Error>> {
    let input_file = File::open(input_path)?;
    let output_file = File::create(output_path)?;
    let buffer_size = 1 * 1024 * 1024; // 1MB
    let newline = b"\n";

    let mut reader = BufReader::new(input_file);
    let mut writer = BufWriter::new(output_file);
    let mut buffer = vec![0; buffer_size];

    // Read the file in chunks of buffer_size
    let mut last_tail: Option<Vec<u8>> = None;
    while let Ok(bytes_read) = reader.read(&mut buffer) {
        if bytes_read == 0 {
            break; // End of file reached
        }

        // If there is a tail from the last chunk, prepend it to the buffer
        if last_tail.is_some() == true {
            buffer.splice(..0, last_tail.take().unwrap());
        }

        // Find the tail, which is any bytes after the last newline character
        let last_newline_index = find_index_of_last_incomplete_line(&buffer);
        last_tail = match last_newline_index {
            Some(last_newline_index) => Some(buffer[last_newline_index..].to_vec()),
            None => None,
        };

        // If there was a tail in the current buffer, don't process it
        let buffer_without_tail = match last_newline_index {
            Some(last_newline_index) => &buffer[..last_newline_index],
            None => &buffer,
        };

        let titles = extract_titles_from_buffer(&buffer_without_tail);

        for title in titles {
            writer.write(&title)?;
            writer.write(newline)?;
        }
    }

    // Flush the writer to ensure all output is written to the file
    writer.flush()?;

    Ok(())
}

pub fn extract_titles_from_buffer(buffer: &[u8]) -> Vec<Vec<u8>> {
    let mut titles = Vec::new();

    let minimum_json_start = 50;
    let minimum_json_size = 259;
    let title_marker = b"\"title\": \"";
    let unicode_escape_bytes = b"\\u";

    let mut index = 0;

    while index < buffer.len() {
        // Assume that we're starting on a new line
        // Skip minimum_json_start
        index = index + minimum_json_start;
        if index >= buffer.len() {
            break;
        }

        // Store the index of the opening curly brace
        let json_start = buffer[index..].iter().position(|&b| b == b'{');
        if json_start.is_none() {
            break;
        }

        // Move the index forward to the start of the JSON
        index = index + json_start.unwrap();
        // println!(
        //     "index: {:?}",
        //     std::str::from_utf8(&buffer[index..index + 32])
        // );

        // Find the first title (TBC - need analysis)
        let title_marker_index = buffer[index..]
            .windows(title_marker.len())
            .position(|window| window == title_marker);
        if title_marker_index.is_none() {
            break;
        }

        // Find the start and end of the title value
        let title_start = index + title_marker_index.unwrap() + title_marker.len();
        let title_end_index_from_title_start = find_unescaped_double_quote(&buffer[title_start..]);
        if title_end_index_from_title_start.is_none() {
            break;
        }
        let title_end = title_start + title_end_index_from_title_start.unwrap();
        // println!(
        //     "title: {:?}",
        //     std::str::from_utf8(&buffer[title_start..title_end])
        // );

        // Extract, decode, and store the title value
        let json_title_bytes = &buffer[title_start..title_end];
        if json_title_bytes
            .windows(unicode_escape_bytes.len())
            .any(|window| window == unicode_escape_bytes)
        {
            // Serde - JSON unicode escape sequences are decoded
            let json_title_including_double_quotes = &buffer[title_start - 1..title_end + 1];
            let title: String =
                serde_json::from_slice(json_title_including_double_quotes).unwrap_or(String::new());
            titles.push(title.into_bytes());
        } else {
            titles.push(json_title_bytes.to_vec());
        }

        // Skip to the minimum possible end of the JSON
        index = index + minimum_json_size;
        if index >= buffer.len() {
            break;
        }
        // println!(
        //     "index: {:?}",
        //     std::str::from_utf8(&buffer[index..index + 32])
        // );

        // Move the index to the start of the next line
        let next_line_start = buffer[index..].iter().position(|&b| b == b'\n');
        if next_line_start.is_none() {
            break;
        }
        index = index + next_line_start.unwrap() + 1;
        // println!(
        //     "index: {:?}",
        //     std::str::from_utf8(&buffer[index..index + 32])
        // );
    }

    titles
}

pub fn find_index_of_last_incomplete_line(buffer: &Vec<u8>) -> Option<usize> {
    match buffer.iter().rposition(|&x| x == b'\n' || x == b'}') {
        Some(index) => Some(index + 1),
        None => None,
    }
}

pub fn find_unescaped_double_quote(buffer: &[u8]) -> Option<usize> {
    let mut previous_was_backslash = false;

    for (i, &byte) in buffer.iter().enumerate() {
        match byte {
            b'\\' => previous_was_backslash = true,
            b'"' if !previous_was_backslash => return Some(i),
            _ => previous_was_backslash = false,
        }
    }

    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_index_of_last_incomplete_line_with_newline() {
        let buffer: Vec<u8> = b"Hello\nWorld".to_vec();
        let index = find_index_of_last_incomplete_line(&buffer);
        assert_eq!(index, Some(6));
    }

    #[test]
    fn test_find_index_of_last_incomplete_line_with_closing_brace() {
        let buffer: Vec<u8> = b"}, \"field\": 100".to_vec();
        let index = find_index_of_last_incomplete_line(&buffer);
        assert_eq!(index, Some(1));
    }

    #[test]
    fn test_find_last_newline_index_none() {
        let buffer: Vec<u8> = b"No newline here".to_vec();
        let index = find_index_of_last_incomplete_line(&buffer);
        assert_eq!(index, None);
    }

    #[test]
    fn find_unescaped_double_quote_finds_unescaped_double_quote() {
        let text = r#"Hello \"World\""!"#;
        let buffer = text.as_bytes();
        assert_eq!(find_unescaped_double_quote(buffer), Some(15));
    }

    #[test]
    fn find_unescaped_double_quote_returns_none_when_no_unescaped_double_quote() {
        let text = r#"Hello \\\"World\\\"!"#;
        let buffer = text.as_bytes();
        assert_eq!(find_unescaped_double_quote(buffer), None);
    }

    #[test]
    fn find_unescaped_double_quote_returns_none_when_no_double_quote_at_all() {
        let text = r#"Hello World!"#;
        let buffer = text.as_bytes();
        assert_eq!(find_unescaped_double_quote(buffer), None);
    }

    #[test]
    fn find_unescaped_double_quote_handles_empty_buffer() {
        let buffer: [u8; 0] = [];
        assert_eq!(find_unescaped_double_quote(&buffer), None);
    }
}

pub fn process_input_file_json(input_path: &str, output_path: &str) -> Result<(), Box<dyn Error>> {
    let input_file = File::open(input_path)?;
    let output_file = File::create(output_path)?;
    let buffer_size = 1 * 1024 * 1024; // 1MB
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
