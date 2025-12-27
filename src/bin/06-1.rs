use std::{cell::OnceCell, str::FromStr};

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
    let mut rows: Vec<Vec<usize>> = vec![];
    let cephalops: OnceCell<Vec<Cephalop>> = OnceCell::new();
    for line in std::io::stdin().lines() {
        let line = line?;
        let line = line.split_whitespace();
        match line.clone().map(usize::from_str).collect() {
            Ok(vec) => rows.push(vec),
            Err(_) => cephalops
                .set(
                    line.map(Cephalop::from_str)
                        .collect::<Result<_, _>>()
                        .unwrap(),
                )
                .unwrap(),
        }
    }
    let Some(cephalops) = cephalops.into_inner() else {
        panic!("cephalop odd");
    };
    let res = cephalops
        .into_iter()
        .enumerate()
        .map(|(i, op)| op.reduce(rows.iter().map(|row| row[i])))
        .sum::<usize>();
    println!("{res}");
    Ok(())
}
