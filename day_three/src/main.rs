fn main() {
    dbg!(part_one());
    dbg!(part_two());
}

fn part_one() -> u32 {
    let mut sum = 0;

    let engine = Engine::new(include_str!("../input.txt"));

    for y in 0..engine.schematic.len() {
        for x in 0..engine.schematic[0].len() {
            if engine.get(x, y).unwrap().is_symbol() {
                sum += engine.add_around_symbol(x, y);
            }
        }
    }

    sum
}

fn part_two() -> u32 {
    let mut sum = 0;

    let engine = Engine::new(include_str!("../input.txt"));

    for y in 0..engine.schematic.len() {
        for x in 0..engine.schematic[0].len() {
            if engine.get(x, y).unwrap() == '*' {
                sum += engine.add_around_star(x, y);
            }
        }
    }

    sum
}

pub trait Symbol {
    fn is_symbol(self) -> bool;
}

impl Symbol for char {
    fn is_symbol(self) -> bool {
        !self.is_ascii_digit() && self != '.'
    }
}

struct Engine {
    schematic: Vec<Vec<char>>,
}

impl Engine {
    fn new(input: &'static str) -> Engine {
        let schematic = input.lines().map(|x| x.chars().collect()).collect();

        Engine { schematic }
    }

    fn get(&self, x: usize, y: usize) -> Option<char> {
        self.schematic
            .get(y)
            .and_then(|column| column.get(x).copied())
    }

    // Assumes that there actually is a symbol at x, y
    // I hate this so fucking much
    fn add_around_symbol(&self, x: usize, y: usize) -> u32 {
        let mut sum = 0;
        let one_left = x.checked_sub(1);
        let one_down = y.checked_sub(1);

        match self.get(x, y + 1) {
            Some(point) if point.is_ascii_digit() => sum += self.full_number_at(x, y + 1),
            _ => {
                if let Some(point) = self.get(x + 1, y + 1) {
                    if point.is_ascii_digit() {
                        sum += self.full_number_at(x + 1, y + 1);
                    }
                }
                if let Some(one_left) = one_left {
                    if let Some(point) = self.get(one_left, y + 1) {
                        if point.is_ascii_digit() {
                            sum += self.full_number_at(one_left, y + 1);
                        }
                    }
                }
            }
        }

        if let Some(one_down) = one_down {
            match self.get(x, one_down) {
                Some(point) if point.is_ascii_digit() => sum += self.full_number_at(x, one_down),
                _ => {
                    if let Some(point) = self.get(x + 1, one_down) {
                        if point.is_ascii_digit() {
                            sum += self.full_number_at(x + 1, one_down);
                        }
                    }
                    if let Some(one_left) = one_left {
                        if let Some(point) = self.get(one_left, one_down) {
                            if point.is_ascii_digit() {
                                sum += self.full_number_at(one_left, one_down);
                            }
                        }
                    }
                }
            }
        }

        if let Some(point) = self.get(x + 1, y) {
            if point.is_ascii_digit() {
                sum += self.full_number_at(x + 1, y);
            }
        }

        if let Some(one_left) = one_left {
            if let Some(point) = self.get(one_left, y) {
                if point.is_ascii_digit() {
                    sum += self.full_number_at(one_left, y);
                }
            }
        }

        sum
    }
	
	fn add_around_star(&self, x: usize, y: usize) -> u32 {
        let mut nums = Vec::new();
        let one_left = x.checked_sub(1);
        let one_down = y.checked_sub(1);

        match self.get(x, y + 1) {
            Some(point) if point.is_ascii_digit() => nums.push(self.full_number_at(x, y + 1)),
            _ => {
                if let Some(point) = self.get(x + 1, y + 1) {
                    if point.is_ascii_digit() {
                        nums.push(self.full_number_at(x + 1, y + 1));
                    }
                }
                if let Some(one_left) = one_left {
                    if let Some(point) = self.get(one_left, y + 1) {
                        if point.is_ascii_digit() {
                            nums.push(self.full_number_at(one_left, y + 1));
                        }
                    }
                }
            }
        }

        if let Some(one_down) = one_down {
            match self.get(x, one_down) {
                Some(point) if point.is_ascii_digit() => nums.push(self.full_number_at(x, one_down)),
                _ => {
                    if let Some(point) = self.get(x + 1, one_down) {
                        if point.is_ascii_digit() {
                            nums.push(self.full_number_at(x + 1, one_down));
                        }
                    }
                    if let Some(one_left) = one_left {
                        if let Some(point) = self.get(one_left, one_down) {
                            if point.is_ascii_digit() {
                                nums.push(self.full_number_at(one_left, one_down));
                            }
                        }
                    }
                }
            }
        }

        if let Some(point) = self.get(x + 1, y) {
            if point.is_ascii_digit() {
                nums.push(self.full_number_at(x + 1, y));
            }
        }

        if let Some(one_left) = one_left {
            if let Some(point) = self.get(one_left, y) {
                if point.is_ascii_digit() {
                    nums.push(self.full_number_at(one_left, y));
                }
            }
        }

		if nums.len() == 2 {
			nums.into_iter().product()
		} else {
			0
		}
	}

    // Assumes that there actually is a number at x, y
    fn full_number_at(&self, x: usize, y: usize) -> u32 {
        let mut right = self.right_number_pos(x, y);
        let mut num = 0;
        let mut num_column = 1; // ie which power of 10

        while let Some(c) = self.get(right, y) {
            if !c.is_ascii_digit() {
                return num;
            }

            num += (c as u8 - b'0') as u32 * num_column;
            right = match right.checked_sub(1) {
                Some(x) => x,
                None => return num,
            };
            num_column *= 10;
        }
        num
    }

    // Assumes that there actually is a number at x, y
    // only returns x coord
    fn right_number_pos(&self, mut x: usize, y: usize) -> usize {
        while let Some(c) = self.get(x + 1, y) {
            if !c.is_ascii_digit() {
                return x;
            }

            x += 1;
        }
        x
    }
}
