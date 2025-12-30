use std::collections::HashMap;

/// stores an INVERSION of the adjacency graph in the input:
/// output maps to its inputs
struct Graph {
    output_to_inputs: OutputToInputs,
    paths_to: HashMap<String, usize>,
}
type OutputToInputs = HashMap<String, Vec<String>>;

impl Graph {
    fn parse() -> Self {
        let mut output_to_inputs: OutputToInputs = HashMap::new();
        for line in std::io::stdin().lines().map(Result::unwrap) {
            let (input, outputs_str) = line.split_once(": ").unwrap();
            for output in outputs_str.split(' ') {
                output_to_inputs
                    .entry(output.to_owned())
                    .or_default()
                    .push(input.to_owned());
            }
        }
        let paths_to = HashMap::from([("you".to_owned(), 1)]);
        Self {
            output_to_inputs,
            paths_to,
        }
    }

    fn paths(&mut self, node: &str) -> usize {
        if let Some(&n) = self.paths_to.get(node) {
            return n;
        }

        let res = self
            .output_to_inputs
            .get(node)
            .cloned()
            .unwrap_or_default()
            .into_iter()
            .map(|input| self.paths(&input))
            .sum();
        self.paths_to.insert(node.to_owned(), res);
        res
    }
}

fn main() {
    let mut graph = Graph::parse();
    println!("{}", graph.paths("out"))
}
