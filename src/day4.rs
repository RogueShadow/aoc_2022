use std::str::FromStr;
use std::time::Instant;
extern crate test;

pub fn day4(data: String) {
    pub fn solver(data: &str, cmp: impl Fn(Vec<Vec<i32>>) -> bool) -> usize {
        data.lines()
            .map(|l| l.split(',')
                .collect::<Vec<_>>())
            .map(|r| {
                r.iter()
                    .map(|v| v.split('-')
                        .map(|n| i32::from_str(n).unwrap_or(0))
                        .collect::<Vec<_>>())
                    .collect::<Vec<_>>()
            })
            .map(|r| cmp(r))
            .filter(|v| *v == true )
            .collect::<Vec<_>>()
            .len()
    }
    let contained = |r: Vec<Vec<i32>>| {
        if (r[0][0] >= r[1][0] &&
            r[0][1] <= r[1][1]) ||
            (r[1][0] >= r[0][0] &&
                r[1][1] <= r[0][1])
        {true} else {false}
    };
    let timer = Instant::now();
    let result = solver(data.as_str(),contained);
    println!("Completed in {:?}",timer.elapsed());
    println!("{}",result);
    let overlap = |r: Vec<Vec<i32>>| {
        if (r[0][1] >= r[1][0] &&
            r[0][0] <= r[1][1]) ||
            (r[1][1] >= r[0][0] &&
                r[1][0] <= r[0][1])
        {true} else {false}
    };
    let timer = Instant::now();
    let result = solver(data.as_str(),overlap);
    println!("Completed in {:?}",timer.elapsed());
    println!("{}",result);
}

#[cfg(test)]
mod tests {
    use super::*;
    use test::Bencher;

    #[bench]
    pub fn bench_day4(b: &mut Bencher) {
        b.iter(||{
            let value = day4(include_str!("../day4_input.txt").to_owned());
            test::black_box(value);
        });
    }
}