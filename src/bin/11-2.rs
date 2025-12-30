use std::collections::HashMap;

/// stores an INVERSION of the adjacency graph in the input:
/// output maps to its inputs
struct Graph {
    output_to_inputs: OutputToInputs,
    paths_to: HashMap<String, Paths>,
}
type OutputToInputs = HashMap<String, Vec<String>>;
type Paths = [usize; 4];

const DAC: usize = 0b01;
const FFT: usize = 0b10;

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
        let paths_to = HashMap::from([("svr".to_owned(), [1, 0, 0, 0])]);
        Self {
            output_to_inputs,
            paths_to,
        }
    }

    fn paths(&mut self, node: &str) -> Paths {
        if let Some(&n) = self.paths_to.get(node) {
            return n;
        }

        let mut paths = [0; 4];
        for input in self.output_to_inputs.get(node).cloned().unwrap_or_default() {
            let input_paths = self.paths(&input);
            let mut mask = 0;
            if node == "dac" {
                mask |= DAC;
            }
            if node == "fft" {
                mask |= FFT;
            }
            for i in 0..4 {
                paths[i | mask] += input_paths[i]
            }
        }
        self.paths_to.insert(node.to_owned(), paths);
        paths
    }
}

fn main() {
    let mut graph = Graph::parse();
    let paths = graph.paths("out");
    println!("{}", paths[DAC | FFT])
}
