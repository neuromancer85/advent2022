use std::fs;

fn main() {
    advent_day_1();
    advent_day_2();
}

fn advent_day_1() {
    println!("Day 1 -------");
    let contents = fs::read_to_string("src/calories.txt").expect("Cannot open file");
    let mut elves_vec: Vec<u32> = contents
        .split("\n\n")
        .map(|group| {
            group.lines().fold(0, |acc, line| {
                acc + line.parse::<u32>().expect("Unsupported file content")
            })
        })
        .collect();

    elves_vec.sort_by(|a, b| b.cmp(a));
    println!("Elf with more calories: {:?}", elves_vec.last());

    let sum_top_3: u32 = elves_vec.iter().take(3).sum();
    println!("Sum of top 3 elves: {sum_top_3}");
    println!("-------------");
}

fn advent_day_2() {
    println!("Day 1 -------");
    let contents = fs::read_to_string("src/rockpaperscissor.txt").expect("Cannot open file");
    
    let guide = contents.lines().map(|line| line.split_once( ' ').unwrap()).collect::<Vec<_>>();
    
    for game in guide {
        println!("{:?}", game);
    }
    
    println!("-------------");
}
