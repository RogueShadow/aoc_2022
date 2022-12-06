use std::str::FromStr;
use std::time::Instant;

pub fn day5(data: String) {
    let timer = Instant::now();
    let global = Instant::now();
    let data = data.split("\n\n").collect::<Vec<_>>();
    println!("Split Data - {:?}",timer.elapsed());
    let timer = Instant::now();
    let crates = data[0];
    let instructions = data[1].lines()
        .map(|s| s
            .split(' ')
            .collect::<Vec<_>>())
            .collect::<Vec<_>>();
    let instructions =
        instructions.iter().map(|s|
        (
            i32::from_str(s[1]).unwrap(),
            i32::from_str(s[3]).unwrap(),
            i32::from_str(s[5]).unwrap()
        )
    ).collect::<Vec<_>>();
    println!("Parsed Instructions - {:?}",timer.elapsed());
    let timer = Instant::now();
    let mut crates1 = get_crates(crates);
    let mut crates2 = crates1.clone();
    println!("Parsed Crates - {:?}",timer.elapsed());
    let timer = Instant::now();
    for (num,from,to) in instructions.iter() {
        perform_move(&mut crates1,*num,*from as usize,*to as usize);
    }
    println!("Perform moves 1 - {:?}",timer.elapsed());
    let timer = Instant::now();
    for (num,from,to) in instructions.iter() {
        perform_updated_move(&mut crates2, *num, *from as usize, *to as usize);
    }
    println!("Perform moves 2 - {:?}",timer.elapsed());
    let timer = Instant::now();
    let result = &crates1.iter().map(|s| s.chars().last().unwrap()).collect::<String>();
    println!("{:?}",result);


    let result = &crates2.iter().map(|s| s.chars().last().unwrap()).collect::<String>();
    println!("{:?}",result);
    println!("Got final crate positions - {:?}",timer.elapsed());
    println!("Completed in {:?}",global.elapsed());

}

pub fn perform_move(crates: &mut Vec<String>, number: i32, from: usize, to: usize) -> &mut Vec<String> {
    let from = from - 1;
    let to = to - 1;
    for _ in 0..number {
        let c = crates[from].pop().unwrap();
        crates[to].push(c);
    }
    crates
}

pub fn perform_updated_move(crates: &mut Vec<String>, number: i32, from: usize, to: usize) -> &mut Vec<String> {
    let c_from = &crates[from - 1];
    let start = c_from.len() - number as usize;
    let end = c_from.len();
    let slice = &c_from[start..end].to_owned();
    let c_to = &mut crates[to - 1];
    *c_to += slice;
    let cfrom = &mut crates[from - 1];
    let _ = cfrom.drain(start..end);
    crates
}

pub fn get_crates(data: &str) -> Vec<String> {
    let lines = data.lines().collect::<Vec<_>>();
    let mut result = String::new();
    let h = lines.len();
    let w = lines[0].len();
    let mut addline = false;
    for x in 0..w {
        for y in 0..h {
            let c = &lines[y][x..x+1].chars().collect::<Vec<_>>()[0];
            if !"[] ".contains(*c) {result.push(*c);addline = true;};
        }
        if addline {
            result.push('\n');
            addline = false;
        }
    }
    let mut crates = vec![];
    for l in result.lines() {
        let new_line = l[0..l.len()-1].chars().rev().collect::<String>().to_owned();
        crates.push(new_line);
    }
    crates
}