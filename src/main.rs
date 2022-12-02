
fn main() {
    day1::day1();
}

mod day1 {
    use std::str::FromStr;

    pub fn day1() {
        let timer = std::time::Instant::now();
        let input = include_str!("../input.txt").split("\n").collect::<Vec<_>>();
        println!("Get Input as lines: {:?}", timer.elapsed());
        let elves = construct_elves(&input);
        println!("Construct Elves from data: {:?}", timer.elapsed());
        let totals = calculate_elf_totals(&elves);
        println!("Calculate calories of elves: {:?}", timer.elapsed());
        let top_three = &totals.as_slice()[0..=2].iter().fold(0,|acc,(_,c)| acc + c);
        println!("Total time: {:?}", timer.elapsed());
        println!("Most calories: {:?} from elf {}",totals.first().expect("Elf").1,totals.first().expect("Elf").0);
        println!("Top three combined calories: {:?}",top_three);
        println!("{:?} Elves",elves.len());
        println!("{:?} food items.",&elves.iter().fold(0,|acc, items|{acc + items.food.len()}));
    }

    pub fn construct_elves(lines: &Vec<&str>) -> Vec<Elf> {
        let mut elves = vec![Elf { food: vec![] }];
        let mut elf = Elf { food: vec![] };
        for food in lines {
            if !food.is_empty() {
                if let Ok(calories) = i32::from_str(food) {
                    elf.food.push(calories);
                } else { println!("Invalid input") }
            } else {
                elves.push(elf);
                elf = Elf { food: vec![] };
            }
        }
        elves
    }

    #[derive(Debug)]
    pub struct Elf {
        pub(crate) food: Vec<i32>
    }

    pub fn calculate_elf_totals(elves: &Vec<Elf>) -> Vec<(usize, i32)> {
        let mut totals = vec![];
        for (i, elf) in elves.iter().enumerate() {
            let total = elf.food.iter().fold(0, |acc, x| acc + x);
            totals.push((i, total));
        }
        totals.sort_by(|(_, a), (_, b)| b.cmp(a));
        totals
    }
}