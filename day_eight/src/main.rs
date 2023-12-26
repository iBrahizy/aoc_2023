use std::collections::HashMap;

fn main() {
    dbg!(part_one());
    dbg!(part_two());
}

#[derive(Debug)]
struct Node {
    left: &'static str,
    right: &'static str,
}

fn parse_graph(lines: &'static str) -> HashMap<&'static str, Node> {
    let mut graph = HashMap::new();

    for line in lines.lines() {
        let node_key = &line[0..3];
        let node = Node {
            left: &line[7..10],
            right: &line[12..15],
        };

        graph.insert(node_key, node);
    }

    graph
}

fn part_one() -> usize {
    let (instructions, graph) = include_str!("../input.txt").split_once("\n\n").unwrap();

    let graph = parse_graph(graph);
    let mut location = "AAA";

    for (steps, direction) in instructions.chars().cycle().enumerate() {
        let current = graph.get(location).unwrap();
        location = match direction {
            'R' => current.right,
            'L' => current.left,
            _ => panic!(),
        };

        if location == "ZZZ" {
            return steps + 1;
        }
    }

    unreachable!();
}

fn part_two() -> usize {
    let (instructions, graph) = include_str!("../input.txt").split_once("\n\n").unwrap();

    let graph = parse_graph(graph);
    // let mut location = "AAA";

    let locations: Vec<_> = graph
        .keys()
        .copied()
        .filter(|x| x.chars().last().unwrap() == 'A')
        .collect();

	let mut nums = Vec::new();

    for mut location in locations {
        for (steps, direction) in instructions.chars().cycle().enumerate() {
            let current = graph.get(location).unwrap();
            location = match direction {
                'R' => current.right,
                'L' => current.left,
                _ => panic!(),
            };

            if location.chars().last().unwrap() == 'Z' {
                nums.push(steps + 1);
				break;
            }
        }
    }

	let mut nums = nums.into_iter();
	let first = nums.next().unwrap();

	nums.fold(first, |acc, e| {
		acc.lowest_common_multiple(e)
	})
}

trait Poggers {
	fn greatest_common_divisor(self, other: Self) -> Self;
	fn lowest_common_multiple(self, other: Self) -> Self;
}

impl Poggers for usize {
	fn greatest_common_divisor(self, other: Self) -> Self {
		if other == 0 {
			self
		} else {
			other.greatest_common_divisor(self % other)
		}
	}

	fn lowest_common_multiple(self, other: Self) -> Self {
		(self * other) / self.greatest_common_divisor(other)
	}
}
