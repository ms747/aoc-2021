use std::collections::HashMap;
use std::time::Instant;

fn coord(line: &str) -> (i32, i32, i32, i32) {
    let coords = line
        .split(" -> ")
        .map(|y| {
            y.split(',')
                .map(|z| z.parse::<i32>().unwrap())
                .collect::<Vec<i32>>()
        })
        .collect::<Vec<_>>();
    (coords[0][0], coords[0][1], coords[1][0], coords[1][1])
}

fn part1(input: &[(i32, i32, i32, i32)], points: &mut HashMap<(i32, i32), i32>) -> usize {
    for coord in input {
        let x1 = coord.0;
        let y1 = coord.1;
        let x2 = coord.2;
        let y2 = coord.3;

        // Horizontal & Vertical Lines
        if x1 == x2 || y1 == y2 {
            if x1 == x2 {
                if y1 < y2 {
                    for y in y1..=y2 {
                        let count = points.entry((x1, y)).or_insert(0);
                        *count += 1;
                    }
                }
                for y in y2..=y1 {
                    let count = points.entry((x1, y)).or_insert(0);
                    *count += 1;
                }
            } else if y1 == y2 {
                if x1 < x2 {
                    for x in x1..=x2 {
                        let count = points.entry((x, y1)).or_insert(0);
                        *count += 1;
                    }
                } else {
                    for x in x2..=x1 {
                        let count = points.entry((x, y1)).or_insert(0);
                        *count += 1;
                    }
                }
            }
        }
    }

    let count = points.values().filter(|x| **x >= 2).count();
    count
}

fn part2(input: &[(i32, i32, i32, i32)], points: &mut HashMap<(i32, i32), i32>) -> usize {
    for coord in input {
        let x1 = coord.0;
        let y1 = coord.1;
        let x2 = coord.2;
        let y2 = coord.3;

        // Diagonal Lines (45 Degrees) Slope = 1 / -1
        if !(x1 == x2 || y1 == y2) {
            let slope = (y2 - y1) / (x2 - x1);
            if x1 < x2 {
                for x in x1..=x2 {
                    let y = slope * (x - x1) + y1;
                    let count = points.entry((x, y)).or_insert(0);
                    *count += 1;
                }
            } else {
                for x in x2..=x1 {
                    let y = slope * (x - x1) + y1;
                    let count = points.entry((x, y)).or_insert(0);
                    *count += 1;
                }
            }
        }
    }

    let count = points.values().filter(|x| **x >= 2).count();
    count
}

fn main() {
    let input: Vec<_> = include_str!("sample.txt").lines().map(coord).collect();
    let mut points: HashMap<(i32, i32), i32> = HashMap::new();

    // Part 1
    let it = Instant::now();
    let result = part1(&input, &mut points);
    println!("Part 1: {result} {:?}", it.elapsed());

    // Part 2
    let it = Instant::now();
    let result = part2(&input, &mut points);
    println!("Part 2: {result} {:?}", it.elapsed());
}
