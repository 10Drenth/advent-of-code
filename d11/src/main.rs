use std::collections::VecDeque;

fn main() {
    println!("p1:      {}", calc_max_monkey_bussiness(20));
    println!("p2:      {}", calc_max_monkey_bussiness(10000));
}

fn calc_max_monkey_bussiness(n: usize) -> u64 {
    let blocks = include_str!("../input.txt")
        .split("\n\n").collect::<Vec<_>>();
    let mut monkeys: Vec<Monkey> = blocks.clone().into_iter().map(parse_monkey).collect();
    let mut items: Vec<VecDeque<u64>> = blocks.into_iter().map(parse_starting_items).collect();

    let item_cap: u64 = monkeys.iter().map(|m| m.test).product::<u64>();

    for _ in 0..n {
        for i in 0..monkeys.len().clone() {
            while let Some(item) = items[i].pop_front(){
                monkeys[i].bussiness += 1;
                let (new_item, target_monkey) = monkeys[i].perform_bussiness(item % item_cap);
                items[target_monkey].push_back(new_item);
            }
        }
    }
    let mut bussiness: Vec<u64> = monkeys.iter()
        .map(|monkey| monkey.bussiness).collect();
    bussiness.sort_by(|a, b| a.partial_cmp(b).unwrap());
    bussiness.iter().rev().take(2).product::<u64>()
}

fn parse_monkey(input: &str) -> Monkey{
    let lines: Vec<Vec<&str>> = input.lines()
        .map(|l| l.split_whitespace().collect::<Vec<&str>>())
        .collect();
    Monkey{
        op_term: parse_op_term(&lines[2]),
        operant: parse_operant(&lines[2]),
        test: parse_test(&lines[3]),
        true_monkey: parse_buddy(&lines[4]),
        false_monkey: parse_buddy(&lines[5]),
        bussiness: 0
    }
}

fn parse_starting_items(block: &str) -> VecDeque<u64> {
    block.lines().collect::<Vec<_>>()[1].split_whitespace().skip(2)
        .map(|s| s.chars().filter(|c| *c != ',').collect::<String>().parse::<u64>().unwrap())
        .collect()
}

fn parse_op_term(words: &Vec<&str>) -> Option<u64> {
    match words[5].parse::<u64>() {
        Ok(n) => Some(n),
        _ => None
    }
}

fn parse_operant(words: &Vec<&str>) -> fn(u64, u64) -> u64 {
    match words[4] {
        "+" => |x, y| x + y,
        _ => |x, y| x * y
    }
}

fn parse_test(words: &Vec<&str>) -> u64 {
    words[3].parse::<u64>().unwrap()
}

fn parse_buddy(words: &Vec<&str>) -> usize{
    words[5].parse::<usize>().unwrap()
}

#[derive(Debug)]
struct Monkey {
    op_term: Option<u64>,
    operant: fn(u64, u64) -> u64,
    test: u64,
    true_monkey: usize,
    false_monkey: usize,
    bussiness: u64
}

impl Monkey{
    fn perform_bussiness(&self, item: u64) -> (u64, usize){
        let x = self.op_term.unwrap_or_else(|| item);
        let item = (self.operant)(item, x);
        let target_monkey = if item % self.test == 0 { 
            self.true_monkey
        } else {
            self.false_monkey
        };
        (item, target_monkey)
    }
}
