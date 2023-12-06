// today kinda wack lol
fn main() {
    dbg!(part_one());
    dbg!(part_two());
}

fn part_one() -> u32 {
    let mut product = 1;

    for Race { time, distance } in races_one() {
        let mut amount_of_ways = 0;

        for hold_length in 1..time {
            if hold_length * (time - hold_length) > distance {
                amount_of_ways += 1;
            }
        }

        product *= amount_of_ways;
    }

    product
}

fn part_two() -> u64 {
    let mut input = include_str!("../input.txt").lines();

    let mut time = 0;
    let mut distance = 0;

    for (column, byte) in input
        .next()
        .unwrap()
        .split_once(": ")
        .unwrap()
        .1
        .bytes()
        .filter(|x| x.is_ascii_digit())
        .rev()
        .enumerate()
    {
		let num = (byte - b'0') as u64;
		let column = column as u32;

        time += num * (10u64.pow(column));
    }

    for (column, byte) in input
        .next()
        .unwrap()
        .split_once(": ")
        .unwrap()
        .1
        .bytes()
        .filter(|x| x.is_ascii_digit())
        .rev()
        .enumerate()
    {
		let num = (byte - b'0') as u64;
		let column = column as u32;

        distance += num * (10u64.pow(column));
    }

	dbg!(time, distance);

    let mut amount_of_ways = 0;
    for hold_length in 1..time {
        if hold_length * (time - hold_length) > distance {
            amount_of_ways += 1;
        }
    }

    amount_of_ways
}

fn races_one() -> impl Iterator<Item = Race> {
    let mut input = include_str!("../input.txt").lines();

    input
        .next()
        .unwrap()
        .split_once(": ")
        .unwrap()
        .1
        .split(" ")
        .filter_map(|x| x.parse::<u32>().ok())
        .zip(
            input
                .next()
                .unwrap()
                .split_once(": ")
                .unwrap()
                .1
                .split(" ")
                .filter_map(|x| x.parse::<u32>().ok()),
        )
        .map(|(time, distance)| Race { time, distance })
}

#[derive(Debug)]
struct Race {
    time: u32,
    distance: u32,
}
