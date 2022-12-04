#![feature(iter_array_chunks)]
use std::{
    fs,
    ops::{Div, RangeBounds},
};

fn main() {
    advent_day_1();
    println!("-------------");
    advent_day_2();
    println!("-------------");
    advent_day_3_part_1();
    println!("-------------");
    advent_day_3_part_2();
    println!("-------------");
    advent_day_4_part1();
    advent_day_4_part2();
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

fn advent_day_3_part_1() {
    println!("Day 3 part 1-");
    let contents = fs::read_to_string("src/items.txt").expect("Cannot open file");
    let rucksacks = contents
        .lines()
        .map(|line| line.split_at(line.len().div(2)))
        .collect::<Vec<_>>();
    let mut acc = 0;
    let letters = ('a'..='z').chain('A'..='Z').collect::<String>();
    for (sack_1, sack_2) in rucksacks {
        let common_char_index = sack_1.chars().filter_map(|c| sack_2.find(c)).last();
        let common_char = match common_char_index {
            Some(index) => sack_2.chars().nth(index).unwrap(),
            None => panic!("oops!"),
        };
        let item_priority = letters.chars().position(|c| c == common_char).unwrap() + 1;
        //println!("{common_char}: {:?}", item_priority);
        acc = acc + item_priority;
    }
    println!("Total part 1: {acc}");
}

fn advent_day_3_part_2() {
    println!("Day 3 part 2-");
    let contents = fs::read_to_string("src/items.txt").expect("Cannot open file");
    let letters = ('a'..='z').chain('A'..='Z').collect::<String>();

    let total = contents
        .lines()
        .array_chunks::<3>()
        .map(|[a, b, c]| {
            a.chars()
                .find(|&a_char| b.contains(a_char) && c.contains(a_char))
                .unwrap()
        })
        .map(|common_char| letters.chars().position(|c| c == common_char).unwrap() + 1)
        .sum::<usize>();

    println!("Total part 2: {total}");
}

fn advent_day_4_part1() {
    println!("Day 4 part 1-");
    let contents = fs::read_to_string("src/sections.txt").expect("Cannot open file");

    let included = contents
        .lines()
        .map(|line| {
            line.split_once(",")
                .map(|(a, b)| {
                    let first_range = a
                        .split_once("-")
                        .map(|(num1, num2)| {
                            num1.parse::<u32>().unwrap()..=num2.parse::<u32>().unwrap()
                        })
                        .unwrap()
                        .collect::<Vec<_>>();
                    let second_range = b
                        .split_once("-")
                        .map(|(num1, num2)| {
                            num1.parse::<u32>().unwrap()..=num2.parse::<u32>().unwrap()
                        })
                        .unwrap()
                        .collect::<Vec<_>>();

                    first_range.iter().all(|z| second_range.contains(z))
                        || second_range.iter().all(|z| first_range.contains(z))
                })
                .unwrap()
        })
        .filter(|&res| res == true)
        .count();

    println!("Total included ranges: {included}");
}

fn advent_day_4_part2() {
    println!("Day 4 part 2-");
    let contents = fs::read_to_string("src/sections.txt").expect("Cannot open file");

    let included = contents
        .lines()
        .map(|line| {
            line.split_once(",")
                .map(|(a, b)| {
                    let first_range = a
                        .split_once("-")
                        .map(|(num1, num2)| {
                            num1.parse::<u32>().unwrap()..=num2.parse::<u32>().unwrap()
                        })
                        .unwrap()
                        .collect::<Vec<_>>();
                    let second_range = b
                        .split_once("-")
                        .map(|(num1, num2)| {
                            num1.parse::<u32>().unwrap()..=num2.parse::<u32>().unwrap()
                        })
                        .unwrap()
                        .collect::<Vec<_>>();

                    first_range.iter().any(|z| second_range.contains(z))
                        || second_range.iter().any(|z| first_range.contains(z))
                })
                .unwrap()
        })
        .filter(|&res| res == true)
        .count();

    println!("Total overlapping ranges: {included}");
}