use std::cmp;

fn main() {
    let g: Grid = Grid::new(include_str!("../input.txt"));
    let mut ts: Vec<Vec<u64>> = vec![vec![u64::MAX; g.size.1]; g.size.0];
    let mut vs: Vec<Vec<bool>> = vec![vec![false; g.size.1]; g.size.0];
    ts[g.start.0][g.start.1] = 0;
    loop {
        if let Some((tent, next)) = g.positions().into_iter()
            .filter(|(x, y)| !vs[*x][*y])
            .map(|(x, y)| (ts[x][y], (x, y)))
            .min_by(|(v1, _), (v2, _)| v1.cmp(v2)){
            if tent == u64::MAX {
                break;
            }
            for (nx, ny) in g.neighbours(&next).iter().filter(|(x, y)| !vs[*x][*y]){
                let prev = ts[*nx][*ny];
                ts[*nx][*ny] = cmp::min(prev, tent + 1);
            }
            vs[next.0][next.1] = true;
        } else {
            break;
        }
    }
    println!("p1:       {}", ts[g.goal.0][g.goal.1]);
    println!("p2        {}", g.positions().into_iter()
             .filter(|(x, y)| g.heights[*x][*y] == 0)
             .map(|(x, y)| ts[x][y])
             .min().unwrap());
}

#[derive(Debug)]
struct Grid {
    size: (usize, usize),
    start: (usize, usize),
    goal: (usize, usize),
    heights: Vec<Vec<u32>>
}

impl Grid {
    fn new(input: &str) -> Grid {
        let size: (usize, usize) = (input.chars().take_while(|c| !c.is_whitespace()).count(),
                                    input.lines().count());
        let mut start: (usize, usize) = (0, 0);
        let mut goal: (usize, usize) = (0, 0);
        let chars: Vec<Vec<char>> = input.lines().map(|l| l.chars().collect()).collect();
        let mut heights: Vec<Vec<u32>> = vec![vec![0; size.1]; size.0];
        for x in 0..(size.0){
            for y in 0..(size.1){
                let c: char = chars[y][x];
                heights[x][y] = match c {
                    'S' => {goal = (x, y); 'a' as u32},
                    'E' => {start = (x, y); 'z' as u32},
                    x => x as u32
                } - 97;
            }
        }
        Grid{
            size,
            start,
            goal,
            heights
        }
    }

    fn neighbours(&self, pos: &(usize, usize)) -> Vec<(usize, usize)>{
        let offsets: Vec<(i32, i32)> = vec![(0, 1), (0, -1), (1, 0), (-1, 0)];
        let (px, py): (i32, i32) = (pos.0 as i32, pos.1 as i32);
        offsets.into_iter()
            .map(|(ox, oy)| self.safe_int_index(ox + px, oy + py))
            .flatten()
            .filter(|(x, y)| self.heights[*x][*y] + 1 >= self.heights[pos.0][pos.1])
            .collect()
    }
    
    fn safe_int_index(&self, x: i32, y: i32) -> Option<(usize, usize)>{
        if let Ok (x) = usize::try_from(x){
            if let Ok (y) = usize::try_from(y){
                if x < self.size.0 && y < self.size.1{
                    return Some((x, y));
                }
            }
        }
        None
    }

    fn positions(&self) -> Vec<(usize, usize)>{
        (0..(self.size.0))
            .flat_map(|x| (0..(self.size.1)).map(move |y| (x, y)))
            .collect()
    }
}
