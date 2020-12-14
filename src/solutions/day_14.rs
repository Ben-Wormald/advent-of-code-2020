use std::collections::HashMap;

#[derive(Debug)]
enum Instruction {
    Mask(String),
    Mem(Mem),
}

#[derive(Debug)]
struct Mem {
    ptr: usize,
    value: String,
}

const BITS: usize = 36;

pub fn solve_part_one(input: &str) -> usize {
    let instructions = process(input);

    let mut memory = HashMap::new();
    let mut mask = String::new();

    for instruction in instructions {
        match instruction {
            Instruction::Mask(value) => mask = value,
            Instruction::Mem(mem) => {
                let masked_value = mask_value(&mem.value, &mask);
                memory.insert(mem.ptr, masked_value);
            }
        }
    }
    
    memory.iter().fold(0, |sum, (_, value)| {
        sum + to_decimal(value)
    })
}

fn process(input: &str) -> Vec<Instruction> {
    input
        .lines()
        .map(|line| {
            let parts: Vec<&str> = line.split(" = ").collect();
            match parts[0] {
                "mask" => Instruction::Mask(parts[1].to_string()),
                _ => {
                    let ptr = parts[0][4..(parts[0].len() - 1)].parse().unwrap();
                    let value_binary = to_binary(parts[1].parse().unwrap());
                    let value = add_leading_zeros(&value_binary);

                    Instruction::Mem(Mem {
                        ptr,
                        value,
                    })
                }
            }
        })
        .collect()
}

fn to_binary(decimal: usize) -> String {
    let mut remainder = decimal;
    let mut bit_val = 2usize.pow((BITS - 1) as u32);
    let mut binary = String::new();
    for _ in 0..BITS {
        if remainder >= bit_val {
            binary.push('1');
            remainder -= bit_val;
        }
        else {
            binary.push('0');
        }
        bit_val /= 2;
    }
    binary
}

fn add_leading_zeros(value: &str) -> String {
    let leading_zeros = vec!["0"; BITS - value.len()].join("");
    format!("{}{}", leading_zeros, value)
}

fn mask_value(value: &str, mask: &str) -> String {
    let mask_chars: Vec<char> = mask.chars().collect();
    let mut value_chars: Vec<char> = value.chars().collect();
    
    for i in 0..mask_chars.len() {
        if mask_chars[i] != 'X' {
            value_chars[i] = mask_chars[i];
        }
    }
    value_chars.iter().collect()
}

fn to_decimal(binary: &str) -> usize {
    let mut bit_val = 2usize.pow(BITS as u32);
    binary.chars().fold(0, |sum, bit| {
        bit_val /= 2;
        match bit {
            '0' => sum,
            '1' => sum + bit_val,
            _ => panic!("not a binary string"),
        }
    })
}

pub fn solve(input: &str) -> usize {
    let instructions = process(input);

    let mut memory = HashMap::new();
    let mut mask = String::new();

    for instruction in instructions {
        match instruction {
            Instruction::Mask(value) => mask = value,
            Instruction::Mem(mem) => {
                let ptrs = get_ptrs(mem.ptr, &mask);
                for ptr in ptrs {
                    memory.insert(ptr, mem.value.clone());
                }
            }
        }
    }
    
    memory.iter().fold(0, |sum, (_, value)| {
        sum + to_decimal(value)
    })
}

fn get_ptrs(ptr: usize, mask: &str) -> Vec<usize> {
    let ptr_chars: Vec<char> = to_binary(ptr).chars().collect();
    let mask_chars: Vec<char> = mask.chars().collect();

    let mut ptrs = vec![ptr_chars];

    for c in 0..mask_chars.len() {
        match mask_chars[c] {
            '1' => {
                for p in 0..ptrs.len() {
                    ptrs[p][c] = '1';
                }
            },
            'X' => {
                let mut ptrs_copy = ptrs.clone();
                for p in 0..ptrs.len() {
                    ptrs[p][c] = '0';
                    ptrs_copy[p][c] = '1';
                }
                ptrs.extend(ptrs_copy);
            },
            _ => ()
        }
    }
    
    ptrs
        .iter()
        .map(|ptr_chars| {
            let ptr_str: String = ptr_chars.iter().collect();
            to_decimal(&ptr_str)
        })
        .collect()
}
