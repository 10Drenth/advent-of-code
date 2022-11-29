fn main() {
    println!("{}", include_str!("../input.txt")
             .lines().into_iter()
             .map(|x| read_line(x))
             .fold((0, i32::MAX), |(c, prev), x| (c + ((x > prev) as i32),x)).0);
}

fn read_line(line: &str) -> i32{
    match line.parse::<i32>(){
        Ok(n) => n,
        Err(e) => {
            eprintln!("{} {}",e, line);
            i32::MIN
        }
    }
}
