#[derive(Clone, Debug)]
enum Op {
    Nop(isize),
    Acc(isize),
    Jmp(isize),
}

#[derive(Clone, Debug)]
struct Instruction {
    op: Op,
    visited: bool,
}

fn process_input(input: &str) -> Vec<Instruction> {
    input
        .lines()
        .map(|line| {
            let parts: Vec<&str> = line.split_whitespace().collect();

            let value = parts[1].parse().unwrap();
            let op = match parts[0] {
                "acc" => Op::Acc(value),
                "jmp" => Op::Jmp(value),
                _ => Op::Nop(value),
            };

            Instruction {
                op,
                visited: false,
            }
        })
        .collect()
}

pub fn solve_part_one(input: &str) -> isize {
    let mut instructions = process_input(input);

    let mut acc: isize = 0;
    let mut ptr = 0;

    loop {
        let instruction = &mut instructions[ptr];
        if instruction.visited {
            break;
        }
        instruction.visited = true;
        
        match instruction.op {
            Op::Nop(_) => ptr += 1,
            Op::Acc(value) => {
                acc += value;
                ptr += 1;
            },
            Op::Jmp(value) => ptr = ((ptr as isize) + value) as usize,
        }
    }

    acc
}

pub fn solve(input: &str) -> isize {
    let instructions = process_input(input);
    run(0, 0, instructions.clone(), false).unwrap()
}

fn run(ptr: usize, acc: isize, mut instructions: Vec<Instruction>, fixed: bool) -> Option<isize> {
    if ptr >= instructions.len() {
        return Some(acc)
    }

    if instructions[ptr].visited {
        return None
    }
    instructions[ptr].visited = true;
    
    match instructions[ptr].op {
        Op::Nop(value) => {
            match run(ptr + 1, acc, instructions.clone(), fixed) {
                Some(n) => Some(n),
                None => match fixed {
                    false => run(((ptr as isize) + value) as usize, acc, instructions.clone(), true),
                    true => None,
                }
            }
        },
        Op::Acc(value) => {
            run(ptr + 1, acc + value, instructions.clone(), fixed)
        },
        Op::Jmp(value) => {
            match run(((ptr as isize) + value) as usize, acc, instructions.clone(), fixed) {
                Some(n) => Some(n),
                None => match fixed {
                    false => run(ptr + 1, acc, instructions.clone(), true),
                    true => None,
                },
            }
        },
    }
}
