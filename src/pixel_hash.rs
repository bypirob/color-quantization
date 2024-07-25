pub fn get_pixel_hash(rgb: &[u8], depth: i8) -> String {
    let mut pixel_hash = String::new();
    for i in 0..depth {
        let r_bit = (rgb[0] >> (7 - i)) & 1;
        let g_bit = (rgb[1] >> (7 - i)) & 1;
        let b_bit = (rgb[2] >> (7 - i)) & 1;

        // Combine these bits into a single value
        let combined_bits = (r_bit << 2) | (g_bit << 1) | b_bit;

        pixel_hash.push_str(&combined_bits.to_string());
    }
    // println!("PIXEL {}", pixel_hash);

    return pixel_hash;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_pixel_hash() {
        // Test case 1
        let rgb = [255, 255, 255];
        let depth = 3;
        let expected_hash = "777";
        assert_eq!(get_pixel_hash(&rgb, depth), expected_hash);

        // Test case 2
        let rgb = [0, 0, 0];
        let depth = 3;
        let expected_hash = "000";
        assert_eq!(get_pixel_hash(&rgb, depth), expected_hash);

        // Test case 3
        let rgb = [1, 1, 1];
        let depth = 3;
        let expected_hash = "000";
        assert_eq!(get_pixel_hash(&rgb, depth), expected_hash);

        // Test case 4
        let rgb = [170, 170, 170]; // Binary: 10101010
        let depth = 5;
        let expected_hash = "70707";
        assert_eq!(get_pixel_hash(&rgb, depth), expected_hash);

        // Test case 5
        let rgb = [170, 85, 255]; // Binary: 10101010, 01010101, 11111111
        let depth = 8;
        let expected_hash = "53535353";
        assert_eq!(get_pixel_hash(&rgb, depth), expected_hash);
    }
}
