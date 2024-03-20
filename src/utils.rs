use regex::Regex;
use std::fs;
use std::process::Command;

const CORES_PATH: &str = "/sys/devices/system/cpu";

pub fn ranges_are_valid(ranges: &str) -> bool {
    let re = Regex::new(r"^([0-9]+-[0-9]+,|[0-9]+,)*([0-9]+-[0-9]+|[0-9]+)$").unwrap();
    re.is_match(ranges)
}

pub fn parse_range(range: String) -> Vec<u32> {
    match range.split_once('-') {
        Some((a, b)) => {
            let x = match a.parse::<u32>() {
                Ok(v) => v,
                Err(_) => panic!("Cannot parse {a} as a u32"),
            };
            let y = match b.parse::<u32>() {
                Ok(v) => v,
                Err(_) => panic!("Cannot parse {b} as a u32"),
            };
            if x > y {
                panic!("Cannot parse range '{}' as {:?} > {:?}", range, x, y)
            } else {
                (x..y + 1).collect()
            }
        }
        None => match range.parse::<u32>() {
            Ok(v) => vec![v],
            Err(_) => panic!("Cannot parse range '{range}' as it is not a u32"),
        },
    }
}

pub fn get_nums_from_ranges(ranges: String) -> Vec<u32> {
    let mut nums: Vec<u32> = vec![];
    if !ranges_are_valid(&ranges) {
        panic!("The ranges '{ranges}' are incorrect, example '1-5,0,41-41'")
    };
    for range in ranges.split(',') {
        nums.append(&mut parse_range(range.to_string()));
    }
    nums
}

pub fn get_all_core_nums() -> Vec<u32> {
    let re_cpu_name = Regex::new(r"^cpu[0-9]+$").unwrap();
    let mut all_core_nums: Vec<u32> = vec![];
    for entry in fs::read_dir(CORES_PATH).unwrap_or_else(|_| panic!("Cannot read {:?}", CORES_PATH))
    {
        let entry = entry.expect("Entry is wrong");
        let path = entry.path();
        if !path.is_dir() {
            continue;
        }
        let dir_name = path.file_name().expect("").to_str().expect("");
        if re_cpu_name.is_match(dir_name) {
            let num_str = &dir_name[3..];
            all_core_nums.push(
                num_str
                    .parse::<u32>()
                    .expect("'{:?}' doesn't have a number"),
            )
        };
    }
    all_core_nums.sort();
    all_core_nums
}

pub fn activate_cores(core_nums: Vec<u32>) {
    for n in core_nums {
        if n == 0 {
            continue;
        }
        let fp = format!("{CORES_PATH}/cpu{n}/online");
        Command::new("sudo")
            .args(["su", "-c", &format!("echo 1 > \"{fp}\"")])
            .output()
            .expect("failed to execute process");
    }
}

pub fn deactivate_cores(core_nums: Vec<u32>) {
    if core_nums.contains(&0) {
        panic!("Cannot deactivate core 0")
    }
    for n in core_nums {
        let fp = format!("{CORES_PATH}/cpu{n}/online");
        Command::new("sudo")
            .args(["su", "-c", &format!("echo 0 > \"{fp}\"")])
            .output()
            .expect("failed to execute process");
    }
}

pub fn show_cores(core_nums: Vec<u32>) {
    for n in core_nums {
        if n == 0 {
            println!("{n}\t1\tAlways on");
            continue;
        }
        let fp = format!("{CORES_PATH}/cpu{n}/online");
        let core_str = fs::read_to_string(fp).expect("Cannot read the file");
        let core_status = core_str.get(..1).expect("Cannot find core status");
        println!("{n}\t{core_status}");
    }
}

#[cfg(test)]
mod tests {
    use crate::utils::get_all_core_nums;
    use crate::utils::get_nums_from_ranges;
    use crate::utils::parse_range;
    use crate::utils::ranges_are_valid;

    #[test]
    fn ranges_are_valid_correct() {
        let ranges = "2-5,1,9-11,31-31,0,55".to_string();
        assert!(ranges_are_valid(&ranges));
    }
    #[test]
    fn ranges_are_valid_negative_number() {
        let ranges = "2-5,-1,9-11,31-31,0,55".to_string();
        assert!(!ranges_are_valid(&ranges));
    }
    #[test]
    fn ranges_are_valid_letter() {
        let ranges = "2-5,1,9-11,f,31-31,0,55".to_string();
        assert!(!ranges_are_valid(&ranges));
    }

    #[test]
    fn parse_range_correct() {
        let ranges = ["2-5", "0", "11-11", "84"];
        let results: Vec<Vec<u32>> = vec![vec![2, 3, 4, 5], vec![0], vec![11], vec![84]];
        for (range, result) in ranges.iter().zip(results.iter()) {
            assert_eq!(&parse_range(range.to_string()), result);
        }
    }

    #[test]
    fn get_nums_from_ranges_correct() {
        let range = "2-5,1,9-11,31-31,0,55".to_string();
        assert_eq!(
            get_nums_from_ranges(range),
            vec![2, 3, 4, 5, 1, 9, 10, 11, 31, 0, 55]
        );
    }

    #[test]
    fn get_all_core_nums_at_least_0() {
        assert!(get_all_core_nums().contains(&0));
    }
}
