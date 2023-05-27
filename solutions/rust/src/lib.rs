pub fn find_index_of_last_incomplete_line(buffer: &Vec<u8>) -> Option<usize> {
    match buffer.iter().rposition(|&x| x == b'\n' || x == b'}') {
        Some(index) => Some(index + 1),
        None => None,
    }
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
}
