use std::time::Instant;
extern crate test;

#[derive(Debug)]
pub struct Rucksack {
    a: String,
    b: String,
}
pub fn day3(data: String) {
    let timer = Instant::now();
    let input = data.lines()
        .map(|s| Rucksack {
            a: s[0..s.len() / 2].to_owned(),
            b: s[s.len() / 2..s.len()].to_owned(),
        }).collect::<Vec<_>>();
    // part 1
    let sum = input.iter().map(|r| get_common_item(&r))
        .map(|i| score_item(&(i as u8)))
        .sum::<i32>();
    println!("Completed in {:?}",timer.elapsed());
    println!("{:?}",sum);

    let timer = Instant::now();
    let elf_groups = get_elf_badges(&input);
    println!("Completed in {:?}",timer.elapsed());
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
#[cfg(test)]
mod tests {
    use super::*;
    use test::Bencher;

    #[bench]
    pub fn bench_day6p1(b: &mut Bencher) {
        b.iter(||{
            let value = day3(include_str!("../inputs/day3_input.txt").to_owned());
            test::black_box(value);
        });
    }
}