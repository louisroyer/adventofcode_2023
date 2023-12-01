// Copyright 2023 Louis Royer. All rights reserved.
// Use of this source code is governed by a MIT-style license that can be
// found in the LICENSE file.
// SPDX-License-Identifier: MIT

use std::fs::read_to_string;

#[derive(Debug, PartialEq)]
struct ParseCalibrationError;

/// Day 1: Trebuchet?!
fn main() {
    let file_path = "inputs/01.in";
    let file = read_to_string(file_path).expect("Could not open calibration document");

    // TODO: use a buffer instead
    let sum = calibration_sum(file.lines()).expect("Could not parse calibration document");
    println!("{sum}");
}

#[cfg(test)]
    mod tests{
    use super::*;

    #[test]
    fn it_calibration_sum() {
        assert_eq!(calibration_sum(vec!["1abc2", "pqr3stu8vwx", "a1b2c3d4e5f", "treb7uchet"].into_iter()), Ok(142));
    }
}

/// Returns the sum of calibration values
fn calibration_sum<'a>(lines: impl Iterator<Item = &'a str> ) -> Result<u32, ParseCalibrationError> {
    lines
        .filter(|x| !x.is_empty()) // exclude empty lines,
        .map(|x| calibration(x))   // compute calibrations
        .sum()                     // and sum them all
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_calibration() {
        assert!(calibration("abcd").is_err());
        assert_eq!(calibration("1abc2"), Ok(12));
        assert_eq!(calibration("pqr3stu8vwx"), Ok(38));
        assert_eq!(calibration("a1b2c3d4e5f"), Ok(15));
        assert_eq!(calibration("treb7uchet"), Ok(77));
    }
}

/// The calibration value can be found by combining the first digit
/// and the last digit (in that order) to form a single two-digit number.
fn calibration(line: &str) -> Result<u32, ParseCalibrationError> {
    let digits : Vec<&str> = line.matches(|c: char| c.is_ascii_digit()).collect();
    let first = parse_calibration_digit(digits.first())?;
    let last = parse_calibration_digit(digits.last())?;
    Ok((first * 10) + last)

}

/// Parse a digit from Calibration value digit into u8
fn parse_calibration_digit(d: Option<&&str>) -> Result<u32, ParseCalibrationError> {
    match d {
        Some(ascii_digit) => match ascii_digit.parse::<u32>() {
            Ok(digit) => Ok(digit),
            Err(_) =>return Err(ParseCalibrationError)
        },
        None => return Err(ParseCalibrationError),
    }
}
