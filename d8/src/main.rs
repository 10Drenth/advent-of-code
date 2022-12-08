use std::cmp;

fn main() {
    let input = include_str!("../input.txt");
    let horizontals = input.lines()
        .map(|l| l.chars().map(|c| c.to_digit(10).unwrap()).collect::<Vec<_>>()).collect::<Vec<_>>();
    let verticals = transpose(&horizontals);
    let unobscured: usize = (0..horizontals.len())
        .map(|y| (0..verticals.len()).filter(|x| check_unobscured(*x, y, &horizontals, &verticals)).count()).sum();
    println!("p1:       {}", unobscured);
    
    let max_score: usize = (0..horizontals.len())
        .map(|y| (0..verticals.len()).map(|x| calc_score(x, y, &horizontals, &verticals)).max().unwrap()).max().unwrap();
    println!("p2:       {}", max_score);

}

fn transpose(v: &Vec<Vec<u32>>) -> Vec<Vec<u32>>{
    (0..v[0].len())
        .map(|i| v.iter().map(|v_inner| v_inner[i].clone()).collect::<Vec<u32>>())
        .collect::<Vec<Vec<u32>>>()
}

fn check_unobscured(x: usize, y: usize, h: &Vec<Vec<u32>>, v: &Vec<Vec<u32>>) -> bool {
    let val = h[y][x].clone();
    h[y].iter().take(x).all(|other| other < &val) 
        || h[y].iter().skip(x+1).all(|other| other < &val) 
        || v[x].iter().take(y).all(|other| other < &val) 
        || v[x].iter().skip(y+1).all(|other| other < &val)
}

fn calc_score(x: usize, y: usize, h: &Vec<Vec<u32>>, v: &Vec<Vec<u32>>) -> usize {
    let val = h[y][x].clone();
    dir_score(h[y].iter().take(x).rev().collect(), val) * dir_score(h[y].iter().skip(x+1).collect(), val) * dir_score(v[x].iter().take(y).rev().collect(), val) * dir_score(v[x].iter().skip(y+1).collect(), val)
}

fn dir_score(row: Vec<&u32>, val: u32) -> usize{
    cmp::min(row.len(), row.iter().take_while(|x| x < &&&val).count() + 1)
}
