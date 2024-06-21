/// This converts an integer to binary
pub fn to_binary(value: i64) -> String {
    format!("{:b}", value)
}

/// concerts an integer to hexadecimal
pub fn to_hex(value: i64) -> String {
    format!("{:x}", value)
}
