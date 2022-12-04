fn main() {
    println!("{}", include_str!("../input.txt")
             .lines().into_iter()
             .map(line_to_round)
             .map(round_to_score).sum::<i32>());
}

fn round_to_score((a, x): (i32, MatchResult)) -> i32{
    (match x {
        MatchResult::Lose => a - 1,
        MatchResult::Draw => a,
        _                 => a + 1
    }) % 3 + (x as i32 * 3)
}

fn line_to_round(line: &str) -> (i32, MatchResult){
    let mut x = line.split(' ');
    (read_move(x.next().unwrap()), read_intent(x.next().unwrap()))
}

fn read_move(c: &str) -> i32{
    match c {
        "A" => 1,
        "B" => 2,
        _   => 3
    }
}

fn read_intent(c: &str) -> MatchResult{
    match c{
        "X" => MatchResult::Lose,
        "Y" => MatchResult::Draw,
        _   => MatchResult::Win
    }
}

enum MatchResult{
    Lose,
    Draw,
    Win
}
