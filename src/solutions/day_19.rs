use std::collections::HashMap;

#[derive(Debug)]
enum Rule {
    Terminal(char),
    Nonterminal(Vec<Vec<usize>>),
}

pub fn solve(input: &str) -> usize {
    let (rules, words) = process(input);

    let initial_rule = match rules.get(&0).unwrap() {
        Rule::Nonterminal(options) => &options[0],
        Rule::Terminal(_) => panic!("hmm"),
    };

    words
        .iter()
        .fold(0, |sum, word| match validate_word(word, &initial_rule, &rules) {
            Ok(consumed) => if consumed == word.len() { sum + 1 } else { sum },
            Err(()) => sum,
        })
}

fn process(input: &str) -> (HashMap<usize, Rule>, Vec<&str>) {
    let parts: Vec<&str> = input.split("\n\n").collect();

    let mut rules: HashMap<usize, Rule> = HashMap::new();

    parts[0]
        .lines()
        .for_each(|line| {
            let line_parts: Vec<&str> = line.split(": ").collect();
            let rule_id = line_parts[0].parse().unwrap();

            let rule = if line_parts[1].starts_with("\"") {
                Rule::Terminal(
                    line_parts[1].replace("\"", "").chars().next().unwrap()
                )
            }
            else {
                let rule_options = line_parts[1]
                    .split(" | ")
                    .map(|rule_option| rule_option
                        .split_whitespace()
                        .map(|rule_part| rule_part.parse().unwrap())
                        .collect()
                    )
                    .collect();

                Rule::Nonterminal(rule_options)
            };

            rules.insert(rule_id, rule);
        });

    let words = parts[1].lines().collect();

    (rules, words)
}

fn validate_word(word: &str, rule_ids: &[usize], rules: &HashMap<usize, Rule>) -> Result<usize, ()> {
    let mut consumed = 0;

    for i in 0..rule_ids.len() {
        let rule = rules.get(&rule_ids[i]).unwrap();
        let remaining = &word[consumed..];
        
        match rule {
            Rule::Terminal(t) => {
                if remaining.chars().next().unwrap() == *t {
                    consumed += 1;
                }
                else {
                    return Err(());
                }
            },
            Rule::Nonterminal(options) => {
                let mut options_valid = false;
                let mut options_consumed = 0;

                for option in options {
                    match validate_word(remaining, option, rules) {
                        Ok(option_consumed) => {
                            options_valid = true;
                            options_consumed = option_consumed;
                            break;
                        },
                        Err(()) => (),
                    }
                }

                if options_valid {
                    consumed += options_consumed;
                }
                else {
                    return Err(());
                }
            }
        }
    }

    Ok(consumed)
}
