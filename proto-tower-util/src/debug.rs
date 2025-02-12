/// Display the data in hex format
/// with rows of 16 bytes (2 columns of 8 bytes)
pub fn debug_hex(data: &[u8]) -> String {
    let mut result = String::new();
    // Print header
    result.push_str("    0  1  2  3  4  5  6  7   8  9  a  b  c  d  e  f\n");
    for (row_num, chunk) in data.chunks(16).enumerate() {
        if row_num > 0 {
            result.push('\n');
        }
        result.push_str(&format!("{:02x} ", (row_num % 16) * 16));
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
