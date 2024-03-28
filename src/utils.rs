use regex::Regex;
use std::error;
use std::fmt;
use std::fs;
use std::process::Command;

const CORES_PATH: &str = "/sys/devices/system/cpu";

#[derive(Debug)]
pub enum RangeError {
    ParseIntError,
    BoundariesError,
}

impl error::Error for RangeError {}
impl fmt::Display for RangeError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            RangeError::ParseIntError => write!(f, "Cannot parse as an integer"),
            RangeError::BoundariesError => write!(f, "Cannot parse as a range"),
        }
    }
}

impl From<std::num::ParseIntError> for RangeError {
    fn from(_e: std::num::ParseIntError) -> Self {
        Self::ParseIntError
    }
}

pub fn parse_range(range: &str) -> Result<Vec<u32>, RangeError> {
    match range.split_once('-') {
        Some((a, b)) => {
            let x = a.parse::<u32>()?;
            let y = b.parse::<u32>()?;
            if x > y {
                Err(RangeError::BoundariesError)
            } else {
                Ok((x..y + 1).collect())
            }
        }
        None => Ok(vec![range.parse::<u32>()?]),
    }
}

pub fn get_nums_from_ranges(ranges: &str) -> Vec<u32> {
    let mut nums: Vec<u32> = vec![];
    for range in ranges.split(',') {
        nums.append(&mut parse_range(range).unwrap_or_else(|e| panic!("{:?}: '{range}'", e)));
    }
    nums
}

pub fn get_all_core_nums() -> Vec<u32> {
    let re_cpu_name = Regex::new(r"^cpu[0-9]+$").unwrap();
    let mut all_core_nums: Vec<u32> = vec![];
    for entry in fs::read_dir(CORES_PATH).unwrap_or_else(|e| panic!("{:?}: '{CORES_PATH}'", e)) {
        let entry = entry.expect("How can entry be wrong?");
        let path = entry.path();
        if !path.is_dir() {
            continue;
        }
        let dir_name = path.file_name().unwrap().to_str().unwrap();
        if re_cpu_name.is_match(dir_name) {
            let num_str = &dir_name[3..];
            all_core_nums.push(
                num_str
                    .parse::<u32>()
                    .expect("'{path}' should only contain a number"),
            )
        };
    }
    all_core_nums.sort();
    all_core_nums
}

pub enum CoreStatus {
    Off,
    On,
}

impl CoreStatus {
    pub fn value(&self) -> u8 {
        match *self {
            CoreStatus::Off => 0,
            CoreStatus::On => 1,
        }
    }

    pub fn _name(&self) -> String {
        match *self {
            CoreStatus::Off => "off".to_string(),
            CoreStatus::On => "on".to_string(),
        }
    }
}

/// Set a status for specified cores.
/// A status of `false` (or `0`) means off and `true` (or `1`) means on.
pub fn set_cores_status(status: CoreStatus, core_nums: &[u32]) {
    let status_str = status.value();
    for n in core_nums {
        // Skip cpu0 as it is always on and cannot be modified
        if n == &0 {
            continue;
        }

        let fp = format!("{CORES_PATH}/cpu{n}/online");
        match Command::new("sudo")
            .args(["su", "-c", &format!("echo {status_str} > '{fp}'")])
            .output()
        {
            Ok(_) => (),
            Err(_) => println!("Failed to execute: sudo su -c \"echo {status_str} > '{fp}'\""),
        }
    }
}

pub fn show_cores(core_nums: &[u32]) {
    for n in core_nums {
        // cpu0 is always on and the file 'CORES_PATH/cpu0/online' does not exist
        if n == &0 {
            let core_status = 1;
            println!("cpu{n:<5}{core_status:<4}Always on");
            continue;
        }
        let fp = format!("{CORES_PATH}/cpu{n}/online");
        let core_str = fs::read_to_string(fp).unwrap_or_else(|e| panic!("{:?}", e));
        // TODO: match 0 to off and 1 to on and chande README.md
        let core_status = core_str
            .get(..1)
            .expect("Cannot get status from {core_str}");
        println!("cpu{n:<5}{core_status}");
    }
}

#[cfg(test)]
mod tests {
    use crate::utils::get_all_core_nums;
    use crate::utils::get_nums_from_ranges;
    use crate::utils::parse_range;
    use crate::utils::RangeError;

    #[test]
    fn parse_range_correct() {
        let ranges = ["2-5", "0", "11-11", "84"];
        let results = [vec![2, 3, 4, 5], vec![0], vec![11], vec![84]];
        for (range, result) in ranges.iter().zip(results.iter()) {
            assert_eq!(&parse_range(range).unwrap(), result);
        }
    }
    #[test]
    fn parse_range_rangeerror_parseinterror() {
        let ranges = ["2f", "-", "0-?", "-4"];
        for range in ranges {
            assert!(matches!(
                &parse_range(range).unwrap_err(),
                RangeError::ParseIntError
            ));
        }
    }
    #[test]
    fn parse_range_rangeerror_boundarieserror() {
        let ranges = ["1-0"];
        for range in ranges {
            assert!(matches!(
                &parse_range(range).unwrap_err(),
                RangeError::BoundariesError
            ));
        }
    }

    #[test]
    fn get_nums_from_ranges_correct() {
        let range = "2-5,1,9-11,31-31,0,55";
        assert_eq!(
            get_nums_from_ranges(range),
            vec![2, 3, 4, 5, 1, 9, 10, 11, 31, 0, 55]
        );
    }

    #[test]
    fn get_all_core_nums_at_least_0() {
        // Should run successfully on any machine as any machine should have
        // at least 1 CPU (cpu0)
        assert!(get_all_core_nums().contains(&0));
    }
}
