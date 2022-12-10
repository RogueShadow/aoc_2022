extern crate test;

use std::str::FromStr;

pub fn day10(data: &str) {
    let ops = data.lines()
        .map(|l| match l {
            s if s.starts_with("addx") => {
                Ops::Addx(i32::from_str(l.split_once(' ').unwrap().1).unwrap())
            }
            s if s == "noop" => {Ops::Noop}
            _ => {panic!("Unsupported Op")}
        }).collect::<Vec<_>>();


    day10p1(&ops);
    day10p2(&ops);
}
pub fn day10p1(ops: &Vec<Ops>) {
    let mut cpu = CPU::new();
    cpu.set_watches(vec![20,60,100,140,180,220]);
    let xvalues = cpu.run_program(ops)
        .iter().map(|w| get_signal_strength(w))
        .collect::<Vec<_>>();

    println!("{:?}",xvalues.iter().sum::<i32>());
}
pub fn day10p2(ops: &Vec<Ops>) {
    let mut cpu = CPU::new();
    let watches = (0..=240).collect::<Vec<usize>>();
    cpu.set_watches(watches);
    let xvalues = cpu.run_program(ops);

    let mut result = String::new();

    for (cycle,x) in xvalues.iter() {
        let i = cycle % 40 - 1;
        if !(*x-1..=*x+1).contains(&(i as i32)) {result.push('.')} else {result.push('#')}
        if cycle % 40 == 0 {result.push('\n')}
    }

    println!("{}",result);
}

#[derive(Debug)]
pub enum Ops {
    Addx(i32),
    Noop,
}
#[derive(Debug)]
pub struct CPU {
    cycle: usize,
    x: i32,
    watches: Vec<usize>,
}
impl CPU {
    pub fn new() -> Self {
        Self {cycle: 0, x: 1, watches: vec![]}
    }
    pub fn set_watches(&mut self, watches: Vec<usize>) {
        self.watches = watches;
    }
    pub fn run_program(&mut self, program: &Vec<Ops>) -> Vec<(usize,i32)> {
        let mut watch: Vec<(usize,i32)> = vec![];
        for op in program {
            let mut do_watch = || {
                for _ in 0..get_cycles(op) {
                    self.cycle += 1;
                    if self.watches.contains(&self.cycle) {
                        watch.push((self.cycle,self.x));
                    }
                }
            };
            match op {
                Ops::Addx(v) => {
                    do_watch();
                    self.x += *v;
                }
                Ops::Noop => {
                    do_watch();
                }
            }
        }
        watch
    }
}
pub fn get_cycles(op: &Ops) -> usize {
    match op {
        Ops::Addx(_) => 2,
        Ops::Noop => 1,
    }
}
pub fn get_signal_strength(watch: &(usize,i32)) -> i32 {
    watch.0 as i32 * watch.1
}

pub mod tests {
    use std::fs::read_to_string;
    use super::*;
    use test::Bencher;

    #[bench]
    pub fn bench_day10(b: &mut Bencher) {
        let input = read_to_string("inputs/day10_input.txt").unwrap();
        b.iter(|| day10(&input));
    }
}