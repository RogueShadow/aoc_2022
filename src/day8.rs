extern crate test;

use std::collections::HashMap;
use std::str::FromStr;

pub fn day8(data: &str) {
    day8p1(data);
    day8p2(data);
}
pub fn day8p1(data: &str) {
    let forest = data
        .lines()
        .map(|l| l
            .chars()
            .map(|c| i32::from_str(
                c.to_string().as_str()
            ).unwrap())
            .collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let map = tree_vis_map(&forest);

    let visible_trees = map.values().filter(|v| **v == true).count();
    println!("Visible Trees: {:?}",visible_trees);

}
pub fn day8p2(data: &str) {
    let forest = data
        .lines()
        .map(|l| l
            .chars()
            .map(|c| i32::from_str(
                c.to_string().as_str()
            ).unwrap())
            .collect::<Vec<_>>())
        .collect::<Vec<_>>();
    let mut score = 0;
    for x in 0..forest.len() {
        for y in 0..forest[0].len() {
            score = score.max(tree_view_score(&forest,x,y));
        }
    }
    println!("Score: {}",score);
}
pub fn tree_view_score(forest: &Vec<Vec<i32>>, x: usize, y: usize) -> i32 {
    let tree = forest[y][x];

    let mut left = 0;
    let mut right = 0;
    let mut top = 0;
    let mut bottom = 0;

    'left: for ix in (0..x).rev() {
        let other = forest[y][ix];
        if other >= tree {
            left += 1;
            break 'left;
        }
        if other < tree {
            left += 1;
        }
    }
    'right: for ix in x+1..forest[0].len() {
        let other = forest[y][ix];
        if other >= tree {
            right += 1;
            break 'right;
        }
        if other < tree {
            right += 1;
        }
    }
    'top: for iy in (0..y).rev() {
        let other = forest[iy][x];
        if other >= tree {
            top += 1;
            break 'top;
        }
        if other < tree {
            top += 1;
        }
    }
    'bottom: for iy in y+1..forest.len() {
        let other = forest[iy][x];

        if other >= tree {
            bottom += 1;
            break 'bottom;
        }
        if other < tree {
            bottom += 1;
        }
    }
    //println!("Tree {}x{} scores L{} * R{} * T{} * B{}",x,y,left,right,top,bottom);
    left * right * top * bottom
}
pub fn tree_vis_map(forest: &Vec<Vec<i32>>) -> HashMap<(usize,usize),bool> {
    let mut map = HashMap::new();
    for x in 0..forest[0].len() {
        for y in 0..forest.len() {
            map.insert((x,y),tree_visible(forest,x,y));
        }
    }
    map
}
pub fn tree_visible(forest: &Vec<Vec<i32>>, x: usize, y: usize) -> bool {
    let tree = forest[x][y];

    if (x == 0 || x == forest[x].len() - 1) || (y == 0 || y == forest.len() - 1)  { return true }

    let mut left = true;
    let mut right = true;
    let mut top = true;
    let mut bottom = true;

    for ix in 0..x {
        if forest[ix][y] >= tree { left = false }
    }
    for ix in x+1..forest[0].len() {
        if forest[ix][y] >= tree { right = false }
    }
    for iy in 0..y {
        if forest[x][iy] >= tree { top = false }
    }
    for iy in y+1..forest.len() {
        if forest[x][iy] >= tree {bottom = false }
    }

    left || right || top || bottom
}

#[allow(unused,unused_imports)]
mod tests {
    use std::fs::read_to_string;
    use super::*;
    use test::Bencher;

    #[bench]
    pub fn bench_day8p1(b: &mut Bencher) {
        let input = read_to_string("day8_input.txt").unwrap();
        b.iter(|| day8p1(&input));
    }
    #[bench]
    pub fn bench_day8p2(b: &mut Bencher) {
        let input = read_to_string("day8_input.txt").unwrap();
        b.iter(|| day8p2(&input));
    }
}