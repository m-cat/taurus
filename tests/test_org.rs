//! Integration tests for [`org`] crate.

extern crate taurus;

use std::fs;
use taurus::org::*;
use taurus::util::file::files_equal;

const TEST_ORG_DIR: &'static str = "tests/files/";
const TEST_FILE_1: &'static str = "test1.org";
const TEST_EXT: &'static str = ".bk";

#[test]
/// Tests processing and writing org files.
fn test_process_org() {
    let fname1 = format!("{}{}", TEST_ORG_DIR, TEST_FILE_1);
    let fname2 = format!("{}{}{}", TEST_ORG_DIR, TEST_FILE_1, TEST_EXT);

    let org = process_org(&fname1).unwrap();

    write_org(&fname2, &org).unwrap();

    // Test that when we process and write back an org file, we get the same result.
    assert!(files_equal(&fname1, &fname2).unwrap());

    fs::remove_file(fname2).unwrap();
}
