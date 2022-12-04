fn main() {
    let res = include_str!("../input.txt")
             .lines().into_iter()
             .map(process_line).collect::<Vec<_>>();
    println!("p1:   {}", res.clone().iter().map(|x|x.0).sum::<i32>());
    println!("p2:   {}", res.iter().map(|x|x.1).sum::<i32>());
}

fn process_line(line: &str) -> (i32, i32){
    let mut pair = line.split(",").map(|p| p.split("-").map(to_int));
    let item1 = pair.next();
    let item2 = pair.next();

    match (item1, item2) {
        (Some(mut r1), Some(mut r2)) => {
            let ra1 = r1.next();
            let ra2 = r1.next();
            let rb1 = r2.next();
            let rb2 = r2.next();
            match (ra1, ra2, rb1, rb2){
                (Some(x1), Some(x2), Some(y1), Some(y2)) =>{
                    ( check_contained(x1, x2, y1, y2)
                    , check_overlap(x1, x2, y1, y2))
                },
                _ =>{
                    eprintln!("one of the ranges in {} is invalid", line);
                    (0, 0)
                }
            }
        },
        _ => {
            eprintln!("found less than 2 ranges in {}", line);
            (0, 0)
        }
    }
}

fn to_int(s: &str) -> i32{
    match s.parse::<i32>(){
        Ok(n) => n,
        Err(e) => {
            eprintln!("{} {}",e, s);
            0
        }
    }
}

fn check_contained(xl: i32, xu: i32, yl: i32, yu: i32) -> i32{
    if xl <= yl && yu <= xu {
        return 1;
    }
    if yl <= xl && xu <= yu {
        return 1;
    }
    0
}

fn check_overlap(xl: i32, xu: i32, yl: i32, yu: i32) -> i32{
    if (xl <= yl && yl <= xu) || (xl <= yu && yu <= xu) {
        return 1;
    }
    if (yl <= xl && xl <= yu) || (yl <= xu && xu <= yu) {
        return 1;
    }
    0
}

