fn main() {
    dbg!(task_one());
    dbg!(task_two());
}

fn parse_line(
    line: &'static str,
) -> (impl Iterator<Item = u8>, impl Iterator<Item = &'static str>) {
    let (_, mut cubes) = line.split_once(':').unwrap();
    cubes = &cubes[1..]; // remove extra space after colon

    let (counts, colours): (Vec<_>, Vec<_>) = cubes
        .split(' ')
        .partition(|x| x.bytes().all(|x| x.is_ascii_digit()));

    let counts = counts
        .into_iter()
        .map(|x| x.parse::<u8>().expect("filter for numbers didnt work"));
    let colours = colours.into_iter().map(|x| cut(x));

    (counts, colours)
}

fn task_one() -> usize {
    const MAX_RED: u8 = 12;
    const MAX_GREEN: u8 = 13;
    const MAX_BLUE: u8 = 14;
    let mut sum = 0;

    // id will be -1 from the actual id
    for (id, line) in include_str!("../input.txt").lines().enumerate() {
        let (counts, mut colours) = parse_line(line);

        if counts.into_iter().all(|x| {
            x <= match colours
                .next()
                .expect("different number of colours to numbers")
            {
                "red" => MAX_RED,
                "green" => MAX_GREEN,
                "blue" => MAX_BLUE,
                err => panic!("{err}"),
            }
        }) {
            sum += id + 1;
        }
    }

    sum
}

fn cut(colour: &'static str) -> &'static str {
    let last = colour.bytes().last().unwrap();

    if last == b',' || last == b';' {
        &colour[..colour.len() - 1]
    } else {
        colour
    }
}

fn task_two() -> u32 {
    let mut sum = 0;

    // id will be -1 from the actual id
    for line in include_str!("../input.txt").lines() {
        let (counts, mut colours) = parse_line(line);

        let mut max_red = 0;
        let mut max_blue = 0;
        let mut max_green = 0;

        for count in counts.into_iter() {
            let colour = match colours
                .next()
                .expect("different number of colours to numbers")
            {
                "red" => &mut max_red,
                "green" => &mut max_green,
                "blue" => &mut max_blue,
                err => panic!("{err}"),
            };
            *colour = (*colour).max(count);
        }

        sum += max_blue as u32 * max_red as u32 * max_green as u32;
    }

    sum
}
