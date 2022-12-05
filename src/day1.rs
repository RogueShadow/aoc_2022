use std::str::FromStr;
use std::time::Instant;

pub fn day1(data: String) {
    let timer = Instant::now();
    let input = data.lines().collect();
    let elves = construct_elves(&input);
    let top_three = &elves.as_slice()[0..=2].iter().fold(0,|acc,e| acc + e.total);
    println!("Completed in {:?}",timer.elapsed());
    println!("Most calories: {:?}",elves.first().expect("Elf").total);
    println!("Top three combined calories: {:?}",top_three);
    println!("{:?} Elves",elves.len());
    println!("{:?} food items.",&elves.iter().fold(0,|acc, items|{acc + items.food.len()}));
}

pub fn construct_elves(lines: &Vec<&str>) -> Vec<Elf> {
    let mut elves = vec![];
    let mut elf = Elf { food: vec![], total: 0 };
    for food in lines {
        if !food.is_empty() {
            if let Ok(calories) = i32::from_str(food) {
                elf.food.push(calories);
                elf.total += calories;
            } else { println!("Invalid input") }
        } else {
            elves.push(elf);
            elf = Elf { food: vec![], total: 0 };
        }
    }
    elves.push(elf);
    elves.sort_by(|a,b| b.total.cmp(&a.total));
    elves
}

#[derive(Debug)]
pub struct Elf {
    pub food: Vec<i32>,
    pub total: i32,
}
