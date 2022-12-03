use std::env::args;
use std::fs::read_to_string;
use std::str::FromStr;

fn main() {
    let args = args().collect::<Vec<_>>();
    if args.len() < 2 {
        println!("Must supply day as an argument. program day [test]");
        return;
    }
    if let Ok(day) = i32::from_str(&args[1]) {
        let test = args.len() == 3 && args[2] == "test";
        if let Ok(input) = read_to_string(
            if !test { format!("day{}_input.txt", day) } else { format!("day{}_test.txt", day) }
        ) {
            match day {
                1 => day1::day1(input),
                2 => day2::day2(input),
                3 => day3::day3(input),
                _ => println!("Day {} not complete.", day)
            }
        } else {
            if !test {
                println!("No input for day {} found", day);
            } else {
                println!("No input for day {} test found.",day);
            }
        }
    } else {
        println!("Must supply day as an argument. program day [test]");
    }
}
mod util {
    use std::time::Instant;
    use colored::Colorize;

    pub(crate) struct Profiler {
        name: String,
        timer: Instant,
        global: Instant,
    }
    impl Profiler {
        pub fn new(title: &str) -> Self {
            Self {
                name: String::from(title),
                timer: Instant::now(),
                global: Instant::now(),
            }
        }
        pub fn log(&mut self, msg: &str) {
            let elapsed = self.timer.elapsed();
            self.timer = Instant::now();
            println!("{} - {}",msg.bright_green(),format!("{:?}",elapsed).red());
        }
        pub fn total(&self) {
            println!("{} - {} - {}",self.name.bright_green(),format!("{:?}",self.global.elapsed()).red(),"total".red());
        }
    }
}
mod day1 {
    use std::str::FromStr;
    use crate::util::Profiler;

    pub fn day1(data: String) {
        let mut prof = Profiler::new("Day 1, Part 1");
        prof.log("Read File");
        let input = data.lines().collect();
        prof.log("Get Lines");
        let elves = construct_elves(&input);
        prof.log("Construct Elves");
        let totals = calculate_elf_totals(&elves);
        prof.log("Calculate Calories");
        let top_three = &totals.as_slice()[0..=2].iter().fold(0,|acc,(_,c)| acc + c);
        prof.log("Aquire Top Three");
        prof.total();
        println!("{:?}",elves);
        println!("{:?}",totals);
        println!("Most calories: {:?} from elf {}",totals.first().expect("Elf").1,totals.first().expect("Elf").0);
        println!("Top three combined calories: {:?}",top_three);
        println!("{:?} Elves",elves.len());
        println!("{:?} food items.",&elves.iter().fold(0,|acc, items|{acc + items.food.len()}));
    }

