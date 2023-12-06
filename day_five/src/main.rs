use std::ops::RangeInclusive;

fn main() {
    dbg!(part_one());
    dbg!(part_two());
}

fn part_one() -> u64 {
    let (seeds, maps) = parse_input();

	let mut locations = Vec::new();
    for mut thing in seeds {
		for map in &maps {
			thing = map.get_mapped_value(thing);
		}
		locations.push(thing);
	}

	locations.into_iter().min().unwrap()
}

fn part_two() -> u64 {
    let (mut seeds, maps) = parse_input();

	let mut new_seeds = Vec::new();

	while let Some(seed) = seeds.next() {
		for i in 0..seeds.next().unwrap() {
			new_seeds.push(seed + i);
		}
	}

	let mut min = u64::MAX;
    for mut thing in new_seeds {
		for map in &maps {
			thing = map.get_mapped_value(thing);
		}
		if thing < min {
			min = thing;
		}
	}

	min
}

trait GetMappedValue {
    fn get_mapped_value(&self, input: u64) -> u64;
}
impl GetMappedValue for Vec<MapElement> {
    fn get_mapped_value(&self, input: u64) -> u64 {
        for MapElement { range, offset } in self {
            if range.contains(&input) {
                return (input as i64 + offset) as u64;
            }
        }

		input
    }
}

fn parse_input() -> (impl Iterator<Item = u64>, Vec<Vec<MapElement>>) {
    let mut sections = include_str!("../input.txt").split("\n\n");

    let seeds = sections
        .next()
        .unwrap()
        .split_once(": ")
        .unwrap()
        .1
        .split(' ')
        .map(|x| x.parse::<u64>().unwrap());

    let mut maps = Vec::new();

    for section in sections {
        let mut map = Vec::new();

        for line in section.lines().skip(1) {
            let mut nums = line.split(" ").map(|x| x.parse().unwrap());

            let dest_start = nums.next().unwrap();
            let source_start = nums.next().unwrap();
            let range = nums.next().unwrap();

            map.push(MapElement {
                range: source_start..=(source_start + range),
                offset: dest_start as i64 - source_start as i64,
            });
        }

        maps.push(map);
    }

    (seeds, maps)
}

#[derive(Debug)]
struct MapElement {
    range: RangeInclusive<u64>,
    offset: i64,
}
