#[derive(Clone, Copy, Debug)]
enum Symbol {
    Num(usize),
    Op(Op),
}

#[derive(Clone, Copy, Debug)]
enum Op {
    Add,
    Multiply,
}

pub fn solve(input: &str) -> usize {
    input
        .lines()
        .map(|line| {
            let line = line.replace("(", "( ").replace(")", " )");
            let expr: Vec<&str> = line.split_whitespace().collect();
            let result = process(&expr);
            match result {
                Symbol::Num(num) => num,
                _ => panic!("oh dear")
            }
        })
        .fold(0, |sum, result| sum + result)
}

fn process(symbols: &[&str]) -> Symbol {
    let mut bracket_depth = 0;
    let mut open_bracket = 0;

    let mut symbols_expanded = Vec::new();

    for i in 0..symbols.len() {
        match symbols[i] {
            "(" => {
                bracket_depth += 1;
                if bracket_depth == 1 {
                    open_bracket = i;
                }
            },
            ")" => {
                bracket_depth -= 1;
                if bracket_depth == 0 {
                    symbols_expanded.push(process(&symbols[(open_bracket + 1)..i]))
                }
            },
            symbol => if bracket_depth == 0 {
                match symbol {
                    "+" => symbols_expanded.push(Symbol::Op(Op::Add)),
                    "*" => symbols_expanded.push(Symbol::Op(Op::Multiply)),
                    num => symbols_expanded.push(Symbol::Num(num.parse().unwrap())),
                }
            },
        }
    }
    
    let mut value: Option<usize> = None;
    let mut operator: Option<Op> = None;

    for symbol in symbols_expanded {
        if value == None {
            match symbol {
                Symbol::Num(num) => value = Some(num),
                _ => panic!("first symbol not a number"),
            }
        }
        else {
            match symbol {
                Symbol::Op(op) => operator = Some(op),
                Symbol::Num(num) => value = match operator.expect("second number before operator") {
                    Op::Add => Some(value.unwrap() + num),
                    Op::Multiply => Some(value.unwrap() * num),
                }
            }
        }
    }

    Symbol::Num(value.unwrap())
}
