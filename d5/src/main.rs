fn main() {
    let input = include_str!("../input.txt").split("\n\n").collect::<Vec<_>>();
    let mut stacks: Vec<Vec<char>> = parse_init_stack(input[0]);
    crate_mover9000(input[1], &mut stacks.clone());
    println!("");
    crate_mover9001(input[1], &mut stacks);
}

fn parse_init_stack(input: &str) -> Vec<Vec<char>> {
    let lines = input.lines().rev().collect::<Vec<&str>>();
    let mut stacks = vec![Vec::<char>::new(); lines[0].split_whitespace().count()];
    for l in 1..lines.len(){
        for i in 0..stacks.len(){
            let value: char = lines[l]
                .chars().collect::<Vec<char>>()
                .chunks(4).collect::<Vec<_>>()[i][1];
            if value != ' '{
                stacks[i].push(value);
            }
        }
    }
    stacks
}

fn crate_mover9000(input: &str, stacks: &mut Vec<Vec<char>>){
    let instructions = input.lines().map(parse_instruction);
    for instr in instructions{
        for _ in 0..instr.amount{
            if let Some(val) = stacks[instr.from].pop(){
                stacks[instr.to].push(val);
            }
        }
    }
    for stack in stacks.iter_mut(){
        if let Some(val) = stack.pop(){
            print!("{}", val);
        }
    }

}

fn crate_mover9001(input: &str, stacks: &mut Vec<Vec<char>>){
    let instructions = input.lines().map(parse_instruction);
    let mut arm = Vec::<char>::new();
    for instr in instructions{
        for _ in 0..instr.amount{
            if let Some(val) = stacks[instr.from].pop(){
                arm.push(val);
            }
        }
        for _ in 0..instr.amount{
            if let Some(val) = arm.pop(){
                stacks[instr.to].push(val);
            }
        }
    }
    for stack in stacks.iter_mut(){
        if let Some(val) = stack.pop(){
            print!("{}", val);
        }
    }
}

fn parse_instruction(input: &str) -> Instruction{
    let args = input.split_whitespace().flat_map(|s| s.parse::<usize>()).collect::<Vec<_>>();
    Instruction{
        amount: args[0],
        from: args[1] - 1,
        to: args[2] - 1
    }
}

#[derive(Debug)]
struct Instruction{
    amount: usize,
    from: usize,
    to: usize
}
