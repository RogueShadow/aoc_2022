extern crate test;

use std::str::FromStr;

pub fn day9(data: &str) {
    let moves = data
        .lines()
        .map(|l| l
            .split_once(' ')
            .map(|(a,b)| match a {
                "U" => Move::Up(i32::from_str(b).unwrap()),
                "D" => Move::Down(i32::from_str(b).unwrap()),
                "L" => Move::Left(i32::from_str(b).unwrap()),
                "R" => Move::Right(i32::from_str(b).unwrap()),
                _ => {panic!("Bad Move.")}
            }).unwrap()
        ).collect::<Vec<_>>();
    println!("Moves {:?}",moves);
    day9p1(&moves);
    day9p2(&moves);

}
pub fn day9p1(moves: &Vec<Move>) {
    use Move::*;
    let mut visited = vec![(0,5)];
    let mut head = Head {x: 0, y: 5};
    let mut tail = Tail {x: 0, y: 5};
    print_map(&head,&tail,&visited);
    for m in moves {
        match m {
            Up(n) => {
                for _ in 0..*n {
                    head.y -= 1;
                    move_tail(&head,&mut tail,Some(&mut visited));
                    print_map(&head,&tail,&visited);
                }
            }
            Down(n) => {
                for _ in 0..*n {
                    head.y += 1;
                    move_tail(&head,&mut tail,Some(&mut visited));
                    print_map(&head,&tail,&visited);
                }
            }
            Left(n) => {
                for _ in 0..*n {
                    head.x -= 1;
                    move_tail(&head,&mut tail,Some(&mut visited));
                    print_map(&head,&tail,&visited);
                }
            }
            Right(n) => {
                for _ in 0..*n {
                    head.x += 1;
                    move_tail(&head,&mut tail,Some(&mut visited));
                    print_map(&head,&tail,&visited);
                }
            }
        }

    }
    println!("Visited: {}",&visited.len());
}
pub fn move_tail(head: &Head, tail: &mut Tail, visited: Option<&mut Vec<(i32,i32)>>) {
    if is_tail_touching(head,tail) {return};
    match (head.x == tail.x, head.y == tail.y, head.x > tail.x, head.y > tail.y) {
        (true,true,_,_) => {println!("This shouldn't happen")}
        (false,true,xg,_) => {tail.x += if xg {1} else {-1}}
        (true,false,_,yg) => {tail.y += if yg {1} else {-1}}
        (false,false,xg,yg) => {tail.x += if xg {1} else {-1}; tail.y += if yg {1} else {-1}}
    }
    if let Some(list) = visited {
        if !list.contains(&(tail.x,tail.y)) {
            list.push((tail.x,tail.y))
        }
    }
}
pub fn move_tail2(head: &Body, tail: &mut Body, visited: Option<&mut Vec<(i32,i32)>>) {
    if is_tail_touching2(head,tail) {return};
    match (head.x == tail.x, head.y == tail.y, head.x > tail.x, head.y > tail.y) {
        (true,true,_,_) => {println!("This shouldn't happen")}
        (false,true,xg,_) => {tail.x += if xg {1} else {-1}}
        (true,false,_,yg) => {tail.y += if yg {1} else {-1}}
        (false,false,xg,yg) => {tail.x += if xg {1} else {-1}; tail.y += if yg {1} else {-1}}
    }
    if let Some(list) = visited {
        if !list.contains(&(tail.x,tail.y)) {
            list.push((tail.x,tail.y))
        }
    }
}
pub fn print_map(head: &Head, tail: &Tail, visited: &Vec<(i32,i32)>) {
    for y in 0..7 {
        for x in 0..7 {
            let  c = match ((x,y) == (head.x,head.y),(x,y) == (tail.x,tail.y), visited.contains(&(x,y))) {
                (true,true,_) => {'H'}
                (true,false,_) => {'h'}
                (false,true,_) => {'t'}
                (false,false,true) => {'o'}
                (false,false,false) => {'.'}
            };
            print!("{} ",c);
        }
        print!("\n");
    }
    println!("-----------");
}
pub fn is_tail_touching(head: &Head, tail: &Tail) -> bool {
    let result  = if (head.x-1..=head.x+1).contains(&tail.x) &&
        (head.y-1..=head.y+1).contains(&tail.y) {true}
    else {false};

    //println!("H {}x{} T {}x{} {}",head.x,head.y,tail.x,tail.y,result);

    result
}
pub fn is_tail_touching2(head: &Body, tail: &Body) -> bool {
    let result  = if (head.x-1..=head.x+1).contains(&tail.x) &&
        (head.y-1..=head.y+1).contains(&tail.y) {true}
    else {false};

    //println!("H {}x{} T {}x{} {}",head.x,head.y,tail.x,tail.y,result);

    result
}
pub fn day9p2(moves: &Vec<Move>) {
    use Move::*;
    let mut visited = vec![(0,5)];
    let mut segments = (0..10).map(|n| Body {x: 0, y: 5}).collect::<Vec<_>>();
    for m in moves {
        match m {
            Up(n) => {
                for _ in 0..*n {
                    segments[0].y -= 1;
                    for i in 0..segments.len()-2 {
                        let one = segments[i].clone();
                        let two = &mut segments[i+1];
                        move_tail2(&one,two,None);
                    }
                    move_tail2(&segments[8].clone(),&mut segments[9],Some(&mut visited));
                }
            }
            Down(n) => {
                for _ in 0..*n {
                    segments[0].y += 1;
                    for i in 0..segments.len()-2 {
                        let one = segments[i].clone();
                        let two = &mut segments[i+1];
                        move_tail2(&one,two,None);
                    }
                    move_tail2(&segments[8].clone(),&mut segments[9],Some(&mut visited));

                }
            }
            Left(n) => {
                for _ in 0..*n {
                    segments[0].x -= 1;
                    for i in 0..segments.len()-2 {
                        let one = segments[i].clone();
                        let two = &mut segments[i+1];
                        move_tail2(&one,two,None);
                    }
                    move_tail2(&segments[8].clone(),&mut segments[9],Some(&mut visited));

                }
            }
            Right(n) => {
                for _ in 0..*n {
                    segments[0].x += 1;
                    for i in 0..segments.len()-2 {
                        let one = segments[i].clone();
                        let two = &mut segments[i+1];
                        move_tail2(&one,two,None);
                    }
                    move_tail2(&segments[8].clone(),&mut segments[9],Some(&mut visited));

                }
            }
        }

    }
    println!("Visited: {}",&visited.len());
}


#[derive(Debug,Copy,Clone)]
pub enum Move {
    Up(i32),
    Down(i32),
    Left(i32),
    Right(i32),
}
#[derive(Debug,Copy,Clone)]
pub struct Head {
    x: i32,
    y: i32,
}
#[derive(Debug,Copy,Clone)]
pub struct Tail {
    x: i32,
    y: i32,
}
#[derive(Debug,Copy,Clone)]
pub struct Body {
    x: i32,
    y: i32,
}