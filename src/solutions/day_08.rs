#[derive(Debug)]
enum Op {
    Nop(isize),
    Acc(isize),
    Jmp(isize),
}

#[derive(Debug)]
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

pub fn solve(input: &str) -> isize {
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
