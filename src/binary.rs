use std::fs;

pub fn load_bin() -> Vec<u16> {
    let bytes = fs::read("resources/challenge.bin").unwrap();
    // Converting to u16 with safe code
    bytes
        .chunks_exact(2)
        .map(|a| u16::from_le_bytes([a[0], a[1]]))
        .collect()
}
