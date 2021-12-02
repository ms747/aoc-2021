use std::time::Instant;

fn mapper(course: &[&str]) -> (i32, i32) {
    let num: i32 = course[1].parse().unwrap();
    match course[0] {
        "forward" => (num, 0),
        "up" => (0, -num),
        "down" => (0, num),
        _ => panic!("Unknown instruction"),
    }
}

fn part1(input: &[Vec<&str>]) -> i32 {
    let compute = input
        .iter()
        .map(|course| mapper(&course[..]))
        .fold((0, 0), |(hor, dep), (x, y)| (hor + x, dep + y));

    compute.0 * compute.1
}

fn part2(input: &[Vec<&str>]) -> i32 {
    let compute = input.iter().map(|course| mapper(&course[..])).fold(
        (0, 0, 0),
        |(hor, dep, aim), (x, y)| {
            if y == 0 {
                return (hor + x, dep + aim * x, aim);
            }
            (hor, dep, aim + y)
        },
    );

    compute.0 * compute.1
}

fn main() {
    let input: Vec<_> = include_str!("input.txt")
        .lines()
        .map(|line| line.split(' ').collect::<Vec<&str>>())
        .collect();

    let it = Instant::now();
    let answer = part1(&input);
    println!("Part 1: {} Time: {:?}", answer, it.elapsed());

    let it = Instant::now();
    let answer = part2(&input);
    println!("Part 2: {} Time: {:?}", answer, it.elapsed());
}
