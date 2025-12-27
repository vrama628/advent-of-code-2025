#![feature(btree_cursors)]

use std::{
    collections::BTreeMap,
    fmt::Display,
    ops::{RangeBounds, RangeInclusive},
};

use itertools::Itertools;

fn main() {
    let mut ranges = RangeSet::new();
    std::io::stdin()
        .lines()
        .map(Result::unwrap)
        .take_while(|line| !line.is_empty())
        .map(|line| {
            let (start_str, end_str) = line.split_once('-').unwrap();
            let start = start_str.parse::<usize>().unwrap();
            let end = end_str.parse::<usize>().unwrap();
            start..=end
        })
        .for_each(|range| ranges.add(range));
    let res = ranges
        .0
        .into_iter()
        .tuples()
        .map(|((start_i, start_bound), (end_i, end_bound))| {
            debug_assert_eq!(start_bound, RangeBoundary::Start);
            debug_assert_eq!(end_bound, RangeBoundary::End);
            end_i - start_i
        })
        .sum::<usize>();
    println!("{res}");
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum RangeBoundary {
    /// inclusive
    Start,
    /// exclusive
    End,
}

/// invariants:
/// - for every start bound, the next element is always its end bound
/// - for every end bount, the previous element is always it start bound
#[derive(Clone)]
struct RangeSet(Inner);
type Inner = BTreeMap<usize, RangeBoundary>;

impl RangeSet {
    fn new() -> Self {
        Self(BTreeMap::new())
    }

    fn add(&mut self, range: RangeInclusive<usize>) {
        let carve_range = *range.start()..=range.end() + 1;
        self.0
            .extract_if(carve_range.clone(), |_, _| true)
            .for_each(drop);
        let starts_in_existing_range = self
            .0
            .upper_bound(carve_range.start_bound())
            .peek_prev()
            .is_some_and(|(_, &b)| matches!(b, RangeBoundary::Start));
        let ends_in_existing_range = self
            .0
            .lower_bound(carve_range.end_bound())
            .peek_next()
            .is_some_and(|(_, &b)| matches!(b, RangeBoundary::End));
        if !starts_in_existing_range {
            let clobbered = self.0.insert(*range.start(), RangeBoundary::Start);
            debug_assert!(clobbered.is_none(), "{self}\n + {range:?}")
        }
        if !ends_in_existing_range {
            let clobbered = self.0.insert(*range.end() + 1, RangeBoundary::End);
            debug_assert!(clobbered.is_none(), "{self} \n {range:?}")
        }
    }
}

impl Display for RangeSet {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for ((&start_i, &start_bound), (&end_i, &end_bound)) in self.0.iter().tuples() {
            debug_assert_eq!(start_bound, RangeBoundary::Start);
            debug_assert_eq!(end_bound, RangeBoundary::End);
            write!(f, "[{start_i}, {end_i})  ")?;
        }
        Ok(())
    }
}
