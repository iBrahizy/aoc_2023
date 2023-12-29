fn main() {
    dbg!(part_one());
}

fn part_one() -> u32 {
    let mut location = Point { x: 0, y: 0 };
    let mut things = Vec::new();

    for (y, line) in include_str!("../input.txt").lines().enumerate() {
        let mut vec = Vec::new();
        for (x, thing) in line.chars().enumerate() {
            if thing == 'S' {
                location = Point { x, y };
            }

            vec.push(thing);
        }
        things.push(vec);
    }

    let mut queue = std::collections::VecDeque::new();
    let mut visited = std::collections::HashSet::new();

    if things[location.y][location.x + 1].connects_west() {
        queue.push_back(Node {
            prev: Direction::West,
            point: Point {
                x: location.x + 1,
                y: location.y,
            },
			depth: 0,
        })
    }

    if things[location.y][location.x - 1].connects_east() {
        queue.push_back(Node {
            prev: Direction::East,
            point: Point {
                x: location.x - 1,
                y: location.y,
            },
			depth: 0,
        })
    }

    if things[location.y + 1][location.x].connects_north() {
        queue.push_back(Node {
            prev: Direction::North,
            point: Point {
                x: location.x,
                y: location.y + 1,
            },
			depth: 0,
        })
    }

    if things[location.y - 1][location.x].connects_south() {
        queue.push_back(Node {
            prev: Direction::South,
            point: Point {
                x: location.x,
                y: location.y - 1,
            },
			depth: 0,
        })
    }

	// breadth first search
	let mut max = 0;
    while let Some(node) = queue.pop_front() {
        let connection = node.connection(&things);

        if !visited.contains(&connection.point) {
			visited.insert(connection.point);

			if connection.depth > max {
				max = connection.depth;
			}

            queue.push_back(connection);
        }
    }

	max + 1
}

struct Node {
    prev: Direction,
    point: Point,
	depth: u32,
}

impl Node {
    fn connection(&self, things: &Vec<Vec<char>>) -> Node {
        let thing = things[self.point.y][self.point.x];

        if thing.connects_south() && self.prev != Direction::South {
            Node {
                point: Point {
                    x: self.point.x,
                    y: self.point.y + 1,
                },
                prev: Direction::North,
				depth: self.depth + 1,
            }
        } else if thing.connects_north() && self.prev != Direction::North {
            Node {
                point: Point {
                    x: self.point.x,
                    y: self.point.y - 1,
                },
                prev: Direction::South,
				depth: self.depth + 1,
            }
        } else if thing.connects_east() && self.prev != Direction::East {
            Node {
                point: Point {
                    x: self.point.x + 1,
                    y: self.point.y,
                },
                prev: Direction::West,
				depth: self.depth + 1,
            }
        } else {
            Node {
                point: Point {
                    x: self.point.x - 1,
                    y: self.point.y,
                },
                prev: Direction::East,
				depth: self.depth + 1,
            }
        }
    }
}

#[derive(PartialEq, Eq)]
enum Direction {
    North,
    East,
    South,
    West,
}

#[derive(PartialEq, Eq, Hash, Clone, Copy)]
struct Point {
    x: usize,
    y: usize,
}

trait Connects {
    fn connects_north(&self) -> bool;
    fn connects_south(&self) -> bool;
    fn connects_west(&self) -> bool;
    fn connects_east(&self) -> bool;
}

impl Connects for char {
    fn connects_west(&self) -> bool {
        match *self {
            '-' => true,
            'J' => true,
            '7' => true,
            _ => false,
        }
    }

    fn connects_north(&self) -> bool {
        match *self {
            '|' => true,
            'J' => true,
            'L' => true,
            _ => false,
        }
    }

    fn connects_south(&self) -> bool {
        match *self {
            '|' => true,
            '7' => true,
            'F' => true,
            _ => false,
        }
    }

    fn connects_east(&self) -> bool {
        match *self {
            '-' => true,
            'F' => true,
            'L' => true,
            _ => false,
        }
    }
}

// struct Field {
//     location: Point,
//     things: Vec<Vec<char>>,
// }
