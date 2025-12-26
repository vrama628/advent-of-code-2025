use std::{num::ParseIntError, ops::RangeInclusive};

use itertools::Itertools;

#[derive(thiserror::Error, Debug)]
enum Error {
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
    #[error("Parse int error: {0}")]
    ParseInt(#[from] ParseIntError),
}

const TEN: usize = 10;

fn number_of_digits(x: usize) -> u32 {
    x.ilog10() + 1
}

fn remove_last_n_digits(x: usize, n: u32) -> usize {
    x / TEN.pow(n)
}

fn retain_last_n_digits(x: usize, n: u32) -> usize {
    x % TEN.pow(n)
}

fn repeateds(range: RangeInclusive<usize>) -> usize {
    let lo = {
        let start = *range.start();
        let n_digits = number_of_digits(start);
        if n_digits % 2 == 0 {
            // even number of digits
            let half_n_digits = n_digits / 2;
            let first_half = remove_last_n_digits(start, half_n_digits);
            let second_half = retain_last_n_digits(start, half_n_digits);
            // check if repeating the first half is within the range
            if second_half <= first_half {
                // repeating the first half is within the range
                first_half
            } else {
                // repeating the first half is NOT within the range
                first_half + 1
            }
        } else {
            // odd number of digits, so the first possible repeat is
            // the smallest number with one more digit
            TEN.pow(n_digits / 2)
        }
    };

    let hi = {
        let end = *range.end();
        let n_digits = number_of_digits(end);
        if n_digits % 2 == 0 {
            // even number of digits
            let half_n_digits = n_digits / 2;
            let first_half = remove_last_n_digits(end, half_n_digits);
            let second_half = retain_last_n_digits(end, half_n_digits);
            // check if repeating the first half is within the range
            if second_half >= first_half {
                // repeating the first half is within the range
                first_half
            } else {
                // repeating the first half is NOT within the range
                first_half - 1
            }
        } else {
            // odd number of digits, so the last possible repeat is
            // the largest number with one less digit
            TEN.pow(n_digits / 2) - 1
        }
    };
    (lo..=hi)
        .map(|half| {
            let n_digits = number_of_digits(half);
            half * TEN.pow(n_digits) + half
        })
        .sum()
}

fn main() -> Result<(), Error> {
    let res = std::io::read_to_string(std::io::stdin())?
        .trim()
        .split(',')
        .map(|range_str| -> Result<_, Error> {
            let (first_str, last_str) = range_str.split_once('-').unwrap();
            let first = first_str.parse()?;
            let last = last_str.parse()?;
            Ok(first..=last)
        })
        .process_results(|ranges| ranges.map(repeateds).sum::<usize>())?;
    println!("{res}");
    Ok(())
}
