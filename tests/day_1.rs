use std::{fs, str::FromStr};

use adventofcode2023::{add, callibration_value::CalibrationValue};

#[test]
fn test_day_1_part_1() {
    let file = fs::read_to_string("resources/day_1.txt").expect("File should be available");

    let mut total: usize = 0;

    for line in file.lines() {
        if line.len() == 0 {
            continue;
        }

        let value = CalibrationValue::from_str(line).expect("Should parse the input line");

        total = total + value.value;
    }

    assert_eq!(total, 55712);
}

#[test]
fn test_day_1_part_2() {
    let file = fs::read_to_string("resources/day_1.txt").expect("File should be available");

    let mut total: usize = 0;

    for line in file.lines() {
        println!("raw: {}", "1rsjbbhtkbbfourqzdhlone4eighttwo");
        if line.len() == 0 {
            continue;
        }

        let value = CalibrationValue::new_v2(line.to_string());
        println!("result: {}", value.value);

        total = total + value.value;
    }

    assert_eq!(total, 1);
}

