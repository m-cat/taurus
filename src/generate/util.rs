//! Generation utility functions.

use GameResult;
use database::Database;
use defs::{GameRatio, to_gameratio};
use over::arr::Arr;
use util::rand::rand_ratio;

pub fn pick_obj_from_tup_arr(arr: &Arr) -> GameResult<Database> {
    let mut roll_count = GameRatio::new(0, 1);
    let len = arr.len();
    let roll = rand_ratio(0, 1, 100);

    for i in 0..len {
        let tup = arr.get(i)?.get_tup()?;
        roll_count = to_gameratio(tup.get(1)?.get_frac()?)? + roll_count;
        if roll <= roll_count {
            return Ok(tup.get(0)?.get_obj()?);
        }
    }

    Err(format_err!(
        "Exhausted array of (object, ratio) tuples:\n{}",
        arr
    ))
}
