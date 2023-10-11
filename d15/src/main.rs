use std::cmp;

fn main() {
    let input: Vec<Sensor> =
        include_str!("../input.txt")
        .lines()
        .map(parse_ln)
        .collect();
    part_one(&input);

    part_two(&input);

}

fn part_one(input: &Vec<Sensor>){
    //create intersection-area per line
    let ranges: Vec<Range> = input
        .iter()
        .flat_map(|&s| s.coverage_at(2000000))
        .collect();
    let flat_ranges = flatten_ranges(ranges);
    //dbg!(&flat_ranges);
    let n: i64 = flat_ranges.iter().map(|&r| r.get_size()).sum();
    dbg!(n);
}


fn part_two(input: &Vec<Sensor>){
    for y in 0..4000000 {
        let ranges: Vec<Range> = input
            .iter()
            .flat_map(|&s| s.coverage_at(y))
            .collect();
        let flat_ranges = flatten_ranges(ranges);
        if flat_ranges.len() > 1 {
            //dbg!(&flat_ranges);
            dbg!(flat_ranges[0].r * 4000000 + y);
        }
    }
}
fn parse_ln(input: &str) -> Sensor{
    let segs: Vec<&str> = input.split('=').skip(1).collect();
    let sx: i64 = segs[0].split_once(',').unwrap().0.parse().unwrap();
    let sy: i64 = segs[1].split_once(':').unwrap().0.parse().unwrap();
    let bx: i64 = segs[2].split_once(',').unwrap().0.parse().unwrap();
    let by: i64 = segs[3].parse().unwrap();

    Sensor { sensor_x: sx, sensor_y: sy, beacon_x: bx, beacon_y: by }
}

fn flatten_ranges(mut ranges: Vec<Range>) -> Vec<Range>{
    ranges.sort();
    //dbg!(ranges.clone());

    let mut flat_ranges: Vec<Range> = Vec::new();
    let mut flat_range: Range = ranges[0];
    for r in ranges {
        if r.l > flat_range.r {
            // new flat range
            flat_ranges.push(flat_range.to_owned());
            flat_range = r.to_owned();
            continue;
        }
        //Otherwise merge with current flat range.
        flat_range.r = cmp::max(r.r, flat_range.r);
    }
    flat_ranges.push(flat_range);
    flat_ranges
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Sensor{
    sensor_x: i64,
    sensor_y: i64,
    beacon_x: i64,
    beacon_y: i64
}

impl Sensor {
    fn coverage_at(&self, y: i64) -> Option<Range>{
        let r = self.sensor_x.abs_diff(self.beacon_x) as i64 
            + self.sensor_y.abs_diff(self.beacon_y) as i64;
        let vdif = self.sensor_y.abs_diff(y) as i64;
        let overlap = r - vdif;
        if overlap >= 0 {
            return Some(Range{
                l: self.sensor_x - overlap,
                r: self.sensor_x + overlap + 1
            });
        }// [l,r)
        None
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
struct Range{
    l: i64,
    r: i64
}

impl Range {
    fn get_size(&self) -> i64{
        self.r - self.l - 1
    }
}
