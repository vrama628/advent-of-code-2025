use itertools::Itertools;

#[derive(Debug, thiserror::Error)]
enum Error {
    #[error("IO error: {0}")]
    IO(#[from] std::io::Error),
}

const SEQ_LEN: usize = 12;

fn joltage(bank: &[u8]) -> usize {
    debug_assert!(bank.len() >= SEQ_LEN);
    let mut largests: Vec<Vec<u8>> = vec![vec![]];
    for &b in bank.iter().rev() {
        for i in (0..largests.len().min(SEQ_LEN)).rev() {
            let mut candidate = vec![b];
            candidate.extend(largests[i].iter());
            match largests.get_mut(i + 1) {
                None => {
                    debug_assert_eq!(i + 1, largests.len());
                    largests.push(candidate)
                }
                Some(existing) => {
                    if candidate > *existing {
                        *existing = candidate
                    }
                }
            }
        }
    }
    largests[SEQ_LEN]
        .iter()
        .fold(0, |acc, &digit| acc * 10 + usize::from(digit))
}

fn main() -> Result<(), Error> {
    let res = std::io::stdin().lines().process_results(|lines| {
        lines
            .map(|line| {
                let mut bytes = line.into_bytes();
                bytes.iter_mut().for_each(|b| *b -= b'0');
                joltage(&bytes)
            })
            .sum::<usize>()
    })?;
    println!("{res}");
    Ok(())
}
