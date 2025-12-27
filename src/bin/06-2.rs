use std::str::FromStr;

use itertools::Itertools;

#[derive(thiserror::Error, Debug)]
enum Error {
    #[error("IO error: {0}")]
    IO(#[from] std::io::Error),
}

#[derive(Debug)]
enum Cephalop {
    Add,
    Multiply,
}

impl FromStr for Cephalop {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "+" => Ok(Self::Add),
            "*" => Ok(Self::Multiply),
            _ => Err(format!("Expected + or *, got {s:?}")),
        }
    }
}

impl Cephalop {
    fn reduce(&self, iter: impl Iterator<Item = usize>) -> usize {
        match self {
            Cephalop::Add => iter.sum(),
            Cephalop::Multiply => iter.product(),
        }
    }
}

fn main() -> Result<(), Error> {
    let mut lines = std::io::stdin().lines().map(Result::unwrap).collect_vec();
    let cephalops = lines.pop().expect("cephalops");
    let rows = lines
        .into_iter()
        .map(|line| line.chars().collect_vec())
        .collect_vec();
    let res = cephalops
        .match_indices(['+', '*'])
        .chain(std::iter::once((cephalops.len(), "")))
        .tuple_windows()
        .map(|((lo, op), (hi, _))| {
            let operands = (lo..hi).filter_map(|i| {
                let operand_str = rows.iter().map(|row| row[i]).collect::<String>();
                let operand_str = operand_str.trim();
                if operand_str.is_empty() {
                    None
                } else {
                    Some(operand_str.parse::<usize>().expect("cephaloperand"))
                }
            });
            op.trim()
                .parse::<Cephalop>()
                .expect("cephalop")
                .reduce(operands)
        })
        .sum::<usize>();
    println!("{res}");
    Ok(())
}
