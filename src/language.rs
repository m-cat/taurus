use utility::{dice, rand_range};

/// Capitalize the first letter of the string
pub fn cap(s: &String) -> String {
    debug_assert!(s.len() > 0, format!("Assert failed: cap({})", s));
    let c = &s[0..1];
    format!("{}{}", c.to_uppercase(), &s[1..])
}

/// Return a randomly-generated name
pub fn name_gen(max_len: usize) -> String {
    // Define the list of consonant, double consonant, etc. sequences
    let consonants = "bdfgklmnprsstvx";
    let start_consonants = "bdfghklmnprsstvwjz";
    let end_consonants = "bdfghklnprsstx";
    let dconsonants = "brchcttrthghdrmmttllstqu";
    let start_dconsonants = "brchcltrthdrslphblcrfrgrstquvrwhwr";
    let end_dconsonants = "chthghstrm";
    let tconsonants = "chrstrthr";
    let vowels = "aaeeiioouy";
    let end_vowels = "aio";
    let dvowels = "ioiaai";
    let end_dvowels = "ia";

    let mut word = String::with_capacity(max_len);

    // Pick starting sequence
    let mut is_vowel = dice(1, 3); // start with vowel?
    if is_vowel {
        word.push_str(pick_seq(&vowels, 1));
    } else {
        let c = match rand_range(1, 100) {
            1...40 => pick_seq(&start_consonants, 1),
            41...80 => pick_seq(&start_dconsonants, 2),
            _ => pick_seq(&tconsonants, 3),
        };
        word.push_str(c);
    }

    // Alternate between choosing vowel and consonant sequences
    let mut m = rand_range(2, 5);
    while m > 0 && word.len() < max_len {
        m -= 1;
        is_vowel = !is_vowel;
        if is_vowel {
            // Pick vowel sequence
            if m == 0 {
                // Last sequence
                let c = match rand_range(1, 100) {
                    1...80 => pick_seq(&end_vowels, 1),
                    _ => pick_seq(&end_dvowels, 2),
                };
                word.push_str(c);
            } else {
                let c = match rand_range(1, 100) {
                    1...80 => pick_seq(&vowels, 1),
                    _ => pick_seq(&dvowels, 2),
                };
                word.push_str(c);
            }
        } else {
            // Pick consonant sequence
            if m == 0 {
                // Last sequence
                let c = match rand_range(1, 100) {
                    1...60 => pick_seq(&end_consonants, 1),
                    _ => pick_seq(&end_dconsonants, 2),
                };
                word.push_str(c);
            } else {
                // Middle sequence
                let c = match rand_range(1, 100) {
                    1...60 => pick_seq(&consonants, 1),
                    61...90 => pick_seq(&dconsonants, 2),
                    _ => pick_seq(&tconsonants, 3),
                };
                word.push_str(c);
            }
        }
    }

    cap(&word)
}

fn pick_seq(s: &str, n: usize) -> &str {
    let i = rand_range(0, s.len() / n - 1);
    &s[n * i..n * (i + 1)]
}

/// Language unit tests
#[cfg(test)]
mod tests {
    use std::str::*;
    use language::*;

    #[test]
    fn test_cap() {
        let str1 = String::from_str("cap").unwrap();
        let str2 = String::from_str("yabba dabba doo").unwrap();

        assert_eq!("Cap", cap(&str1));
        assert_eq!("Yabba dabba doo", cap(&str2));
    }
}
