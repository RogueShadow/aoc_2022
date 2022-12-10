extern crate test;

use std::str::FromStr;
use image::ColorType;

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

    day9p1(&moves);
    day9p2(&moves);

}
pub fn day9p1(moves: &Vec<Move>) {
    use Move::*;
    let mut visited = vec![(0,0)];
    let mut head = Body {x: 0, y: 0};
    let mut tail = Body {x: 0, y: 0};
    let mut xsize = (0,0);
    let mut ysize = (0,0);

    let mut frame = 1;
    print_map(&head,&tail,&visited,(-268,48),(-192,89),0);
    for m in moves {
        match m {
            Up(n) => {
                for _ in 0..*n {
                    head.y -= 1;
                    move_tail(&head,&mut tail,Some(&mut visited));
                    print_map(&head,&tail,&visited,(-268,48),(-192,89),frame);
                    frame += 1;
                }
            }
            Down(n) => {
                for _ in 0..*n {
                    head.y += 1;
                    move_tail(&head,&mut tail,Some(&mut visited));
                    print_map(&head,&tail,&visited,(-268,48),(-192,89),frame);
                    frame += 1;
                }
            }
            Left(n) => {
                for _ in 0..*n {
                    head.x -= 1;
                    move_tail(&head,&mut tail,Some(&mut visited));
                    print_map(&head,&tail,&visited,(-268,48),(-192,89),frame);
                    frame += 1;
                }
            }
            Right(n) => {
                for _ in 0..*n {
                    head.x += 1;
                    move_tail(&head,&mut tail,Some(&mut visited));
                    print_map(&head,&tail,&visited,(-268,48),(-192,89),frame);
                    frame += 1;
                }
            }
        }
        println!("{}/11532",frame);
    }
    println!("Visited: {}",&visited.len());
}
pub fn move_tail(head: &Body, tail: &mut Body, visited: Option<&mut Vec<(i32,i32)>>) {
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

pub fn print_map(head: &Body, tail: &Body, visited: &Vec<(i32,i32)>,xsize: (i32,i32), ysize: (i32,i32), frame: i32) {
    let mut result = String::new();
    let width = (xsize.1 - xsize.0).abs();
    let height = (ysize.1 - ysize.0).abs();
    let mut image = image::RgbImage::new(width as u32,height as u32);

    for y in ysize.0..ysize.1 {
        for x in xsize.0..xsize.1 {
            let  c = match ((x,y) == (head.x,head.y),(x,y) == (tail.x,tail.y), visited.contains(&(x,y))) {
                (true,true,_) => {image.put_pixel((x + xsize.0.abs()) as u32, (y + ysize.0.abs()) as u32,image::Rgb([0,255,255]));'H'}
                (true,false,_) => {image.put_pixel((x + xsize.0.abs()) as u32, (y + ysize.0.abs()) as u32,image::Rgb([0,255,0]));'h'}
                (false,true,_) => {image.put_pixel((x + xsize.0.abs()) as u32, (y + ysize.0.abs()) as u32,image::Rgb([255,0,0]));'t'}
                (false,false,true) => {image.put_pixel((x + xsize.0.abs()) as u32, (y + ysize.0.abs()) as u32,image::Rgb([100,100,100]));'.'}
                (false,false,false) => {' '}
            };
            result.push(c);
        }
        result.push('\n');
    }
    image.save(format!("frames/{}_movie.png",frame)).expect("Save image.");
}
pub fn is_tail_touching(head: &Body, tail: &Body) -> bool {
    let result  = if (head.x-1..=head.x+1).contains(&tail.x) &&
        (head.y-1..=head.y+1).contains(&tail.y) {true}
    else {false};

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
                        move_tail(&one,two,None);
                    }
                    move_tail(&segments[8].clone(),&mut segments[9],Some(&mut visited));
                }
            }
            Down(n) => {
                for _ in 0..*n {
                    segments[0].y += 1;
                    for i in 0..segments.len()-2 {
                        let one = segments[i].clone();
                        let two = &mut segments[i+1];
                        move_tail(&one,two,None);
                    }
                    move_tail(&segments[8].clone(),&mut segments[9],Some(&mut visited));

                }
            }
            Left(n) => {
                for _ in 0..*n {
                    segments[0].x -= 1;
                    for i in 0..segments.len()-2 {
                        let one = segments[i].clone();
                        let two = &mut segments[i+1];
                        move_tail(&one,two,None);
                    }
                    move_tail(&segments[8].clone(),&mut segments[9],Some(&mut visited));

                }
            }
            Right(n) => {
                for _ in 0..*n {
                    segments[0].x += 1;
                    for i in 0..segments.len()-2 {
                        let one = segments[i].clone();
                        let two = &mut segments[i+1];
                        move_tail(&one,two,None);
                    }
                    move_tail(&segments[8].clone(),&mut segments[9],Some(&mut visited));

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
pub struct Body {
    x: i32,
    y: i32,
}