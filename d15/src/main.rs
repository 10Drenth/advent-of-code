fn main() {
    //create intersection-area per line
    let mut ranges: Vec<Range> = 
        include_str!("../input.txt")
        .lines()
        .flat_map(parse_ln).collect();
    ranges.sort();
    dbg!(ranges.clone());

    let mut max_r = ranges[0].r;
    let mut n: i64 = 0;
    for i in 1..ranges.len(){
        let a = ranges[i-1];
        let b = ranges[i];
        if a.r < b.l {
            n += max_r.min(b.l) - a.l;
        }else{
            n += b.l - a.l;
        }
        max_r = b.r.max(max_r);
        dbg!(n);
    }
    n += max_r - ranges.last().unwrap().l;
    dbg!(n);
}

fn parse_ln(input: &str) -> Option<Range>{
    let segs: Vec<&str> = input.split('=').skip(1).collect();
    let sx: i64 = segs[0].split_once(',').unwrap().0.parse().unwrap();
    let sy: i64 = segs[1].split_once(':').unwrap().0.parse().unwrap();
    let bx: i64 = segs[2].split_once(',').unwrap().0.parse().unwrap();
    let by: i64 = segs[3].parse().unwrap();

    let r = sx.abs_diff(bx) as i64 + sy.abs_diff(by) as i64;
    // let vdif = sy.abs_diff(2000000) as i64;
    let vdif = sy.abs_diff(10) as i64;
    let overlap = r - vdif;
    if overlap >= 0 {
        return Some(Range{
            l: sx - overlap,
            r: sx + overlap + 1
        });
    }// [l,r)
    None
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
struct Range{
    l: i64,
    r: i64
}
