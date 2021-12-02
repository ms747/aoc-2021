use std::time::Instant;

fn part1(input: &[u32]) -> usize {
    input.windows(2).filter(|w| w[1] > w[0]).count()
}

fn part2(input: &[u32]) -> usize {
    input.windows(4).filter(|w| w[3] > w[0]).count()
}

fn main() {
    let input: Vec<u32> = include_str!("input.txt")
        .lines()
        .map(|n| n.parse().unwrap())
        .collect();

    // Part 1
    let it = Instant::now();
    let count = part1(&input);
    println!("Part 1: {} Time: {:?}", count, it.elapsed());

    // Part 2
    let it = Instant::now();
    let count = part2(&input);
    println!("Part 2: {} Time: {:?}", count, it.elapsed());
}
