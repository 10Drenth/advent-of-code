fn main() {
    println!("{}", include_str!("../input.txt")
             .lines().into_iter()
             .map(line_to_round)
             .map(round_to_score).sum::<i32>());
}

fn round_to_score((opponent, me): (i32, i32)) -> i32{
    let match_score = match (opponent, me){
        (3, 1) | (1, 2) | (2, 3) => 6,
        (1, 3) | (2, 1) | (3, 2) => 0,
        _                        => 3
    };
    match_score + me
}

fn line_to_round(line: &str) -> (i32, i32){
    let mut x = line.split(' ').map(char_to_move);
    (x.next().unwrap(), x.next().unwrap())
}

fn char_to_move(c: &str) -> i32{
    match c {
        "A" | "X" => 1,
        "B" | "Y" => 2,
        _         => 3
    }
}
