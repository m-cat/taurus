//! Conversion utility functions.

/// Converts a hex char to a u8.
pub fn hex_char_to_int(ch: char) -> Option<u8> {
    ch.to_digit(16).map(|i| i as u8)
}

/// Converts a hex string to a u32.
/// The string can optionally start with '#'.
pub fn hex_str_to_int(s: &str) -> Option<u32> {
    let mut chars = s.chars();

    let mut res = match chars.next() {
        Some('#') => {
            match chars.next() {
                Some(ch) => hex_char_to_int(ch)?,
                _ => return None,
            }
        }
        Some(ch) => hex_char_to_int(ch)?,
        _ => return None,
    } as u32;
    let mut c = 1;

    loop {
        if c >= 8 {
            return None;
        }

        res = res * 16 +
            match chars.next() {
                Some(ch) => hex_char_to_int(ch)? as u32,
                None => return Some(res),
            };
        c += 1;
    }
}

/// Converts a 6-digit hex color code to a (r, g, b) tuple of u8's.
/// The string can optionally start with '#'.
pub fn color_code_to_rgb(s: &str) -> Option<(u8, u8, u8)> {
    let mut chars = s.chars();

    let mut r = match chars.next() {
        Some('#') => {
            match chars.next() {
                Some(ch) => hex_char_to_int(ch)?,
                _ => return None,
            }
        }
        Some(ch) => hex_char_to_int(ch)?,
        _ => return None,
    };

    r = r * 16 +
        match chars.next() {
            Some(ch) => hex_char_to_int(ch)?,
            _ => return None,
        };

    let mut g = match chars.next() {
        Some(ch) => hex_char_to_int(ch)?,
        _ => return None,
    };
    g = g * 16 +
        match chars.next() {
            Some(ch) => hex_char_to_int(ch)?,
            _ => return None,
        };

    let mut b = match chars.next() {
        Some(ch) => hex_char_to_int(ch)?,
        _ => return None,
    };
    b = b * 16 +
        match chars.next() {
            Some(ch) => hex_char_to_int(ch)?,
            _ => return None,
        };

    if let Some(_) = chars.next() {
        return None;
    }

    Some((r, g, b))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hex_char_to_int() {
        assert_eq!(hex_char_to_int('0'), Some(0));
        assert_eq!(hex_char_to_int('a'), Some(10));
        assert_eq!(hex_char_to_int('F'), Some(15));
        assert_eq!(hex_char_to_int('G'), None);
        assert_eq!(hex_char_to_int('#'), None);
    }

    #[test]
    fn test_hex_str_to_int() {
        assert_eq!(hex_str_to_int("#FF"), Some(255));
        assert_eq!(hex_str_to_int("FF"), Some(255));
        assert_eq!(hex_str_to_int("#0"), Some(0));
        assert_eq!(hex_str_to_int("0"), Some(0));

        assert_eq!(hex_str_to_int("G"), None);
        assert_eq!(hex_str_to_int("#"), None);
    }

    #[test]
    fn test_color_code_to_rgb() {
        let (r, g, b) = color_code_to_rgb("#ff00ff").unwrap();
        assert_eq!(r, 255);
        assert_eq!(g, 0);
        assert_eq!(b, 255);

        let (r, g, b) = color_code_to_rgb("0f0f00").unwrap();
        assert_eq!(r, 15);
        assert_eq!(g, 15);
        assert_eq!(b, 0);

        assert_eq!(color_code_to_rgb("#FF00"), None);
        assert_eq!(color_code_to_rgb("FF00FFF"), None);
        assert_eq!(color_code_to_rgb("FF00GG"), None);
    }
}
