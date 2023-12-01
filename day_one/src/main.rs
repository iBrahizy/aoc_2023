fn main() {
    dbg!(part_one());
    dbg!(part_two());
}

fn part_one() -> u32 {
    let mut sum = 0;
    for line in include_str!("../input.txt").lines() {
        let mut digits = line
            .bytes()
            .filter(|x| x.is_ascii_digit())
            .map(|x| x - b'0');

        let first = digits.next().expect("line had no first digit present");
        let last = digits.rev().next().unwrap_or(first);

        sum += (first * 10 + last) as u32
    }
    sum
}

fn part_two() -> u32 {
    let mut sum = 0;
    for line in include_str!("../input.txt").lines() {
        let first = 'block: {
            let mut word_so_far = String::new();

            for c in line.chars() {
                if c.is_digit(10) {
                    break 'block c as u8 - b'0';
                }

                word_so_far.push(c);

                if let Some(num) = word_to_num(&word_so_far) {
                    break 'block num;
                }
            }

            unreachable!("first digit not present")
        };

        let mut word_so_far = String::new();
        let mut last = first;

        for c in line.chars().rev() {
            if c.is_digit(10) {
                last = c as u8 - b'0';
                break;
            }

            word_so_far = c.to_string() + &word_so_far;

            if let Some(num) = word_to_num(&word_so_far) {
                last = num;
                break;
            }
        }

        sum += (first * 10 + last) as u32;
    }
    sum
}

fn word_to_num(word: &str) -> Option<u8> {
    const NUMS: [&str; 9] = [
        "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
    ];

    for (i, num) in NUMS.into_iter().enumerate() {
        if word.contains(num) {
            return Some((i + 1) as u8);
        }
    }

    None
}
