fn main() {
    println!("{}", include_str!("../input.txt")
             .lines().into_iter()
             .map(|x| read_line(x))
             .fold((0, 0), |(s, m), e| {
                match e {
                    Some(x) => (s + x, m),
                    None => (0, std::cmp::max(m, s))
                }}).1);
}

fn read_line(line: &str) -> Option<i32>{
    match line.parse::<i32>(){
        Ok(n) => Some(n),
        _ => None
    }
}
