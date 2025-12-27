use std::collections::{HashMap, HashSet};

struct State {
    timelines: HashMap<usize, usize>,
}

impl State {
    fn new(start: usize) -> Self {
        Self {
            timelines: [(start, 1)].into(),
        }
    }

    fn step(self, splitters: HashSet<usize>) -> Self {
        let Self { timelines } = self;
        let mut new_timelines = HashMap::new();
        for (beam, count) in timelines {
            if splitters.contains(&beam) {
                *new_timelines.entry(beam - 1).or_default() += count;
                *new_timelines.entry(beam + 1).or_default() += count;
            } else {
                *new_timelines.entry(beam).or_default() += count;
            }
        }
        Self {
            timelines: new_timelines,
        }
    }

    fn timelines(&self) -> usize {
        self.timelines.values().sum()
    }
}

fn main() {
    let mut lines = std::io::stdin().lines().map(Result::unwrap);
    let start = lines
        .next()
        .expect("first line")
        .char_indices()
        .find_map(|(i, c)| (c == 'S').then_some(i))
        .expect("starting position");
    let manifold = lines.map(|line| {
        line.char_indices()
            .filter_map(|(i, c)| (c == '^').then_some(i))
            .collect::<HashSet<usize>>()
    });
    let state = manifold.fold(State::new(start), |state, splitters| state.step(splitters));
    println!("{}", state.timelines());
}
