pub fn Data_Enum_toCharCode(value: char) -> i64 {
    purust_core::purust_char_to_code_unit(value) as i64
}

pub fn Data_Enum_fromCharCode(value: i64) -> char {
    // Match String.fromCharCode's reduction modulo 2^16, including surrogates.
    purust_core::purust_char_from_code_unit(value as u16)
}
