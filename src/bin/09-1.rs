use itertools::Itertools;

fn main() {
    let reds = std::io::stdin()
        .lines()
        .map(|line| {
            let line = line.unwrap();
            let (x_str, y_str) = line.split_once(',').unwrap();
            let x = x_str.parse::<usize>().unwrap();
            let y = y_str.parse::<usize>().unwrap();
            (x, y)
        })
        .collect_vec();
    let res = reds
        .clone()
        .into_iter()
        .cartesian_product(reds)
        .map(|((x1, y1), (x2, y2))| (x1.abs_diff(x2) + 1) * (y1.abs_diff(y2) + 1))
        .max()
        .unwrap();
    println!("{res}");
}
