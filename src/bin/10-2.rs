use std::{collections::HashSet, str::FromStr};

use itertools::Itertools;
use z3::{Optimize, SatResult, ast::Int};

type CounterIndex = usize;
type CounterValue = u64;

struct Machine {
    buttons: Vec<HashSet<CounterIndex>>,
    joltage: Vec<CounterValue>,
}

impl FromStr for Machine {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (_first, second) = s.split_once("] (").ok_or(())?;
        let (second, third) = second.split_once(") {").ok_or(())?;

        let joltage_str = third.strip_suffix("}").ok_or(())?;
        let joltage = joltage_str
            .split(',')
            .map(|j| j.parse::<CounterValue>().unwrap())
            .collect();

        let buttons = second
            .split(") (")
            .map(|schematic| {
                schematic
                    .split(',')
                    .map(|wiring| wiring.parse::<CounterIndex>().unwrap())
                    .collect()
            })
            .collect();

        Ok(Self { buttons, joltage })
    }
}

impl Machine {
    /// returns the shortest sequence of button presses to reach this machine's goal
    fn solve(&self) -> u64 {
        let optimizer = Optimize::new();
        let vars = self
            .buttons
            .iter()
            .map(|counters| {
                let var = Int::fresh_const("b");
                optimizer.assert(&var.ge(0));
                (var, counters)
            })
            .collect_vec();
        for (i, &target) in self.joltage.iter().enumerate() {
            let sum: Int = vars
                .iter()
                .filter_map(|(var, counters)| counters.contains(&i).then(|| var))
                .sum();
            optimizer.assert(&sum.eq(target))
        }
        let to_minimize: Int = vars.iter().map(|(var, _counters)| var).sum();
        optimizer.minimize(&to_minimize);
        let sat = optimizer.check(&[]);
        assert_eq!(sat, SatResult::Sat);
        let model = optimizer.get_model().unwrap();
        model.eval(&to_minimize, false).unwrap().as_u64().unwrap()
    }
}

fn main() {
    let res = std::io::stdin()
        .lines()
        .map(|line| line.unwrap().parse::<Machine>().unwrap())
        .map(|machine| machine.solve())
        .sum::<u64>();
    println!("{res}")
}
