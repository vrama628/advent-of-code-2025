use itertools::Itertools;

#[derive(Debug, thiserror::Error)]
enum Error {
    #[error("IO error: {0}")]
    IO(#[from] std::io::Error),
}

fn joltage(bank: &[u8]) -> u8 {
    debug_assert!(bank.len() >= 2);
    // largest single-digit and double-digit numbers seen in the suffix processed so far
    let last = bank[bank.len() - 1];
    let second_last = bank[bank.len() - 2];
    let mut largest_single = last.max(second_last);
    let mut largest_double = (second_last, last);
    for &b in bank.iter().rev().skip(2) {
        largest_double = largest_double.max((b, largest_single));
        largest_single = largest_single.max(b)
    }
    largest_double.0 * 10 + largest_double.1
}

fn main() -> Result<(), Error> {
    let res = std::io::stdin().lines().process_results(|lines| {
        lines
            .map(|line| {
                let mut bytes = line.into_bytes();
                bytes.iter_mut().for_each(|b| *b -= b'0');
                joltage(&bytes)
            })
            .map(usize::from)
            .sum::<usize>()
    })?;
    println!("{res}");
    Ok(())
}
