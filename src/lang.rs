#![allow(unknown_lints)]

use util::rand::{dice, rand_range};

/// Capitalizes the first letter of a string.
pub fn cap(s: &str) -> String {
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

/// Returns a randomly-generated name.
#[allow(collapsible_if)]
pub fn name_gen(min_len: usize, max_len: usize) -> String {
    debug_assert!(max_len >= 3, "The maximum length must be greater than 3.");
    debug_assert!(
        max_len >= min_len,
        "The maximum length must be greater than the minimum length"
    );

    // Define the list of consonant, double consonant, etc. sequences
    let consonants = "bdfgklmnprsstvx";
    let start_consonants = "bdfghklmnprsstvwjz";
    let end_consonants = "bdfghklnprsstx";
    let dconsonants = "brchcttrthghdrmmttllstqu";
    let start_dconsonants = "brchcltrthdrslphblcrfrgrstquvrwhwr";
    let end_dconsonants = "chghrmstth";
    let tconsonants = "chrstrthr";
    let vowels = "aaeeiioouy";
    let end_vowels = "aou";
    let dvowels = "iaio";
    let end_dvowels = "ia";

    let len = rand_range(min_len, max_len);
    let mut word = String::with_capacity(len);

    // Pick starting sequence.
    let mut is_vowel = if dice(1, 3) {
        word.push_str(pick_seq(vowels, 1));
        false
    } else {
        let c = match rand_range(1, 100) {
            1...40 => pick_seq(start_consonants, 1),
            41...80 => pick_seq(start_dconsonants, 2),
            _ => pick_seq(tconsonants, 3),
        };
        word.push_str(c);
        true
    };

    // Pick middle sequences.
    // Alternate between choosing vowel and consonant sequences.
    while word.len() <= len - 3 {
        if is_vowel {
            // Pick vowel sequence
            let c = match rand_range(1, 100) {
                1...85 => pick_seq(vowels, 1),
                _ => pick_seq(dvowels, 2),
            };
            word.push_str(c);
        } else {
            // Pick consonant sequence
            let c = match rand_range(1, 100) {
                1...60 => pick_seq(consonants, 1),
                61...90 => pick_seq(dconsonants, 2),
                _ => pick_seq(tconsonants, 3),
            };
            word.push_str(c);
        }

        is_vowel = !is_vowel;
    }

    // Pick last sequence.
    if is_vowel {
        // Pick vowel.
        let c = match rand_range(1, 100) {
            1...90 => pick_seq(end_vowels, 1),
            _ => pick_seq(end_dvowels, 2),
        };
        word.push_str(c);
    } else {
        // Pick consonant.
        let c = match rand_range(1, 100) {
            1...60 => pick_seq(end_consonants, 1),
            _ => pick_seq(end_dconsonants, 2),
        };
        word.push_str(c);
    }

    cap(&word)
}

// Helper function for `name_gen`.
fn pick_seq(s: &str, n: usize) -> &str {
    let i = rand_range(0, s.len() / n - 1);
    &s[n * i..n * (i + 1)]
}

#[cfg(test)]
mod tests {
    use lang::*;

    #[test]
    fn test_cap() {
        assert_eq!("C", cap("c"));
        assert_eq!("", cap(""));
        assert_eq!("Cap", cap("cap"));
        assert_eq!("Yabba dabba", cap("yabba dabba"));
    }

    #[test]
    fn test_name_gen() {
        for n in 1..1000 {
            let min = rand_range(3, 5);
            let max = rand_range(6, 40);
            let name = name_gen(min, max);

            assert!(name.len() >= min);
            assert!(name.len() <= max);
        }
    }
}
