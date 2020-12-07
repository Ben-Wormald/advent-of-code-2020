use std::collections::HashMap;
use std::collections::HashSet;

pub fn solve_part_one(input: &str) -> usize {
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

#[derive(Debug)]
struct Containee {
    name: String,
    count: usize,
}

pub fn solve(input: &str) -> usize {
    let mut containers = HashMap::new();
    input
        .lines()
        .for_each(|line| {
            let parts: Vec<&str> = line.split(" bags contain ").collect();
            let container = parts[0];

            let contains = parts[1];
            let mut containees: Vec<Containee> = Vec::new();
            if contains != "no other bags." {
                contains.split(", ").for_each(|containee| {
                    let containee_words: Vec<&str> = containee.split_whitespace().collect();
                    let name = format!("{} {}", containee_words[1], containee_words[2]);
                    let count: usize = containee_words[0].parse().expect("oh no");
                    containees.push(Containee {
                        name,
                        count,
                    });
                });
            }
            containers.insert(container, containees);
        });

    let initial_bag = "shiny gold";
    get_containees(initial_bag, &containers) - 1
}

fn get_containees(container: &str, containers: &HashMap<&str, Vec<Containee>>) -> usize {
    containers.get(container).expect("oh no").iter().fold(1, |sum, containee| {
        sum + containee.count * get_containees(&containee.name, &containers)
    })
}
