use itertools::Itertools;

use crate::utils::{
    activate_cores, deactivate_cores, get_all_core_nums, get_nums_from_ranges, show_cores,
};
use crate::{CoresArgs, OptionalCoresArgs};

pub fn activate_cmd(args: &OptionalCoresArgs) {
    let all_core_nums = get_all_core_nums();
    let core_nums = match &args.cores {
        Some(ranges) => {
            let mut core_nums = get_nums_from_ranges(ranges);
            if args.no_duplicates {
                core_nums = core_nums.into_iter().unique().collect();
            }
            if args.sort {
                core_nums.sort()
            }
            core_nums
        }
        None => all_core_nums.clone(),
    };
    if !core_nums.iter().all(|num| all_core_nums.contains(num)) {
        panic!(
            "Error: the cores selected must be among {:?}",
            all_core_nums
        );
    }
    activate_cores(&core_nums);
}

pub fn deactivate_cmd(args: &CoresArgs) {
    let all_core_nums = get_all_core_nums();
    let core_nums = {
        let mut core_nums = get_nums_from_ranges(&args.cores);
        if args.no_duplicates {
            core_nums = core_nums.into_iter().unique().collect();
        }
        if args.sort {
            core_nums.sort()
        }
        core_nums
    };
    if !core_nums.iter().all(|num| all_core_nums.contains(num)) {
        panic!(
            "Error: the cores selected must be among {:?}",
            all_core_nums
        );
    }
    deactivate_cores(&core_nums);
}

pub fn show_cmd(args: &OptionalCoresArgs) {
    let all_core_nums = get_all_core_nums();
    let core_nums = match &args.cores {
        Some(ranges) => {
            let mut core_nums = get_nums_from_ranges(ranges);
            if args.no_duplicates {
                core_nums = core_nums.into_iter().unique().collect();
            }
            if args.sort {
                core_nums.sort()
            }
            core_nums
        }
        None => all_core_nums.clone(),
    };
    if !core_nums.iter().all(|num| all_core_nums.contains(num)) {
        panic!(
            "Error: the cores selected must be among {:?}",
            all_core_nums
        );
    }
    show_cores(&core_nums);
}
