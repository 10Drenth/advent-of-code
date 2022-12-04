use std::collections::HashSet;

fn main() {
    let input = include_str!("../input.txt");

    println!("p1: {}", input.lines().into_iter()
        .map(read_line1).sum::<i32>());

    println!("p2: {}",input.lines().into_iter()
        .map(|l| l.chars().map(char_to_priority).collect::<HashSet<_>>())
        .collect::<Vec<_>>().chunks(3).map(assign_badge).sum::<i32>());
}

fn read_line1(line: &str) -> i32{
    let compl = line.len() / 2;
    let comp1 = line.chars().take(compl).collect::<HashSet<_>>();
    let comp2 = line.chars().skip(compl).collect::<HashSet<_>>();
    let intersection = comp1.intersection(&comp2).collect::<String>();
    intersection.chars().map(char_to_priority).into_iter().max().unwrap()
}

fn assign_badge(group: &[HashSet<i32>]) -> i32{
    let mut iter = group.iter();
    let (i1, i2, i3) = (iter.next().unwrap(), iter.next().unwrap(), iter.next().unwrap());
    let r1 = i1.intersection(&i2).map(|x| *x).collect::<HashSet<i32>>();
    *r1.intersection(&i3).into_iter().max().unwrap()
}

fn char_to_priority(c: char) -> i32{
    match c.is_uppercase(){
        false => (c as i32) - 96,
        _     => (c as i32) - 38
    }
}
