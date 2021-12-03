fn divide(invert: bool, width: usize, input: &mut [u32]) -> u32 {
    if input.len() == 1 {
        return input[0];
    }

    let index = itertools::partition(&mut *input, |x| x & (1 << (width - 1)) != 0);

    if invert ^ (index * 2 >= input.len()) {
        divide(invert, width - 1, &mut input[..index])
    } else {
        divide(invert, width - 1, &mut input[index..])
    }
}

fn part1_a(cols: usize, numbers: &[u32]) -> u32 {
    let val: u32 = (0..cols)
        .map(|x| 1 << x)
        .filter(|mask| numbers.iter().filter(|num| *num & *mask != 0).count() * 2 > numbers.len())
        .sum();

    let ans = val * (((1 << cols) - 1) ^ val);

    ans
}

fn part2_a(cols: usize, numbers: &mut [u32]) -> u32 {
    divide(false, cols, numbers) * divide(true, cols, numbers)
}

use std::time::Instant;

fn main() {
    let input: Vec<_> = include_str!("input.txt").lines().collect();

    let cols = input[0].len();
    let mut numbers: Vec<u32> = input
        .iter()
        .map(|x| u32::from_str_radix(x, 2).unwrap())
        .collect();

    let it = Instant::now();
    let result = part1_a(cols, &numbers);
    println!("Part 1: {} Time: {:?}", result, it.elapsed());

    let it = Instant::now();
    let result = part2_a(cols, &mut numbers);
    println!("Part 2: {} Time: {:?}", result, it.elapsed());
}
