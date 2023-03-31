use std::env;
use std::fmt::format;
use std::fs::File;
use std::io::prelude::*;
use std::path::PathBuf;
use paste::paste;

use super::*;
use crate::mods::{heap_binary::*, std_heaps::*, std_vecs::*};
use crate::{correct_order, tasks};

// test data is parsed only once
lazy_static! {
    #[derive(Debug)]
    static ref TEST_DATA: Vec<TestData> = parse_test_file("schr.data").unwrap();
}

#[derive(Debug, Default)]
struct TestData {
    data_name: String,
    data_size: usize,
    data: Vec<Task>,
    order: Vec<Task>,
    cmax: u32,
}

struct ResultData {
    cmax: u32,
    order: Option<Vec<Task>>,
}

fn parse_test_file(filename: &str) -> Option<Vec<TestData>> {
    let mut file_path: PathBuf = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    file_path.push("src/tests");
    file_path.push(&filename);
    let mut file = File::open(&file_path).map_err(|e| {
        eprintln!("Error opening test case file {}: {e}", &file_path.display())
    }).unwrap();
    let mut buf = String::new();
    file.read_to_string(&mut buf).unwrap();

    let mut dv: Vec<TestData> = Vec::new();
    let mut lines = buf.lines();
    while let Some(l) = lines.next() {
        if l.is_empty() {
            continue;
        }
        if l[0..1] == *"d" {
            let mut data = TestData::default();
            data.data_name = l.to_owned();
            if let Some(l) = lines.next() {
                data.data_size = l.parse().unwrap();
                data.data = Vec::with_capacity(data.data_size);
            }
            while let Some(l) = lines.next() {
                if l.is_empty() {
                    break;
                }
                let mut t = l.split(" ").map(|w| w.parse::<u32>().unwrap());
                data.data.push(Task {
                    r: t.next().unwrap(),
                    p: t.next().unwrap(),
                    q: t.next().unwrap(),
                });
            }
            while let Some(l) = lines.next() {
                if l == "schr:" {
                    break;
                }
            }
            if let Some(l) = lines.next() {
                data.cmax = l.parse::<u32>().unwrap();
            }
            if let Some(mut l) = lines.next() {
                if let Some(l2) = lines.next() {
                    if !l2.is_empty() {
                        let l = &format!("{}{}", l, l2);
                    }
                }
                data.order = l
                    .split(" ")
                    .map(|pos| data.data[pos.parse::<usize>().unwrap() - 1])
                    .collect();
            }
            dv.push(data);
        }
    }
    if !dv.is_empty() {
        return Some(dv);
    }
    None
}

#[test]
fn parsed_test() {
    assert_eq!(TEST_DATA.len(), 8);
}

macro_rules! generate_algorithm_tests {
    ($test_name:ident, $alg_func:ident, $idx:expr) => {
        #[test]
        fn $test_name() {
            let cmax = $alg_func(TEST_DATA[$idx].data.clone());
            assert_eq!(&cmax, &TEST_DATA[$idx].cmax);
        }
    };
}

macro_rules! test_alg {
    ($alg_name:ident) => {
        paste!{
            generate_algorithm_tests!([<$alg_name _test1>], $alg_name, 0);
            generate_algorithm_tests!([<$alg_name _test2>], $alg_name, 1);
            generate_algorithm_tests!([<$alg_name _test3>], $alg_name, 2);
            generate_algorithm_tests!([<$alg_name _test4>], $alg_name, 3);
            generate_algorithm_tests!([<$alg_name _test5>], $alg_name, 4);
            generate_algorithm_tests!([<$alg_name _test6>], $alg_name, 5);
            generate_algorithm_tests!([<$alg_name _test7>], $alg_name, 6);
            generate_algorithm_tests!([<$alg_name _test8>], $alg_name, 7);
        }
    };
}

test_alg!(schrage_vecs_sort_q_cmax);
test_alg!(schrage_vecs_sort_r_cmax);
test_alg!(schrage_custom_heaps_cmax);
test_alg!(schrage_heaps_bh_cmax);



// fn test_on_all_algs(case: usize) {
//     let cmax1 = schrage_heaps_bh_cmax(TEST_DATA[case].data.clone());
//     let cmax2 = schrage_custom_heaps_cmax(TEST_DATA[case].data.clone());
//     let cmax3 = schrage_vecs_sort_q_cmax(TEST_DATA[case].data.clone());
//     let cmax4 = schrage_vecs_sort_r_cmax(TEST_DATA[case].data.clone());
//     assert_eq!(&cmax2, &TEST_DATA[case].cmax);
//     assert_eq!(&cmax4, &TEST_DATA[case].cmax);
//     assert_eq!(&cmax3, &TEST_DATA[case].cmax);
//     assert_eq!(&cmax1, &TEST_DATA[case].cmax);
// }

// #[test]
// fn test_shrage_case1() {
//     test_on_all_algs(0);
// }

// #[test]
// fn test_shrage_case2() {
//     test_on_all_algs(1);
// }

// #[test]
// fn test_shrage_case3() {
//     test_on_all_algs(2);
// }

// #[test]
// fn test_shrage_case4() {
//     test_on_all_algs(3);
// }

// #[test]
// fn test_shrage_case5() {
//     test_on_all_algs(4);
// }

// #[test]
// fn test_shrage_case6() {
//     test_on_all_algs(5);
// }

// #[test]
// fn test_shrage_case7() {
//     test_on_all_algs(6);
// }

// #[test]
// fn test_shrage_case8() {
//     test_on_all_algs(7);
// }

// #[test]
// fn test_shrage_case9() {
//     test_on_all_algs(8);
// }
