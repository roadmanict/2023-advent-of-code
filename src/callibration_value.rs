use std::str::FromStr;

pub struct CalibrationValue {
    raw_value: String,
    string_value: String,
    pub value: usize,
}

struct IntStringMap {
    string_value: String,
    int_value: char,
}

impl IntStringMap {
    fn new(string_value: String, int_value: char) -> IntStringMap {
        IntStringMap {
            string_value,
            int_value,
        }
    }
}

fn find_first_number_in_string(value: &String) -> Option<char> {
    value.chars().find(|c| c.is_digit(10))
}

fn find_first_number_or_string_number_in_string(value: &String, reverse: bool) -> Option<char> {
    let mut result: Option<char> = None;
    let mut value = value.to_owned();
    if reverse {
        value = value.chars().rev().collect::<String>();
    }
    let int_value_position = value.chars().position(|c| c.is_digit(10));

    // println!("pos: {:?}, reverse: {}", int_value_position, reverse);

    let int_string_map: Vec<IntStringMap> = vec![
        IntStringMap::new("one".to_string(), '1'),
        IntStringMap::new("two".to_string(), '2'),
        IntStringMap::new("three".to_string(), '3'),
        IntStringMap::new("four".to_string(), '4'),
        IntStringMap::new("five".to_string(), '5'),
        IntStringMap::new("six".to_string(), '6'),
        IntStringMap::new("seven".to_string(), '7'),
        IntStringMap::new("eight".to_string(), '8'),
        IntStringMap::new("nine".to_string(), '9'),
    ];

    let mut lowest_position: usize = 10000;

    for item in int_string_map.iter() {
        let mut string_value = item.string_value.to_owned();
        if reverse {
            string_value = item.string_value.chars().rev().collect::<String>();
        }

        let position = value.find(&string_value);

        if let Some(position) = position {
            if position < lowest_position {
                lowest_position = position;
                result = Some(item.int_value)
            }
        }
    }

    if let Some(pos) = int_value_position {
        if pos < lowest_position {
            result = value.chars().find(|c| c.is_digit(10));
        }
    }
    
    println!("final: {:?}", result);

    result
}

impl CalibrationValue {
    pub fn new(raw_value: String) -> CalibrationValue {
        let raw_first_number =
            find_first_number_in_string(&raw_value).expect("Should have found a number");
        let raw_second_number =
            find_first_number_in_string(&raw_value.chars().rev().collect::<String>())
                .expect("Should have found a number");

        let mut string_value = String::new();
        string_value.push(raw_first_number);
        string_value.push(raw_second_number);

        let value = string_value
            .parse::<usize>()
            .expect("Value should be parsed to usize");

        CalibrationValue {
            raw_value,
            string_value,
            value,
        }
    }

    pub fn new_v2(raw_value: String) -> CalibrationValue {
        let raw_first_number = find_first_number_or_string_number_in_string(&raw_value, false)
            .expect("Should have found a number");
        let raw_second_number = find_first_number_or_string_number_in_string(&raw_value, true)
            .expect("Should have found a number");

        let mut string_value = String::new();
        string_value.push(raw_first_number);
        string_value.push(raw_second_number);

        let value = string_value
            .parse::<usize>()
            .expect("Value should be parsed to usize");

        CalibrationValue {
            raw_value,
            string_value,
            value,
        }
    }
}

#[derive(Debug)]
pub enum CalibrationValueFromStrError {
    Generic(),
}

impl FromStr for CalibrationValue {
    type Err = CalibrationValueFromStrError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(CalibrationValue::new(s.to_string()))
    }
}
