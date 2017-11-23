//! Name generation.

#![allow(unknown_lints)]

use GameResult;
use database::Database;
use defs::{GameRatio, big_to_usize, to_gameratio};
use failure::ResultExt;
use num_traits::One;
use util::rand::{dice, rand_int, rand_ratio};
use util::string;

#[derive(Debug)]
pub struct NameProfile {
    pub min_seqs: usize,
    pub max_seqs: usize,

    pub start_consonants: (String, GameRatio),
    pub start_dconsonants: (String, GameRatio),
    pub start_tconsonants: (String, GameRatio),
    pub mid_consonants: (String, GameRatio),
    pub mid_dconsonants: (String, GameRatio),
    pub mid_tconsonants: (String, GameRatio),
    pub end_consonants: (String, GameRatio),
    pub end_dconsonants: (String, GameRatio),

    pub start_vowels: (String, GameRatio),
    pub mid_vowels: (String, GameRatio),
    pub mid_dvowels: (String, GameRatio),
    pub end_vowels: (String, GameRatio),
    pub end_dvowels: (String, GameRatio),
}

impl NameProfile {
    pub fn new(data: &Database) -> GameResult<NameProfile> {
        fn helper(data: &Database, name: &str) -> GameResult<(String, GameRatio)> {
            let tup = data.get_tup(name)?;
            Ok((
                tup.get(0)?.get_str().context(
                    format!("String for {}", name),
                )?,
                to_gameratio(tup.get(1)?.get_frac()?)?,
            ))
        }

        Ok(NameProfile {
            min_seqs: big_to_usize(data.get_int("min_seqs")?)?,
            max_seqs: big_to_usize(data.get_int("max_seqs")?)?,

            start_consonants: helper(data, "start_consonants")?,
            start_dconsonants: helper(data, "start_dconsonants")?,
            start_tconsonants: helper(data, "start_tconsonants")?,
            mid_consonants: helper(data, "mid_consonants")?,
            mid_dconsonants: helper(data, "mid_dconsonants")?,
            mid_tconsonants: helper(data, "mid_tconsonants")?,
            end_consonants: helper(data, "end_consonants")?,
            end_dconsonants: helper(data, "end_dconsonants")?,

            start_vowels: helper(data, "start_vowels")?,
            mid_vowels: helper(data, "mid_vowels")?,
            mid_dvowels: helper(data, "mid_dvowels")?,
            end_vowels: helper(data, "end_vowels")?,
            end_dvowels: helper(data, "end_dvowels")?,
        })
    }
}

/// Returns a randomly-generated name.
#[allow(collapsible_if)]
pub fn name_gen(profile: &NameProfile, len_limit: usize) -> String {
    let (min_seqs, max_seqs) = (profile.min_seqs, profile.max_seqs);

    // TODO: Throw errors here instead
    assert!(
        max_seqs >= 2,
        "The maximum sequence amount must be greater than 1"
    );
    assert!(len_limit >= 3);
    assert!(
        max_seqs >= min_seqs,
        "The maximum sequence amount must be greater than or equal to the minimum amount"
    );

    let one = GameRatio::one();
    let (ref start_consonants, s_c_ratio) = profile.start_consonants;
    let (ref start_dconsonants, s_dc_ratio) = profile.start_dconsonants;
    let (ref start_tconsonants, s_tc_ratio) = profile.start_tconsonants;
    let (ref mid_consonants, m_c_ratio) = profile.mid_consonants;
    let (ref mid_dconsonants, m_dc_ratio) = profile.mid_dconsonants;
    let (ref mid_tconsonants, m_tc_ratio) = profile.mid_tconsonants;
    let (ref end_consonants, e_c_ratio) = profile.end_consonants;
    let (ref end_dconsonants, e_dc_ratio) = profile.end_dconsonants;

    let (ref start_vowels, s_v_ratio) = profile.start_vowels;
    let (ref mid_vowels, m_v_ratio) = profile.mid_vowels;
    let (ref mid_dvowels, m_dv_ratio) = profile.mid_dvowels;
    let (ref end_vowels, e_v_ratio) = profile.end_vowels;
    let (ref end_dvowels, e_dv_ratio) = profile.end_dvowels;

    // Choose random
    let num_seqs = rand_int(min_seqs, max_seqs);
    let mut word = String::with_capacity(len_limit);

    // Pick starting sequence.
    let mut is_vowel = if dice(1, 3) {
        word.push_str(pick_seq(start_vowels, 1));
        false
    } else {
        let c = match rand_ratio(0, 1, 100) {
            _roll if _roll < s_c_ratio => pick_seq(start_consonants, 1),
            _roll if _roll < s_c_ratio + s_dc_ratio => pick_seq(start_dconsonants, 2),
            _ => pick_seq(start_tconsonants, 3),
        };
        word.push_str(c);
        true
    };
    let mut seqs = 1;

    // Pick middle sequences.
    // Alternate between choosing vowel and consonant sequences.
    while seqs < num_seqs - 1 && word.len() <= len_limit - 3 {
        if is_vowel {
            // Pick vowel sequence
            let c = match rand_ratio(0, 1, 100) {
                _roll if _roll < m_v_ratio => pick_seq(mid_vowels, 1),
                _ => pick_seq(mid_dvowels, 2),
            };
            word.push_str(c);
        } else {
            // Pick consonant sequence
            let c = match rand_ratio(0, 1, 100) {
                _roll if _roll < m_c_ratio => pick_seq(mid_consonants, 1),
                _roll if _roll < m_c_ratio + m_dc_ratio => pick_seq(mid_dconsonants, 2),
                _ => pick_seq(mid_tconsonants, 3),
            };
            word.push_str(c);
        }

        is_vowel = !is_vowel;
        seqs += 1;
    }

    // Pick last sequence.
    if is_vowel {
        // Pick vowel.
        let c = match rand_ratio(0, 1, 100) {
            _roll if _roll < e_v_ratio => pick_seq(end_vowels, 1),
            _ => pick_seq(end_dvowels, 2),
        };
        word.push_str(c);
    } else {
        // Pick consonant.
        let c = match rand_ratio(0, 1, 100) {
            _roll if _roll < e_c_ratio => pick_seq(end_consonants, 1),
            _ => pick_seq(end_dconsonants, 2),
        };
        word.push_str(c);
    }

    string::capitalize(&word)
}

// Helper function for `name_gen`.
fn pick_seq(s: &str, n: usize) -> &str {
    let i = rand_int(0, s.len() / n - 1);
    &s[n * i..n * (i + 1)]
}
