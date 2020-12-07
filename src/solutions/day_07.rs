use std::collections::HashMap;
use std::collections::HashSet;

pub fn solve(input: &str) -> usize {
    let mut containers = HashMap::new();
    input
        .lines()
        .for_each(|line| {
            let parts: Vec<&str> = line.split(" bags contain ").collect();
            let container = parts[0];

            let contains = parts[1];
            if contains != "no other bags." {
                contains.split(", ").for_each(|containee| {
                    let containee_words: Vec<&str> = containee.split_whitespace().collect();
                    let containee_name = format!("{} {}", containee_words[1], containee_words[2]);
                    
                    let containee_containers = containers.entry(containee_name).or_insert(HashSet::new());
                    (*containee_containers).insert(container);
                });
            }
        });

    let mut bag_containers: HashSet<String> = HashSet::new();
    let initial_bag = "shiny gold";
    bag_containers.insert(initial_bag.to_string());

    let mut done = false;
    while !done {
        let len_before = bag_containers.len();
        for bag_container in bag_containers.clone() {
            for (containee, containee_containers) in &containers {
                if *containee == bag_container {
                    for containee_container in containee_containers {
                        bag_containers.insert(containee_container.to_string());
                    }
                }
            }
        }
        if bag_containers.len() == len_before {
            done = true;
        }
    }

    bag_containers.remove(initial_bag);
    bag_containers.len()
}
