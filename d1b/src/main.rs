fn main() {

    let calories = include_str!("../input.txt")
             .lines().into_iter()
             .map(|x| read_line(x));
    let mut top3 = [0,0,0];
    let mut acc = 0;
    for val in calories {
        match val {
            Some(n) => acc += n,
            None => {
                top3 = insert_elf(top3, acc);
                acc = 0;
            }
        }
    }
    println!("{}", top3.into_iter().sum::<i32>());
}

fn insert_elf(top3: [i32; 3], new_elf: i32) -> [i32; 3]{
    match top3 {
        [e1, e2, e3] =>{
            if new_elf > e3 {
                [e2, e3, new_elf]
            } else if new_elf > e2{
                [e2, new_elf, e3]
            } else if new_elf > e1{
                [new_elf, e2, e3]
            } else {
                [e1, e2, e3]
            }
        }
    }
}

fn read_line(line: &str) -> Option<i32>{
    match line.parse::<i32>(){
        Ok(n) => Some(n),
        _ => None
    }
}
