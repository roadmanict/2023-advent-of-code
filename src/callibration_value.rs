use std::str::FromStr;

pub struct CalibrationValue {
    raw_value: String,
    string_value: String,
    pub value: usize,
}

fn find_first_number_in_string(value: &String) -> Option<char> {
    value.chars().find(|c| c.is_digit(10))
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
