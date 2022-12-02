fn main() {
    println!("{}", include_str!("../input.txt")
             .lines().into_iter()
             .map(line_to_round)
             .map(round_to_score).sum::<i32>());
}

fn round_to_score((opponent, outcome_intent): (Move, MatchResult)) -> i32{
    match outcome_intent {
        MatchResult::Lose => 0 + opponent.previous(),
        MatchResult::Draw => 3 + opponent.index,
        _                 => 6 + opponent.next()
    }
}

fn line_to_round(line: &str) -> (Move, MatchResult){
    let mut x = line.split(' ');
    (read_move(x.next().unwrap()), read_intent(x.next().unwrap()))
}

fn read_move(c: &str) -> Move{
    match c {
        "A" => Move{index : 1},
        "B" => Move{index : 2},
        _   => Move{index : 3}
    }
}

fn read_intent(c: &str) -> MatchResult{
    match c{
        "X" => MatchResult::Lose,
        "Y" => MatchResult::Draw,
        _   => MatchResult::Win
    }
}

struct Move{
    index: i32
}

impl Move{
    fn next(&self) -> i32{
        match self.index {
            n@1..=2 => n + 1,
            _       => 1
        }
    }

    fn previous(&self) -> i32{
        match self.index {
            n@2..=3 => n - 1,
            _       => 3
        }
    }
}

enum MatchResult{
    Lose,
    Draw,
    Win
}
