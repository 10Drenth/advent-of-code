fn main() {
    println!("p1:    {}",solve(4));
    println!("p2:    {}",solve(14));
}

fn solve(n: usize) -> usize{
    let input = include_str!("../input.txt").chars().collect::<Vec<char>>();
    for i in (n-1) .. input.len(){
        let slice = input.get((i-(n-1))..(i+1));
        if all_unique(slice.unwrap()) {
            return i + 1;
        }
    }
    0
}

fn all_unique(items: &[char]) -> bool{
    let mut copy = items.clone().into_iter().collect::<Vec<_>>();
    copy.sort();
    copy.dedup();
    copy.len() == items.len()
}
