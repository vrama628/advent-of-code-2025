use std::collections::HashSet;

struct State {
    beams: HashSet<usize>,
    splits: usize,
}

impl State {
    fn new(start: usize) -> Self {
        Self {
            beams: [start].into(),
            splits: 0,
        }
    }

    fn step(self, splitters: HashSet<usize>) -> Self {
        let Self { beams, mut splits } = self;
        let mut new_beams = HashSet::new();
        for beam in beams {
            if splitters.contains(&beam) {
                new_beams.insert(beam - 1);
                new_beams.insert(beam + 1);
                splits += 1;
            } else {
                new_beams.insert(beam);
            }
        }
        Self {
            beams: new_beams,
            splits,
        }
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
    println!("{}", state.splits);
}
