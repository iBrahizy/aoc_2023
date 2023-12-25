use std::cmp::Ordering;

fn main() {
    dbg!(part_one());
    dbg!(part_two());
}

fn part_one() -> u32 {
    let mut hands = Vec::new();

    for line in include_str!("../input.txt").lines() {
        let (cards, bid) = line.split_once(' ').unwrap();

        hands.push((Hand::new(cards), bid.parse::<u32>().unwrap()));
    }

    hands.sort_by(|x, y| x.0.cmp(&y.0));

    hands
        .into_iter()
        .map(|x| x.1)
        .enumerate()
        .fold(0, |acc, (i, x)| acc + ((i as u32 + 1) * x))
}

fn part_two() -> u32 {
    let mut hands = Vec::new();

    for line in include_str!("../input.txt").lines() {
        let (cards, bid) = line.split_once(' ').unwrap();

        hands.push((Hand::new(cards), bid.parse::<u32>().unwrap()));
    }

    hands.sort_by(|x, y| part_two_cmp(&x.0, &y.0));

    hands
        .into_iter()
        .map(|x| x.1)
        .enumerate()
        .fold(0, |acc, (i, x)| acc + ((i as u32 + 1) * x))
}

fn part_two_cmp(hand: &Hand, other_hand: &Hand) -> Ordering {
    let hand_map = HandMap::new(hand);
	let hand_j_count = hand_map.map[(11 - Card::MIN) as usize];
    let other_hand_map = HandMap::new(other_hand);
	let other_hand_j_count = other_hand_map.map[(11 - Card::MIN) as usize];

    // finds the highest 2, idk i feel like this is a stupid way to do it
    let mut hand_maxes = hand_map.into_iter().fold([0, 0], |mut acc, (card_type, e)| {
		// joker
		if card_type == 11 {
			return acc;
		}
        if e > acc[0] {
            acc[1] = acc[0];
            acc[0] = e;
        } else if e > acc[1] {
            acc[1] = e;
        }
        acc
    });

	hand_maxes[0] += hand_j_count;

    let mut other_hand_maxes = other_hand_map.into_iter().fold([0, 0], |mut acc, (card_type, e)| {
		// joker
		if card_type == 11 {
			return acc;
		}
        if e > acc[0] {
            acc[1] = acc[0];
            acc[0] = e;
        } else if e > acc[1] {
            acc[1] = e;
        }
        acc
    });

	other_hand_maxes[0] += other_hand_j_count;

    if hand_maxes[0] == other_hand_maxes[0] {
        let comparison = hand_maxes[1].cmp(&other_hand_maxes[1]);

        if comparison != Ordering::Equal {
            comparison
        } else {
			for (yeppers, other) in hand.cards.into_iter().zip(other_hand.cards) {
				let yeppers = if yeppers.value == 11 {
					1
				} else {
					yeppers.value
				};

				let other = if other.value == 11 {
					1
				} else {
					other.value
				};

				match yeppers.cmp(&other) {
					Ordering::Equal => (),
					x => return x,
				};
			}

			unreachable!();
        }
    } else {
        hand_maxes[0].cmp(&other_hand_maxes[0])
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug, PartialOrd, Ord)]
struct Card {
    value: u8,
}

impl Card {
    const MAX: u8 = 14;
    const MIN: u8 = 2;

    fn new(c: char) -> Self {
        Self {
            value: match c {
                'T' => 10,
                'J' => 11,
                'Q' => 12,
                'K' => 13,
                'A' => 14,
                x if x.is_ascii_digit()
                    && x as u8 - b'0' >= Self::MIN
                    && x as u8 - b'0' <= Self::MAX =>
                {
                    x as u8 - b'0'
                }
                _ => panic!(),
            },
        }
    }
}

#[derive(PartialEq, Eq, Debug, Clone)]
struct Hand {
    cards: [Card; 5],
}

impl Hand {
    fn new(input: &'static str) -> Hand {
        if input.len() != 5 {
            panic!();
        }

        let mut cards = [Card { value: 0 }; 5];

        for (i, card) in input.chars().enumerate() {
            cards[i] = Card::new(card);
        }

        Hand { cards }
    }
}

impl std::cmp::PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl std::cmp::Ord for Hand {
    fn cmp(&self, other: &Self) -> Ordering {
        let self_map = HandMap::new(self);
        let other_map = HandMap::new(other);

        // finds the highest 2, idk i feel like this is a stupid way to do it
        let self_maxes = self_map.into_iter().fold([0, 0], |mut acc, (_, e)| {
            if e > acc[0] {
                acc[1] = acc[0];
                acc[0] = e;
            } else if e > acc[1] {
                acc[1] = e;
            }
            acc
        });

        let other_maxes = other_map.into_iter().fold([0, 0], |mut acc, (_, e)| {
            if e > acc[0] {
                acc[1] = acc[0];
                acc[0] = e;
            } else if e > acc[1] {
                acc[1] = e;
            }
            acc
        });

        if self_maxes[0] == other_maxes[0] {
            let comparison = self_maxes[1].cmp(&other_maxes[1]);

            if comparison != Ordering::Equal {
                comparison
            } else {
                self.cards.into_iter().cmp(other.cards.into_iter())
            }
        } else {
            self_maxes[0].cmp(&other_maxes[0])
        }
    }
}

#[derive(Debug)]
struct HandMap {
    map: [u8; (Card::MAX - Card::MIN + 1) as usize],
    i: usize,
}

impl HandMap {
    fn new(hand: &Hand) -> HandMap {
        let mut map = [0; (Card::MAX - Card::MIN + 1) as usize];

        for card in hand.cards {
            map[(card.value - Card::MIN) as usize] += 1;
        }

        HandMap { map, i: 0 }
    }
}

impl Iterator for HandMap {
    type Item = (u8, u8);

    fn next(&mut self) -> Option<Self::Item> {
        let e = self.map.get(self.i)?;
        if *e == 0 {
            self.i += 1;
            self.next()
        } else {
            self.i += 1;
            Some((self.i as u8 + Card::MIN - 1, *e))
        }
    }
}
