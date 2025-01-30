/// Display the data in hex format
/// with rows of 16 bytes (2 columns of 8 bytes)
pub fn debug_hex(data: &[u8]) -> String {
    let mut result = String::new();
    for (row_num, chunk) in data.chunks(16).enumerate() {
        if row_num > 0 {
            result.push('\n');
        }
        for (col_num, byte) in chunk.iter().enumerate() {
            if col_num > 0 {
                result.push(' ');
            }
            if col_num == 8 {
                result.push(' ');
            }
            result.push_str(&format!("{:02x}", byte));
        }
    }
    result
}
