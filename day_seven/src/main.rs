use std::cmp::Ordering;

fn main() {
    dbg!(part_one());
}

fn part_one() -> u32 {
    let mut hands = Vec::new();

    for line in include_str!("../input.txt").lines() {
        let (cards, bid) = line.split_once(' ').unwrap();

        hands.push((Hand::new(cards), bid.parse::<u32>().unwrap()));
    }

    hands.sort_by(|x, y| x.0.cmp(&y.0));
	dbg!(&hands);

    hands
        .into_iter()
        .map(|x| x.1)
        .enumerate()
        .fold(0, |acc, (i, x)| acc + ((i as u32 + 1) * x))
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

        let mut self_maxes = self_map.map.iter().fold([0, 0], |mut acc, e| {
            let min = acc.iter_mut().min().unwrap();

            if *e > *min {
                *min = *e;
            }

            acc
        });

        self_maxes.sort();

        let mut other_maxes = other_map.map.iter().fold([0, 0], |mut acc, e| {
            let min = acc.iter_mut().min().unwrap();

            if *e > *min {
                *min = *e;
            }

            acc
        });

        other_maxes.sort();

        if self_maxes[1] == other_maxes[1] {
            let comparison = self_maxes[0].cmp(&other_maxes[0]);

            if comparison != Ordering::Equal {
                return comparison;
            }

            // for (yeppers, other) in self.cards.into_iter().zip(other.cards.into_iter()) {
            //     match yeppers.value.cmp(&other.value) {
            //         Ordering::Equal => (),
            //         x => return x,
            //     }
            // }

			self.cards.into_iter().cmp(other.cards.into_iter())
        } else {
            self_maxes[1].cmp(&other_maxes[1])
        }
    }
}

#[derive(Debug)]
struct HandMap {
    map: [u8; (Card::MAX - Card::MIN + 1) as usize],
}

impl HandMap {
    fn new(hand: &Hand) -> HandMap {
        let mut map = [0; (Card::MAX - Card::MIN + 1) as usize];

        for card in hand.cards {
            map[(card.value - Card::MIN) as usize] += 1;
        }

        HandMap { map }
    }
}
