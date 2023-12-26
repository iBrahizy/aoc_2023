fn main() {
    dbg!(part_one());
}

fn next_element(sequence: &[i32]) -> i32 {
    if sequence.iter().all(|x| *x == 0) {
        0
    } else {
        let mut differences = Vec::new();

        for arr in sequence.windows(2) {
            differences.push(arr[1] - arr[0]);
        }

        sequence.iter().last().unwrap() + next_element(&differences)
    }
}

fn part_one() -> i32 {
    include_str!("../input.txt")
        .lines()
        .map(|x| next_element(&x.split(' ').map(|x| x.parse().unwrap()).collect::<Vec<_>>()))
        .sum()
}
