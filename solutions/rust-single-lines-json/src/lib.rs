use memchr::memmem;
use memmap::MmapOptions;
use std::error::Error;
use std::fs::File;
use std::io::{BufWriter, Write};

pub fn process_input_file_json(
    input_file_path: &str,
    output_file_path: &str,
) -> Result<(), Box<dyn Error>> {
    // Input file
    let input_file = File::open(input_file_path)?;
    let input_memmap = unsafe { MmapOptions::new().map(&input_file)? };

    // Output file
    let output_file = File::create(output_file_path)?;
    let mut output_buffered_writer = BufWriter::new(output_file);

    // Buffer for each line
    // let mut line_buffer = Vec::new();

    let title_marker = b"\"title\": \"";
    let title_finder = memmem::Finder::new(title_marker);
    // let newline_finder = memmem::Finder::new(b"\n");

    // let chunk_size = 1024 * 1024 * 1;

    title_finder
        .find_iter(&input_memmap)
        .for_each(|title_marker_index| {
            // println!("Found title marker at {}", title_marker_index);
        });

    // input_memmap.chunks(chunk_size).for_each(|chunk| {
    //     println!("Processing chunk of size {}", chunk.len());
    //     title_finder
    //         .find_iter(chunk)
    //         .for_each(|title_marker_index| {
    //             println!("Found title marker at {}", title_marker_index);
    //             // // Find the start index of the title value
    //             // let title_start = title_marker_index + title_marker.len();

    //             // // Find the end index of the title value
    //             // let title_end_index_from_title_start =
    //             //     find_unescaped_double_quote(&chunk[title_start..]);
    //             // if title_end_index_from_title_start.is_none() {
    //             //     println!("No end quote found");
    //             //     return;
    //             // }
    //             // let title_end = title_start + title_end_index_from_title_start.unwrap();

    //             // // The title in raw bytes, we need to check if there are any encoded characters
    //             // let title_raw = &chunk[title_start..title_end];

    //             // let found_encoded_bytes =
    //             //     title_raw.iter().any(|&byte| byte == b'"' || byte == b'\\');

    //             // // If we found any encoded bytes, use simd_json to safely decode the JSON string value
    //             // if found_encoded_bytes {
    //             //     let mut cloned_title = chunk[title_start - 1..title_end + 1].to_vec();
    //             //     let title = simd_json::from_slice(&mut cloned_title).unwrap_or(String::new());
    //             //     let _ = output_buffered_writer.write(format!("{}\n", title).as_bytes());
    //             // } else {
    //             //     let _ = output_buffered_writer.write(title_raw);
    //             //     let _ = output_buffered_writer.write(&[b'\n']);
    //             // }
    //         });
    // });

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
