//! String utilities.

/// Capitalizes the first letter of a string.
pub fn capitalize(s: &str) -> String {
    let mut res = String::with_capacity(s.len());
    let mut chars = s.chars();

    match chars.next() {
        Some(c) => res.push_str(&c.to_uppercase().to_string()),
        None => return res,
    }
    loop {
        match chars.next() {
            Some(c) => res.push(c),
            None => return res,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_capitalize() {
        assert_eq!("C", capitalize("c"));
        assert_eq!("", capitalize(""));
        assert_eq!("Cap", capitalize("cap"));
        assert_eq!("Yabba dabba", capitalize("yabba dabba"));
        assert_eq!("Doo", capitalize("Doo"));
    }
}
