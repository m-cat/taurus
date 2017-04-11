extern crate taurus;

use taurus::org::*;
use taurus::util::*;

use std::fs;

const TEST_ORG_DIR: &'static str = "tests/files";
const TEST_EXT: &'static str = ".bk";
const TEST_FILE_1: &'static str = "test1.org";

#[test]
fn test_process_org() {
    let path1 = make_path_dir(TEST_ORG_DIR, TEST_FILE_1);
    let path2 = make_path_dir_cat(TEST_ORG_DIR, TEST_FILE_1, TEST_EXT);

    let org = process_org(&path1).unwrap();

    write_org(&path2, &org).unwrap();

    assert!(files_equal(&path1, &path2).unwrap());

    fs::remove_file(path2).unwrap();
}
