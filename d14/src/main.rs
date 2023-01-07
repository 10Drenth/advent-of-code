fn main() {
    p1();
    p2();
}

fn p2(){
    let rocks: Vec<Vec<(u32, u32)>> = include_str!("../input.txt")
        .lines()
        .map(parse_ln)
        .collect();
    let lowest_point = rocks.iter().flatten().map(|(_, y)| y).max().unwrap() + 2;
    let mut sands: Vec<(u32, u32)> = Vec::new();
    let mut x: u32 = 500;
    let mut y: u32 = 0;
    loop {
        let mut moved: bool = false;
        for p@(x1, y1) in [(x, y+1), (x-1, y+1), (x+1, y+1)].iter(){
            if is_available(&p, &rocks, &sands) && y1 > &lowest_point{
                x = *x1;
                y = *y1;
                moved = true;
                break;
            }
        }
        if !moved{
            sands.push((x, y));
            if y == 0 && x == 500{
                break;
            }
            x = 500;
            y = 0;
        }
    }
    println!("p2:       {}", sands.len());
}

fn p1(){
    let rocks: Vec<Vec<(u32, u32)>> = include_str!("../input.txt")
        .lines()
        .map(parse_ln)
        .collect();
    let lowest_point = rocks.iter().flatten().map(|(_, y)| y).max().unwrap();
    let mut sands: Vec<(u32, u32)> = Vec::new();
    let mut x: u32 = 500;
    let mut y: u32 = 0;
    loop {
        if y >= *lowest_point{
            break;
        }
        let mut moved: bool = false;
        for p@(x1, y1) in [(x, y+1), (x-1, y+1), (x+1, y+1)].iter(){
            if is_available(&p, &rocks, &sands){
                x = *x1;
                y = *y1;
                moved = true;
                break;
            }
        }
        if !moved{
            sands.push((x, y));
            x = 500;
            y = 0;
        }
    }
    println!("p1:       {}", sands.len());
}

fn is_available(pos: &(u32, u32), rocks: &Vec<Vec<(u32, u32)>>, sands: &Vec<(u32, u32)>) -> bool{
    !(sands.contains(pos) || rocks.iter().any(|seq| lines_contain(pos, seq)))
}

fn lines_contain((x, y): &(u32, u32), rocks: &Vec<(u32, u32)>) -> bool{
    rocks.iter().zip(rocks.clone().into_iter().skip(1))
        .any(|((x1, y1), (x2, y2))|{
            (x1 == x && &x2 == x && ((y1 <= y && y <= &y2) || (&y2 <= y && y <= y1)))
                ||
            (y1 == y && &y2 == y && ((x1 <= x && x <= &x2) || (&x2 <= x && x <= x1)))
    })
}

fn parse_ln(ln: &str) -> Vec<(u32, u32)>{
    ln.split(" -> ")
        .map(parse_point)
        .collect()
}

fn parse_point(input: &str) -> (u32, u32){
    let (l, r) = input.split_once(",").unwrap();
    (l.parse().unwrap(), r.parse().unwrap())
}
