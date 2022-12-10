fn main() {
    let instrs = include_str!("../input.txt")
        .lines().flat_map(parse_instr);
    let mut cyc: u32 = 1;
    let mut x: i32 = 1;
    let mut intervals: Vec<u32> = vec![220, 180, 140, 100, 60, 20];
    let mut interval: Option<u32> = intervals.pop();
    let mut sum: i32 = 0;
    println!("P2:");
    for instr in instrs{
        match interval {
            None => interval = intervals.pop(),
            _ => {}
        };
        match interval {
            Some(z) if z == cyc => {
                sum += (cyc as i32) * x;
                interval = None;
            },
            _ => {}
        };

        draw_pixel(x, (cyc - 1) as i32);

        cyc += 1;

        match instr{
            Instr::Add(y) => {
                x += y;
            },
            _ => {}
        };

    }
    println!("P1:       {}", sum);
}

fn draw_pixel(x: i32, p: i32){

    let h = p % 40;
    let covered: bool = (x - 1) == h || x == h || (x + 1) == h;
    let pixel = match covered {
        true => "#",
        _ => "."
    };
    print!("{}",pixel);
    if h == 39 {
        println!("");
    }
}

fn parse_instr(line: &str) -> Vec<Instr>{
    return match line {
        "noop" => vec![Instr::Wait],
        _ => {
            let x = line.split_whitespace().collect::<Vec<_>>()[1].parse::<i32>().unwrap();
            vec![Instr::Wait, Instr::Add(x)]
        }
    };
}

#[derive(Debug)]
enum Instr{
    Wait,
    Add(i32)
}
