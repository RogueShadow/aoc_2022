
fn main() {
    day1::day1();
    day2::day2();
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
mod day2 {
    use RPS::*;
    use WLD::*;

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
    pub fn  day2() {

        // part 1.
        let input = include_str!("../day2_input.txt")
            .split("\n")
            .map(|s| s.split(" ").collect::<Vec<_>>())
            .map(|s| (*s.first().unwrap(),*s.last().unwrap()))
            .map(|(them,you)| (map_data_rps(them),map_data_rps(you)))
            .collect::<Vec<_>>();
        let score = input.iter().fold(0,|acc,(them,you)|{acc + score_round(them,you)});
        println!("Score: {:?}",score);

        // part 2.
        let input = include_str!("../day2_input.txt")
            .split("\n")
            .map(|s| s.split(" ").collect::<Vec<_>>())
            .map(|s| (*s.first().unwrap(),*s.last().unwrap()))
            .map(|(them,you)| (map_data_rps(them),map_win_lose_draw(you)))
            .map(|(them, you)| (them, play_to_wld(&them,&you)))
            .collect::<Vec<_>>();
        let score = input.iter().fold(0,|acc,(them,you)|{acc + score_round(them,you)});
        println!("Score: {:?}",score);

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