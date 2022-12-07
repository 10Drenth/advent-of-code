use std::cell::RefCell;
use std::rc::Rc;
use std::cmp;

fn main() {
    let instrs = parse_input(include_str!("../input.txt"));
    let fs = apply_instructions(instrs);
    println!("p1:       {}", fs.borrow().sum_under(100000));
    println!("p2:       {}", fs.borrow().smallest_above(fs.borrow().size() - 40000000));
}

#[derive(Debug)]
struct DirNode{
    files: Vec<i32>,
    sub_dirs: Vec<Rc<RefCell<DirNode>>>,
    par_dir: Option<Rc<RefCell<DirNode>>>
}

impl DirNode{
    fn new() -> DirNode{
        return DirNode{
            files: Vec::new(),
            sub_dirs: Vec::new(),
            par_dir: None
        };
    }

    fn size(&self) -> i32{
        self.files.iter().sum::<i32>() + 
            self.sub_dirs.iter().map(|sub| sub.borrow().size()).sum::<i32>()
    }

    fn sum_under(&self, n: i32) -> i32{
        let sum = self.sub_dirs.iter().map(|sub| sub.borrow().sum_under(n)).sum::<i32>();
        let my_size = self.size();
        if my_size > n{
            return sum;
        }
        sum + my_size
    }

    fn smallest_above(&self, n: i32) -> i32{
        let mut my_size = self.size();
        if my_size < n {
            my_size = i32::MAX;
        }
        return match self.sub_dirs
            .iter()
            .map(|sub| sub.borrow().smallest_above(n)).min(){
                Some(x) => cmp::min(x, my_size),
                None => my_size
            };
    }
}

#[derive(Debug)]
enum Instr{
    Cd(CdArg),
    Ls(Vec<i32>)
}

#[derive(Debug)]
enum CdArg{
    Up,
    Sub
}

fn apply_instructions(instrs: Vec<Instr>) -> Rc<RefCell<DirNode>>{
    let root = Rc::new(RefCell::new(DirNode::new()));
    let mut current = Rc::clone(&root);
    for instr in instrs.iter(){
        match instr{
            Instr::Ls(vals) => {
                current.borrow_mut().files = vals.clone();
            },
            Instr::Cd(CdArg::Sub) => {
                let child = Rc::new(RefCell::new(DirNode::new()));
                current.borrow_mut().sub_dirs.push(Rc::clone(&child));
                {
                    let mut mut_child = child.borrow_mut();
                    mut_child.par_dir = Some(Rc::clone(&current));
                }
                current = child;
            },
            Instr::Cd(CdArg::Up) => {
                current = Rc::clone(Rc::clone(&current).borrow().par_dir.as_ref().unwrap());
            }
        }
    }
    root
}

fn parse_input(input: &str) -> Vec<Instr>{
    let mut instrs: Vec<Instr> = Vec::new();
    let mut lines = input.lines().skip(1).collect::<Vec<_>>().into_iter().rev().collect::<Vec<_>>();
    while let Some(l) = lines.pop() {
        let parts = l.split_whitespace().collect::<Vec<_>>();
        instrs.push(match (parts[0], parts[1]){
            ("$", "cd") => {
                Instr::Cd(
                    match parts[2]{
                        ".." => CdArg::Up,
                        _ => CdArg::Sub
                    })
            },
            _ => {
                let mut files: Vec<i32> = Vec::new();
                while let Some(i) = lines.pop(){
                    let partsi = i.split_whitespace().collect::<Vec<_>>();
                    if partsi[0] == "$"{
                        lines.push(i);
                        break;
                    }
                    if let Ok(file) = partsi[0].parse::<i32>(){
                        files.push(file);
                    }
                }
                Instr::Ls(files)
            }
        });
    }
    instrs
}