    pub fn construct_elves(lines: &Vec<&str>) -> Vec<Elf> {
        println!("{:?}",lines);
        let mut elves = vec![];
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
        elves.push(elf);
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
mod day2 {
    use RPS::*;
    use WLD::*;
    use crate::util::Profiler;

    #[derive(Debug,Copy,Clone)]
    pub enum RPS {
        Rock,
        Paper,
        Scissors,
        Unknown,
    }
    #[derive(Debug,Copy,Clone)]
    pub enum WLD {
        Win,
        Lose,
        Draw,
        Unknown,
    }
    pub fn  day2(data: String) {
        let mut p = Profiler::new("Day 2 Part 1");
        // part 1.
        let input = data.split("\n")
            .map(|s| s.split(" ").collect::<Vec<_>>())
            .map(|s| (*s.first().unwrap(),*s.last().unwrap()))
            .map(|(them,you)| (map_data_rps(them),map_data_rps(you)))
            .collect::<Vec<_>>();
        p.log("Input Processed");
        let score = input.iter().fold(0,|acc,(them,you)|{acc + score_round(them,you)});
        p.log(format!("Played {} rounds of RPS",input.len()).as_str());
        println!("Score: {:?}",score);
        p.total();

        // part 2.
        let mut p = Profiler::new("Day 2 Part 2");
        let input = data.split("\n")
            .map(|s| s.split(" ").collect::<Vec<_>>())
            .map(|s| (*s.first().unwrap(),*s.last().unwrap()))
            .map(|(them,you)| (map_data_rps(them),map_win_lose_draw(you)))
            .map(|(them, you)| (them, play_to_wld(&them,&you)))
            .collect::<Vec<_>>();
        p.log("Processed Input");
        let score = input.iter().fold(0,|acc,(them,you)|{acc + score_round(them,you)});
        p.log(format!("Playing {} rounds of RPS",input.len()).as_str());
        println!("Score: {:?}",score);
        p.total();
    }
    pub fn map_win_lose_draw(value: &str) -> WLD {
        match value {
            "X" => Lose,
            "Y" => Draw,
            "Z" => Win,
            _ => WLD::Unknown,
        }
    }
    pub fn map_data_rps(value: &str) -> RPS {
        match value {
            "A" | "X" => Rock,
            "B" | "Y" => Paper,
            "C" | "Z" => Scissors,
            _ => RPS::Unknown
        }
    }
    pub fn play_to_wld(them: &RPS, you: &WLD) -> RPS {
        match (them, you) {
            (Rock, Lose) => Scissors,
            (Paper, Lose) => Rock,
            (Scissors, Lose) => Paper,
            (Rock, Draw) => Rock,
            (Paper, Draw) => Paper,
            (Scissors, Draw) => Scissors,
            (Rock, Win) => Paper,
            (Paper, Win) => Scissors,
            (Scissors, Win) => Rock,
            _ => RPS::Unknown
        }
    }
    pub fn score_round(them: &RPS, you: &RPS) -> i32 {
        match (them,you) {
            (Rock,Rock) => 1 + 3,
            (Paper,Rock) => 1 + 0,
            (Scissors,Rock) => 1 + 6,
            (Rock,Paper) => 2 + 6,
            (Paper,Paper) => 2 + 3,
            (Scissors,Paper) => 2 + 0,
            (Rock,Scissors) => 3 + 0,
            (Paper,Scissors) => 3 + 6,
            (Scissors,Scissors) => 3 + 3,
            _ => 0,
        }
    }
}
mod day3 {
    use crate::util::Profiler;

    #[derive(Debug)]
    pub struct Rucksack {
        a: String,
        b: String,
    }
    pub fn day3(data: String) {
        let mut p = Profiler::new("Day 3 Part 1");
        let input = data.lines()
            .map(|s| Rucksack {
                a: s[0..s.len() / 2].to_owned(),
                b: s[s.len() / 2..s.len()].to_owned(),
            }).collect::<Vec<_>>();
        p.log("Create Rucksack Vector from input");
        // part 1
        let sum = input.iter().map(|r| get_common_item(&r))
            .map(|i| score_item(&(i as u8)))
            .sum::<i32>();
        p.log("Find common item and score and sum.");
        println!("{:?}",sum);
        p.total();
        p = Profiler::new("Day 3 Part 2");
        let elf_groups = get_elf_badges(&input);
        p.total();
        println!("{:?}",elf_groups);
    }
    pub fn get_elf_badges(elves: &Vec<Rucksack>) -> i32 {
        let mut tally = 0;
        for (i,_) in elves.iter().enumerate().step_by(3) {
            let rs1 = elves[i].a.to_owned() + &elves[i].b;
            let rs2 = elves[i+1].a.to_owned() + &elves[i+1].b;
            let rs3 = elves[i+2].a.to_owned() + &elves[i+2].b;
            let common = get_common_item2(&rs1,&rs2,&rs3);
            let badge = score_item(&(common as u8));
            tally += badge;
        }
        tally
    }
    pub fn get_common_item2(list1: &str, list2: &str, list3: &str) -> char {
        let mut common = vec![];
        for c in list1.chars() {
           if list2.contains(c) {
               if !common.contains(&c) {
                   common.push(c);
               }
           };
        }
        for c in common {
            if list3.contains(c) {
                return c
            }
        }
        panic!("No common item found.")
    }
    pub fn get_common_item(sack: &Rucksack) -> char {
        for c in sack.a.chars().into_iter() {
            if sack.b.contains(c) {
                return c;
            }
        }
        ' '
    }
    pub fn score_item(item: &u8) -> i32 {
        let items = b"abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ";
        for (i,c) in items.iter().enumerate() {
            if item == c {return i as i32 + 1}
        }
        panic!("Couldn't find item.")
    }
}