use advent2024::CALENDAR;

fn main() {
    CALENDAR.iter().enumerate().rev().for_each(|(day, soln)| {
        println!("----------------- DAY {} -----------------", day + 1);
        println!("  Part 1: {}", soln.part1());
        println!("  Part 2: {}", soln.part2());
    });
}
