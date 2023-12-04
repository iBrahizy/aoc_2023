use std::collections::HashSet;

fn main() {
    dbg!(part_one());
    dbg!(part_two());
}

fn part_one() -> u32 {
    let mut sum = 0;

    for line in include_str!("../input.txt").lines() {
        let (winning, yours) = line[10..].split_once('|').unwrap();
        let mut points = 1u32;

        let winning: HashSet<_> = winning
            .split(' ')
            .filter_map(|x| x.parse::<u8>().ok())
            .collect();

        for num in yours.split(' ').filter_map(|x| x.parse::<u8>().ok()) {
            if winning.contains(&num) {
                points *= 2;
            }
        }

        sum += points / 2
    }

    sum
}

fn part_two() -> u32 {
	let mut scratch_cards = [1u32; 204];
	// card num is actually 1 below actual card num
	for (card_num, line) in include_str!("../input.txt").lines().enumerate() {
        let (winning, yours) = line[10..].split_once('|').unwrap();
        let mut points = 0;

        let winning: HashSet<_> = winning
            .split(' ')
            .filter_map(|x| x.parse::<u8>().ok())
            .collect();

        for num in yours.split(' ').filter_map(|x| x.parse::<u8>().ok()) {
            if winning.contains(&num) {
				points += 1;
            }
        }

		let multiplier = scratch_cards[card_num];
		for i in 0..points {
			let Some(amount) = scratch_cards.get_mut(card_num + 1 + i) else {
				break;
			};

			*amount += multiplier;
		}
	}

	scratch_cards.into_iter().sum()
}
