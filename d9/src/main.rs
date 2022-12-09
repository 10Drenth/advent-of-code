use std::collections::HashSet;

fn main() {
    let stack = include_str!("../input.txt")
             .lines().rev()
             .map(Move::from_line)
             .collect::<Vec<Move>>();
    println!("p1:       {}", calc_unique_positions(stack.clone(), 2));
    println!("p2:       {}", calc_unique_positions(stack.clone(), 10));
}

fn calc_unique_positions(mut stack: Vec<Move>, length: usize) -> usize {
    let mut rope = vec![Pos::new(0,0);length];
    let mut visited: HashSet<Pos> = HashSet::new();
    visited.insert(rope[length-1]);
    while let Some(m) = stack.pop(){
        for _ in 0..(m.count){
            rope[0] = rope[0].add(m.step);
            for i in 1..length{
                rope[i] = drag_tail(rope[i-1], rope[i]);
            }
            visited.insert(rope[length-1]);
        }
    }
    visited.len()
}

fn drag_tail(h: Pos, t: Pos) -> Pos {
    let diff = h.sub(t);
    if diff.x.abs() <= 1 && diff.y.abs() <= 1 { //Adjacent -> do nothing
        return t;
    }
    t.add(diff.signum())
}

#[derive(Debug, Hash, Eq, PartialEq, Copy, Clone)]
struct Pos{
    x: i32,
    y: i32
}

impl Pos{
    fn new(x: i32, y: i32) -> Pos{
        Pos{x, y}
    }

    fn add(&self, other: Pos) -> Pos {
        Pos::new(self.x + other.x, self.y + other.y)
    }

    fn sub(&self, other: Pos) -> Pos {
        Pos::new(self.x - other.x, self.y - other.y)
    }

    fn signum(&self) -> Pos{
        Pos::new(self.x.signum(), self.y.signum())
    }
}

#[derive(Debug, Clone)]
struct Move{
    step: Pos,
    count: u32
}

impl Move{
    fn from_line(line: &str) -> Move{
        let line_segments = line.split_whitespace().collect::<Vec<_>>();
        let count = line_segments[1].parse::<u32>().expect("");
        return match line_segments[0]{
            "R" => Move{
                step: Pos::new(1, 0),
                count
            },
            "U" => Move{
                step: Pos::new(0, 1),
                count
            },
            "L" => Move{
                step: Pos::new(-1, 0),
                count
            },
            _ => Move{
                step: Pos::new(0, -1),
                count
            }
        };
    }
}
