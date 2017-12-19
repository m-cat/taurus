//! Name generation.

#![allow(unknown_lints)]

use GameResult;
use database::Database;
use defs::{GameRatio, big_to_usize, to_gameratio};
use failure::ResultExt;
use num_traits::One;
use util::rand::*;
use util::string;

/// Returns a randomly-generated name.
#[allow(collapsible_if)]
pub fn name_gen(profile: &Database, len_limit: usize) -> GameResult<String> {
    fn helper(profile: &Database, name: &str) -> GameResult<(String, GameRatio)> {
        let tup = profile.get_tup(name)?;
        Ok((
            tup.get(0)?.get_str().context(
                format!("String for {}", name),
            )?,
            to_gameratio(tup.get(1)?.get_frac()?)?,
        ))
    }

    let min_seqs = big_to_usize(profile.get_int("min_seqs")?)?;
    let max_seqs = big_to_usize(profile.get_int("max_seqs")?)?;

    let vowel_start = to_gameratio(profile.get_frac("vowel_start")?)?;

    let (ref start_consonants, s_c_ratio) = helper(profile, "start_consonants")?;
    let (ref start_dconsonants, s_dc_ratio) = helper(profile, "start_dconsonants")?;
    let (ref start_tconsonants, s_tc_ratio) = helper(profile, "start_tconsonants")?;
    let (ref mid_consonants, m_c_ratio) = helper(profile, "mid_consonants")?;
    let (ref mid_dconsonants, m_dc_ratio) = helper(profile, "mid_dconsonants")?;
    let (ref mid_tconsonants, m_tc_ratio) = helper(profile, "mid_tconsonants")?;
    let (ref end_consonants, e_c_ratio) = helper(profile, "end_consonants")?;
    let (ref end_dconsonants, e_dc_ratio) = helper(profile, "end_dconsonants")?;

    let (ref start_vowels, s_v_ratio) = helper(profile, "start_vowels")?;
    let (ref mid_vowels, m_v_ratio) = helper(profile, "mid_vowels")?;
    let (ref mid_dvowels, m_dv_ratio) = helper(profile, "mid_dvowels")?;
    let (ref end_vowels, e_v_ratio) = helper(profile, "end_vowels")?;
    let (ref end_dvowels, e_dv_ratio) = helper(profile, "end_dvowels")?;

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

    // Choose random
    let num_seqs = rand_int(min_seqs, max_seqs);
    let mut word = String::with_capacity(len_limit);

    // Pick starting sequence.
    let mut is_vowel = if chance(vowel_start) {
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

    Ok(string::capitalize(&word))
}

// Helper function for `name_gen`.
fn pick_seq(s: &str, n: usize) -> &str {
    let i = rand_int(0, s.len() / n - 1);
    &s[n * i..n * (i + 1)]
}
