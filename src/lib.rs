pub trait AdventSolution {
    fn part1(&self) -> u32;
    fn part2(&self) -> u32;
}

pub mod days {
    pub mod day1;
    pub mod day2;
}

pub const CALENDAR: [&dyn AdventSolution; 2] = [
    &days::day1::Day1,
    &days::day2::Day2
];
