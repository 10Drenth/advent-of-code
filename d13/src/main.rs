use std::cmp::Ordering;

fn main() {
    let p1: usize = include_str!("../input.txt")
        .split("\n\n")
        .map(|p| p.split_once('\n').unwrap())
        .enumerate()
        .filter(|(_,(p1, p2))| validate(*p1, *p2))
        .map(|(i, _)| i + 1)
        .sum();
    println!("p1:        {}", p1);

    let m1 = Token::List(vec![Token::List(vec![Token::Num(6)])]);
    let m2 = Token::List(vec![Token::List(vec![Token::Num(2)])]);

    println!("p2:        {}", (ordered_pos(&m1)+1) * ordered_pos(&m2));
}

fn ordered_pos(token: &Token) -> usize{
    include_str!("../input.txt")
        .split_whitespace()
        .filter(|l| parse_list(*l).0.compare(token) == Ordering::Less)
        .count() + 1
}

fn validate(left: &str, right: &str) -> bool{
    let x = parse_list(left).0.compare(&parse_list(right).0) == Ordering::Less;
    x
}

fn parse_num(input: &str) -> (Token, String){
    (Token::Num(input.chars()
                .take_while(|c| c.is_digit(10))
                .collect::<String>()
                .parse::<u32>()
                .unwrap())
     , input.chars().skip_while(|c| c.is_digit(10)).collect())
}

fn parse_list(input: &str) -> (Token, String){
    let mut cs = input.chars();
    cs.next();
    let mut input: String = cs.collect();
    let mut tokens: Vec<Token> = Vec::new();
    loop{
        if let Some((t, rem)) = if input.starts_with(|c:char| c.is_digit(10)){
            Some(parse_num(input.as_str()))
        }else if input.starts_with("[") {
            Some(parse_list(input.as_str()))
        }else{
            let mut chars = input.as_str().chars();
            match chars.next() {
                Some(']') => {
                    return (Token::List(tokens), chars.collect());
                },
                _ =>{
                    input = chars.collect();
                }
            };
            None
        }{
            input = rem;
            tokens.push(t);
        }
    }
}

#[derive(Debug, Clone)]
enum Token {
    Num(u32),
    List(Vec<Token>)
}

impl Token{
    fn compare(&self, other: &Token) -> Ordering{
        match (self, other){
            (Token::Num(x), Token::Num(y)) => x.cmp(&y),
            (Token::List(xs), Token::List(ys)) =>{
                for (i, token) in xs.iter().enumerate(){
                    if let Some(other) = ys.get(i){
                        match token.compare(other){
                            Ordering::Equal => {},
                            ord => {return ord;}
                        };
                    } else {
                        break;
                    }
                }
                xs.len().cmp(&ys.len())
            },
            (xs@Token::List(_), y@Token::Num(_)) => xs.compare(&Token::List(vec![y.clone()])),
            (x@Token::Num(_), ys@Token::List(_)) => Token::List(vec![x.clone()]).compare(ys)
        }
    }
}
