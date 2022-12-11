extern crate test;
use std::collections::HashMap;
use std::str::FromStr;

pub fn day7(data: String) {
    day7p1(data.clone());
    day7p2(data.clone());
}
pub fn day7p1(data: String)  {
    let mut fs = FsBuilder::new();
    fs.build(data);
    //println!("{}",fs.get_path("/").size());
    //println!("{:?}",fs.fs);
    println!("{:?}",fs.fs.scan_for_size().iter().filter(|n|  **n < 100000_u32 ).sum::<u32>());
}
pub fn day7p2(data: String) {
    let mut fs = FsBuilder::new();
    fs.build(data);
    let total_size = 70000000_u32;
    let required_space = 30000000_u32;
    //println!("{:?}",fs.fs.scan_for_size().iter().sum::<u32>());
    let used_space = fs.fs.size();
   // println!("Used Space: {}",used_space);
    let unused_space = total_size - used_space;
   // println!("Free Space: {}",unused_space);
    let space_needed = required_space - unused_space;
   // println!("{:?}",fs.fs.scan_for_size_with_name());
    let dirs = fs.fs.scan_for_size_with_name();
    let mut filtered_dirs = dirs.iter().filter(|(_, f)| *f > space_needed).collect::<Vec<_>>();
    filtered_dirs.sort_by(|(_,s),(_,s1)| s.cmp(s1) );
    println!("{:?}",filtered_dirs.first().unwrap().1);

}

#[derive(Debug)]
pub struct FsBuilder {
    pub fs: Folder,
    pub current_dir: String,
}
impl FsBuilder{
    pub fn get_folder(&mut self) -> &mut Folder {
        let mut result= &mut self.fs;
        if self.current_dir == "/" {return result}
        let path = self.current_dir[1..].split('/').collect::<Vec<_>>();
        for p in path {
            if !p.is_empty() {result = result.folders.get_mut(p).unwrap()};
        }
        result
    }
    pub fn new() -> Self {
        Self {
            fs: Folder::new(),
            current_dir: "/".to_owned(),
        }
    }
    pub fn build(&mut self, data: String) {
        let mut reading_data = false;
        let mut output = vec![];
        for line in data.lines() {
            match &line[0..1] {
                "$" => {
                    if reading_data {
                        reading_data = false;
                    }
                    let tokens = &line[2..].split(' ').collect::<Vec<_>>();
                    match tokens[0] {
                        "cd" => {
                            if tokens[1] == ".." {
                                let dir = self.current_dir.rfind('/').unwrap();
                                self.current_dir.drain(dir..);
                            } else {
                                if tokens[1] == "/" {
                                    self.current_dir = "/".to_owned();
                                }else {
                                    let folder = self.get_folder();
                                    folder.folders.insert(tokens[1].to_owned(),Folder::new());
                                    self.current_dir.push_str(&*("/".to_owned() + &tokens[1]));
                                }
                            }
                        }
                        "ls" => {}
                        _ => {}
                    }
                },
                _ => {
                    if !reading_data {
                        reading_data = true;
                    }
                    if reading_data {
                        let file = line.split_once(' ').unwrap();
                        match file {
                            ("dir",name) => {
                                let current_folder = self.get_folder();
                                current_folder.folders.insert(name.to_owned(),Folder::new());
                            },
                            (size,name) => {
                                let current_folder = self.get_folder();
                                current_folder.files.push(File {_name: name.to_owned(), size: u32::from_str(size).unwrap()});
                            }
                        }
                        output.push(line.to_owned());
                    }
                }
            }
        }
    }
}
#[derive(Debug)]
pub struct Folder {
    pub folders: HashMap<String,Folder>,
    pub files: Vec<File>,
}
impl Folder {
    pub fn new() -> Self {
        Self {
            folders: HashMap::new(),
            files: vec![],
        }
    }
    pub fn size(&self) -> u32 {
        let size = self.files.iter().map(|f| f.size).sum::<u32>();
        let fsize = self.folders.iter().map(|(_,f)| f.size()).sum::<u32>();
        size + fsize
    }
    pub fn scan_for_size(&self) -> Vec<u32> {
        let mut result = vec![];
        for (_,f) in &self.folders {
            result.push(f.size());
            result.extend(f.scan_for_size())
        }
        result
    }
    pub fn scan_for_size_with_name(&self) -> Vec<(String,u32)> {
        let mut result = vec![];
        for (n,f) in &self.folders {
            result.push((n.to_owned(),f.size()));
            result.extend(f.scan_for_size_with_name())
        }
        result
    }
}
#[derive(Debug)]
pub struct File {
    _name: String,
    size: u32,
}

#[allow(unused)]
mod tests {
    use super::*;
    use test::Bencher;

    #[bench]
    pub fn bench_day7p1(b: &mut Bencher) {
        b.iter(||{
            day7p1(include_str!("../inputs/day7_input.txt").to_owned());
        });
    }
    #[bench]
    pub fn bench_day7p2(b: &mut Bencher) {
        b.iter(||{
            day7p2(include_str!("../inputs/day7_input.txt").to_owned());
        });
    }
}