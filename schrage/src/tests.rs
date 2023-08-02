use std::env;
use std::fs::File;
use std::io::Read;
use std::path::PathBuf;

use lazy_static::lazy_static;
use paste::paste;

use crate::custom_heap_impl::*;
use crate::std_heap_impl::*;
use crate::std_vecs_impl::*;
use crate::task::Task;

#[derive(Debug, Default)]
struct TestData {
    data_size: usize,
    data: Vec<Task>,
    order: Vec<Task>,
    cmax: u32,
    cmax_preemptive: u32,
    _data_name: String,
}

// test data is parsed only once
lazy_static! {
    #[derive(Debug)]
    static ref TEST_DATA: Vec<TestData> = parse_test_file("test_data/schr.data");
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

macro_rules! generate_algorithm_tests_preemptive {
    ($test_name:ident, $alg_func:ident, $idx:expr) => {
        #[test]
        fn $test_name() {
            let cmax = $alg_func(TEST_DATA[$idx].data.clone());
            assert_eq!(&cmax, &TEST_DATA[$idx].cmax_preemptive);
        }
    };
}

macro_rules! test_alg {
    ($alg_name:ident, $($test_case:expr),*) => {
        paste!{
            $(
                generate_algorithm_tests!([<$alg_name _test $test_case>], $alg_name, $test_case);
            )*
        }
    };
}

macro_rules! test_alg_preemptive {
    ($alg_name:ident, $($test_case:expr),*) => {
        paste!{
            $(
                generate_algorithm_tests_preemptive!([<$alg_name _test $test_case>], $alg_name, $test_case);
            )*
        }
    }
}

// test cases from the website
test_alg!(schrage_vecs_sort_q_cmax, 0, 1, 2, 3, 5, 6, 7);
test_alg!(schrage_vecs_sort_r_cmax, 0, 1, 2, 3, 5, 6, 7);
test_alg!(schrage_custom_heaps_cmax, 0, 1, 2, 3, 5, 6, 7);
test_alg!(schrage_heaps_std_cmax, 0, 1, 2, 3, 5, 6, 7);
test_alg_preemptive!(schrage_preemptive_heaps_std_cmax, 0, 1, 2, 3, 5, 6, 7);
test_alg_preemptive!(schrage_preemptive_custom_heaps_cmax, 0, 1, 2, 3, 5, 6, 7);
test_alg_preemptive!(schrage_preemptive_vecs_cmax, 0, 1, 3, 5);

// looks kinda ugly but gets the parsing done
fn parse_test_file(filename: &str) -> Vec<TestData> {
    let mut file_path: PathBuf = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    file_path.push(filename);
    let mut file = File::open(&file_path).unwrap();

    let mut buf = String::new();
    file.read_to_string(&mut buf).unwrap();

    let mut dv: Vec<TestData> = Vec::new();
    let mut lines = buf.lines();

    while let Some(l) = lines.next() {
        if l.is_empty() {
            continue;
        }
        if l[0..1] == *"d" {
            let mut data = TestData {
                _data_name: l.to_owned(),
                ..Default::default()
            };
            if let Some(l) = lines.next() {
                data.data_size = l.parse().unwrap();
                data.data = Vec::with_capacity(data.data_size);
            }
            for l in lines.by_ref() {
                if l.is_empty() {
                    break;
                }
                let mut t = l.split(' ').map(|w| w.parse::<u32>().unwrap());
                data.data.push(Task {
                    r: t.next().unwrap(),
                    p: t.next().unwrap(),
                    q: t.next().unwrap(),
                });
            }
            for l in lines.by_ref() {
                if l == "schrpmtn:" {
                    break;
                }
            }
            if let Some(l) = lines.next() {
                data.cmax_preemptive = l.parse::<u32>().unwrap();
            }

            for l in lines.by_ref() {
                if l == "schr:" {
                    break;
                }
            }
            if let Some(l) = lines.next() {
                data.cmax = l.parse::<u32>().unwrap();
            }
            if let Some(l) = lines.next() {
                data.order = l
                    .split(' ')
                    .map(|pos| data.data[pos.parse::<usize>().unwrap() - 1])
                    .collect();
            }
            dv.push(data);
        }
    }
    dv
}
