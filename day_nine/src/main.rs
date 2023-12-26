fn main() {
    dbg!(part_one());
    dbg!(part_two());
}

fn next_element(sequence: &[i32]) -> i32 {
    if sequence.iter().all(|x| *x == 0) {
        0
    } else {
        let differences: Box<[_]> = sequence.windows(2).map(|arr| arr[1] - arr[0]).collect();
        sequence.iter().last().unwrap() + next_element(&differences)
    }
}

fn part_one() -> i32 {
    include_str!("../input.txt")
        .lines()
        .map(|x| {
            next_element(
                &x.split(' ')
                    .map(|x| x.parse().unwrap())
                    .collect::<Box<[_]>>(),
            )
        })
        .sum()
}

fn part_two() -> i32 {
    include_str!("../input.txt")
        .lines()
        .map(|x| {
            next_element(
                &x.split(' ')
                    .map(|x| x.parse().unwrap())
                    .rev()
                    .collect::<Box<[_]>>(),
            )
        })
        .sum()
}
