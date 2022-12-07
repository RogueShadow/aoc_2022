extern crate test;

pub fn day6(data: &str) {
    println!("{}",day6p1(data).unwrap());
    println!("{}",day6p2(data).unwrap());
}
pub fn day6p1(data: &str) -> Option<usize> {
    find_marker(&data,4)
}
pub fn day6p2(data: &str) -> Option<usize> {
    find_marker(&data,14)
}

pub fn find_marker(stream: &str, len: usize) -> Option<usize> {
    for i in 0..stream.len() - len {
        let window = &stream[i..i + len];
        if is_unique(window) {
            return Some(i + len);
        }
    }
    None
}

pub fn is_unique(value: &str) -> bool {
    for i in 0..value.len() {
        let char = &value[i..i+1];
        for t in i+1..value.len() {
            if char == &value[t..t+1] {
                return false;
            }
        }
    }
    true
}

#[cfg(test)]
mod tests {
    use super::*;
    use test::Bencher;

    #[bench]
    pub fn bench_day6p1(b: &mut Bencher) {
        b.iter(||{
            let value = day6p1(include_str!("../day6_input.txt")).unwrap();
            test::black_box(value);
        });
    }
    #[bench]
    pub fn bench_day6p2(b: &mut Bencher) {
        b.iter(||{
            let value = day6p2(include_str!("../day6_input.txt")).unwrap();
            test::black_box(value);
        });
    }
}
