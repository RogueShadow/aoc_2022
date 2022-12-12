extern crate test;

use std::collections::HashMap;


pub fn day11(data: &str) {
    let mut monkeys = parse_monkeys(data);
    day11p1(&mut monkeys);
}
pub fn parse_monkeys(data: &str) -> Vec<Monkey> {
    let monkey_data = data
        .split("\n\n")
        .collect::<Vec<_>>();
    monkey_data.iter().map(|d| get_monkey(d)).collect::<Vec<_>>()
}
pub fn day11p1(monkeys: &mut Vec<Monkey>) {
    for _ in 0..10000 {
        for i in 0..monkeys.len() {
            let monkey = &mut monkeys[i];
            let throws = monkey.take_turn();
            for t in throws {
                monkeys[t.monkey].items.push(t.item);
            }
        }
    }
    //println!("{:?}",&monkeys);
    let mut monkeyi = monkeys.iter().map(|m| m.inspected.to_owned()).collect::<Vec<_>>();
    monkeyi.sort();
    monkeyi.reverse();
    println!("Monkey Business: {}",monkeyi[0].to_owned() * monkeyi[1].to_owned())
}

pub fn get_monkey(data: &str) -> Monkey {
    let mut monkey = Monkey::new();
    data.lines().enumerate().for_each(|(i,m)| {
        match i {
            1 => {
                let tokens = m.rsplit_once(':').unwrap().1.split(',')
                    .map(|n| Item::new(u128::from(n.trim().parse::<u128>().unwrap()) as f64)).collect();
                monkey.items = tokens;
            }
            2 => {
                let tokens = m.rsplit_once('=').unwrap().1;
                let tokens = tokens.split_whitespace().collect::<Vec<_>>();
                if tokens[2] == "old" {monkey.op = Ops::Square} else {
                    let n = tokens[2].parse::<u128>().unwrap();
                    if tokens[1] == "+" {monkey.op = Ops::Add(u128::from(n))};
                    if tokens[1] == "*" {monkey.op = Ops::Mul(u128::from(n))};
                }
            }
            3 => {
                let tokens = m.rsplit_once(' ').unwrap().1.parse::<u32>().unwrap();
                monkey.test = tokens;
            }
            4 => {
                let tokens = m.rsplit_once(' ').unwrap().1.parse::<u128>().unwrap();
                monkey.throw.0 = tokens as usize;
            }
            5 => {
                let tokens = m.rsplit_once(' ').unwrap().1.parse::<u128>().unwrap();
                monkey.throw.1 = tokens as usize;
            }
            _ => {}
        }
    });
    monkey
}
#[derive(Debug)]
pub struct Monkey {
    items: Vec<Item>,
    test: u32,
    op: Ops,
    throw: (usize,usize),
    inspected: u128,
}
impl Monkey {
    pub fn new() -> Self {
        Self {
            items: vec![],
            test: 0,
            op: Ops::Mul(1),
            throw: (0,0),
            inspected: 0,
        }
    }
    pub fn take_turn(&mut self) -> Vec<Throw> {
        let mut throw = vec![];
        for i in self.items.iter_mut() {
            match &self.op {
                Ops::Mul(n) => {i.mul(*n as f64)}
                Ops::Add(n) => {i.add(*n as f64)}
                Ops::Square => {i.square()}
            };
            if i.map[&self.test] == 0.0 {
                throw.push(Throw::new(i.to_owned(), self.throw.0));
            }else{
                throw.push(Throw::new(i.to_owned(), self.throw.1));
            }
            self.inspected += 1;
        }
        let remove_items = throw.iter().map(|t| &t.item).collect::<Vec<_>>();
        self.items.drain_filter(|f| remove_items.contains(&&*f));
        throw
    }
}
#[derive(Debug)]
pub enum Ops {
    Mul(u128),
    Add(u128),
    Square,
}
#[derive(Debug,Clone)]
pub struct Throw {
    item: Item,
    monkey: usize,
}
impl Throw {
    pub fn new(item: Item, monkey: usize) -> Self {
        Self {
            item,
            monkey,
        }
    }
}

#[derive(Debug,Clone,PartialEq)]
pub struct Item {
    map: HashMap<u32,f64>
}
impl Item {
    const PRIMES: [u32; 9] = [2,3,5,7,11,13,17,19,23];
    pub fn new(n: f64) -> Self {
        let mut map = HashMap::new();
        for p in Item::PRIMES {
            map.insert(p,n as f64 % p as f64);
        }
        Self {
            map,
        }
    }
    pub fn add(&mut self, n: f64) {
        for i in 0..Item::PRIMES.len() {
            let p = *Item::PRIMES.get(i).unwrap();
            let v = *self.map.get(&p).unwrap();
            self.map.insert(p,(v + n) % p as f64);
        }
    }
    pub fn mul(&mut self, n: f64) {
        for i in 0..Item::PRIMES.len() {
            let p = *Item::PRIMES.get(i).unwrap();
            let v = *self.map.get(&p).unwrap();
            self.map.insert(p,(v * n) % p as f64);
        }
    }
    pub fn square(&mut self) {
        for i in 0..Item::PRIMES.len() {
            let p = *Item::PRIMES.get(i).unwrap();
            let v = *self.map.get(&p).unwrap();
            self.map.insert(p,(v * v) % p as f64);
        }
    }
}

pub mod tests {
    use std::fs::read_to_string;
    use super::*;
    use test::Bencher;

    #[bench]
    pub fn bench_day11_parsing(b: &mut Bencher) {
        let input = read_to_string("inputs/day11_input.txt").unwrap();
        b.iter(|| parse_monkeys(&input));
    }
    #[bench]
    pub fn bench_day11(b: &mut Bencher) {
        let input = read_to_string("inputs/day11_input.txt").unwrap();
        b.iter(|| day11(&input))
    }
}