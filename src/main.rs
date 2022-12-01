use std::fs;

fn main() {
    advent_day_1();
}

fn advent_day_1() {
    println!("Day 1 -------");
    let contents = fs::read_to_string("src/calories.txt").expect("Cannot open file");
    let summed_calories: Vec<u32> = contents
        .split("\n\n")
        .map(|group| {
            group.lines().fold(0, |acc, line| {
                acc + line.parse::<u32>().expect("Unsupported file content")
            })
        })
        .collect();

    let mut elves_vec = vec![];
    for &sum in summed_calories.iter() {
        elves_vec.push(sum);
    }

    elves_vec.sort();
    elves_vec.reverse();
    println!("Elf with more calories: {:?}", elves_vec.first());

    let sum_top_3 = elves_vec.iter().take(3).fold(0, |acc, i| acc + i);
    println!("Sum of top 3 elves: {sum_top_3}");
    println!("-------------");
}
