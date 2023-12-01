// Copyright 2023 Louis Royer. All rights reserved.
// Use of this source code is governed by a MIT-style license that can be
// found in the LICENSE file.
// SPDX-License-Identifier: MIT

use regex::Regex;
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
mod tests {
    use super::*;

    #[test]
    fn it_calibration_sum() {
        // part 1: only ascii digits
        assert_eq!(
            calibration_sum(vec!["1abc2", "pqr3stu8vwx", "a1b2c3d4e5f", "treb7uchet"].into_iter()),
            Ok(142)
        );
        // part 2: digits spelled with letters
        assert_eq!(
            calibration_sum(
                vec![
                    "two1nine",
                    "eightwothree",
                    "abcone2threexyz",
                    "xtwone3four",
                    "4nineeightseven2",
                    "zoneight234",
                    "7pqrstsixteen"
                ]
                .into_iter()
            ),
            Ok(281)
        )
    }

    #[test]
    fn it_calibration() {
        assert!(calibration("abcd").is_err());
        // part 1: only ascii digits
        assert_eq!(calibration("1abc2"), Ok(12));
        assert_eq!(calibration("pqr3stu8vwx"), Ok(38));
        assert_eq!(calibration("a1b2c3d4e5f"), Ok(15));
        assert_eq!(calibration("treb7uchet"), Ok(77));
        // part 2: digits spelled with letters
        assert_eq!(calibration("two1nine"), Ok(29));
        assert_eq!(calibration("eightwothree"), Ok(83));
        assert_eq!(calibration("abcone2threexyz"), Ok(13));
        assert_eq!(calibration("xtwone3four"), Ok(24));
        assert_eq!(calibration("4nineeightseven2"), Ok(42));
        assert_eq!(calibration("zoneight234"), Ok(14));
        assert_eq!(calibration("7pqrstsixteen"), Ok(76));
    }
}

/// Returns the sum of calibration values
fn calibration_sum<'a>(lines: impl Iterator<Item = &'a str>) -> Result<u32, ParseCalibrationError> {
    lines
        .filter(|x| !x.is_empty()) // exclude empty lines,
        .map(|x| calibration(x)) // compute calibrations
        .sum() // and sum them all
}

/// The calibration value can be found by combining the first digit
/// and the last digit (in that order) to form a single two-digit number.
fn calibration(line: &str) -> Result<u32, ParseCalibrationError> {
    // regex for a digit (one to nine), written as an ascii digit or spelled out with letters
    let re = Regex::new(r"([1-9]|one|two|three|four|five|six|seven|eight|nine)").unwrap();
    let digits: Vec<&str> = re.find_iter(line).map(|m| m.as_str()).collect();

    // first digit
    let first = digits.first().ok_or(ParseCalibrationError)?;
    let first = parse_calibration_digit(first)?;

    // last digit
    let last = digits.last().ok_or(ParseCalibrationError)?;
    let last = parse_calibration_digit(last)?;

    // concatenate digits
    Ok((first * 10) + last)
}

/// Parse a digit from Calibration value digit into u32.
/// A digit is either an ascii digit or spelled out with letters.
/// Only one to nine digits are considered (zero is excluded).
fn parse_calibration_digit(d: &str) -> Result<u32, ParseCalibrationError> {
    match d {
        "1" | "one" => Ok(1),
        "2" | "two" => Ok(2),
        "3" | "three" => Ok(3),
        "4" | "four" => Ok(4),
        "5" | "five" => Ok(5),
        "6" | "six" => Ok(6),
        "7" | "seven" => Ok(7),
        "8" | "eight" => Ok(8),
        "9" | "nine" => Ok(9),
        // zero doesn't count
        _ => return Err(ParseCalibrationError),
    }
}
