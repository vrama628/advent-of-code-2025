use std::{collections::HashSet, num::ParseIntError, ops::RangeInclusive};

use itertools::Itertools;

const TEN: usize = 10;

fn number_of_digits(x: usize) -> u32 {
    x.ilog10() + 1
}

fn repeat_n_times(x: usize, n: u32) -> usize {
    let mut res = 0;
    let seq_len = number_of_digits(x);
    for i in 0..n {
        res += x * TEN.pow(i * seq_len)
    }
    res
}

fn repeateds(range: RangeInclusive<usize>) -> usize {
    let least_digits = number_of_digits(*range.start());
    let most_digits = number_of_digits(*range.end());
    let mut sequences = HashSet::new();
    for n_repetitions in 2..=most_digits {
        // make sure there's a multiple of this number within the lower and upper bounds
        if (most_digits / n_repetitions) * n_repetitions < least_digits {
            continue;
        }
        let start = if least_digits.is_multiple_of(n_repetitions) {
            // can use the prefix of the lower bound as a starting point
            let starting_segment_length = least_digits / n_repetitions;
            range.start() / TEN.pow((n_repetitions - 1) * starting_segment_length)
        } else {
            let seq_len = least_digits.next_multiple_of(n_repetitions) / n_repetitions;
            TEN.pow(seq_len - 1)
        };
        sequences.extend(
            (start..)
                .map(|seq| repeat_n_times(seq, n_repetitions))
                .skip_while(|seq| seq < range.start())
                .take_while(|seq| seq <= range.end()),
        );
    }
    sequences.into_iter().sum()
}

#[derive(thiserror::Error, Debug)]
enum Error {
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
    #[error("Parse int error: {0}")]
    ParseInt(#[from] ParseIntError),
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
