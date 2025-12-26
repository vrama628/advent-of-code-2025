use std::collections::BTreeSet;

use itertools::Itertools;

#[derive(Debug, thiserror::Error)]
enum Error {
    #[error("IO error: {0}")]
    IO(#[from] std::io::Error),
}

fn main() -> Result<(), Error> {
    let input: BTreeSet<Position> = std::io::stdin().lines().process_results(|lines| {
        lines
            .enumerate()
            .flat_map(|(row, line)| {
                line.char_indices()
                    .filter_map(|(col, c)| (c == '@').then(|| Position { row, col }))
                    .collect_vec()
            })
            .collect()
    })?;
    let res = input
        .iter()
        .filter(|pos| pos.adjacent().filter(|adj| input.contains(adj)).count() < 4)
        .count();
    println!("{res}");
    Ok(())
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
struct Position {
    row: usize,
    col: usize,
}

impl Position {
    fn adjacent(&self) -> impl Iterator<Item = Self> {
        let rows = [self.row, self.row + 1]
            .into_iter()
            .chain(self.row.checked_sub(1));
        let cols = [self.col, self.col + 1]
            .into_iter()
            .chain(self.col.checked_sub(1));
        rows.cartesian_product(cols)
            .map(|(row, col)| Self { row, col })
            .filter(move |pos| pos != self)
    }
}
