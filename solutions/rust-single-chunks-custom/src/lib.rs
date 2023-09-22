use memchr::memmem;
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

    let title_marker = b"\"title\": \"";
    let finder = memmem::Finder::new(title_marker);

    loop {
        line_buffer.clear();

        // Read line into buffer
        let bytes_read = reader.read_until(b'\n', &mut line_buffer)?;

        // Exit loop if EOF
        if bytes_read == 0 {
            break;
        }

        // Find the start index of the title value
        let maybe_title_marker_index = finder.find(line_buffer.as_slice());

        // If we didn't find a title, bail out for next loop
        if maybe_title_marker_index.is_none() {
            continue;
        }

        let title_start = maybe_title_marker_index.unwrap() + title_marker.len();

        // Find the end index of the title value
        let title_end_index_from_title_start =
            find_unescaped_double_quote(&line_buffer[title_start..]);
        if title_end_index_from_title_start.is_none() {
            continue;
        }
        let title_end = title_start + title_end_index_from_title_start.unwrap();

        // The title in raw bytes, we need to check if there are any encoded characters
        let title_raw = &line_buffer[title_start..title_end];

        let found_encoded_bytes = title_raw.iter().any(|&byte| byte == b'"' || byte == b'\\');

        // If we found any encoded bytes, use simd_json to safely decode the JSON string value
        if found_encoded_bytes {
            let title = simd_json::from_slice(&mut line_buffer[title_start - 1..title_end + 1])
                .unwrap_or(String::new());
            output_buffered_writer.write(format!("{}\n", title).as_bytes())?;
        } else {
            output_buffered_writer.write(title_raw)?;
            output_buffered_writer.write(&[b'\n'])?;
        }
    }

    // Flush the writer
    output_buffered_writer.flush()?;

    Ok(())
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
