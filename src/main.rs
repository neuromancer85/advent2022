use std::fs;

fn main() {
    advent_day_1();
    println!("-------------");
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
}

fn advent_day_2() {
    println!("Day 2 -------");
    let contents = fs::read_to_string("src/rockpaperscissor.txt").expect("Cannot open file");

    let guide = contents
        .lines()
        .map(|line| line.split_once(' ').unwrap())
        .collect::<Vec<_>>();

    let mut part_1_score = 0;
    for turn in guide.clone() {
        let res = match turn {
            ("A", "X") => 4,
            ("A", "Y") => 8,
            ("A", "Z") => 3,
            ("B", "X") => 1,
            ("B", "Y") => 5,
            ("B", "Z") => 9,
            ("C", "X") => 7,
            ("C", "Y") => 2,
            ("C", "Z") => 6,
            (_, _) => panic!("oops!"),
        };
        part_1_score = part_1_score + res;
    }
    println!("Total Part 1 score: {part_1_score}");

    let mut part_2_score = 0;
    for turn in guide {
        let res = match turn {
            ("A", "X") => 3,
            ("A", "Y") => 4,
            ("A", "Z") => 8,
            ("B", "X") => 1,
            ("B", "Y") => 5,
            ("B", "Z") => 9,
            ("C", "X") => 2,
            ("C", "Y") => 6,
            ("C", "Z") => 7,
            (_, _) => panic!("oops!"),
        };
        part_2_score = part_2_score + res;
    }
    println!("Total Part 2 score: {part_2_score}");
}
