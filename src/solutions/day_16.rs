#[derive(Clone, Debug)]
struct Field {
    name: String,
    range_a: Range,
    range_b: Range,
}

#[derive(Clone, Debug)]
struct Range {
    lower: usize,
    upper: usize,
}

pub fn solve_part_one(input: &str) -> usize {
    let (fields, _, tickets) = process(input);
    
    tickets
        .iter()
        .fold(0, |error_sum, ticket| {
            error_sum + get_error(&fields, ticket)
        })
}

fn process(input: &str) -> (Vec<Field>, Vec<usize>, Vec<Vec<usize>>) {
    let parts: Vec<&str> = input.split("\n\n").collect();

    let fields = parts[0].lines().map(|field| {
        let field_parts: Vec<&str> = field.split(": ").collect();
        let ranges_parts: Vec<&str> = field_parts[1].split(" or ").collect();
        let range_a_parts: Vec<&str> = ranges_parts[0].split("-").collect();
        let range_b_parts: Vec<&str> = ranges_parts[1].split("-").collect();

        Field {
            name: field_parts[0].to_string(),
            range_a: Range {
                lower: range_a_parts[0].parse().unwrap(),
                upper: range_a_parts[1].parse().unwrap(),
            },
            range_b: Range {
                lower: range_b_parts[0].parse().unwrap(),
                upper: range_b_parts[1].parse().unwrap(),
            },
        }
    }).collect();

    let my_ticket: Vec<&str> = parts[1].lines().collect();
    let my_ticket = my_ticket[1]
        .split(",")
        .map(|n| n.parse().unwrap())
        .collect();

    let tickets: Vec<&str> = parts[2].lines().collect();
    let tickets = tickets[1..]
        .iter()
        .map(|ticket| ticket
            .split(",")
            .map(|n| n.parse().unwrap())
            .collect()
        )
        .collect();

    (fields, my_ticket, tickets)
}

fn get_error(fields: &Vec<Field>, ticket: &Vec<usize>) -> usize {
    ticket
        .iter()
        .fold(0, |error_sum, &value| {
            let valid = fields
                .iter()
                .fold(false, |valid, field| {
                    match validate(field, value) {
                        true => true,
                        false => valid,
                    }
                });

            match valid {
                true => error_sum,
                false => error_sum + value,
            }
        })
}

fn validate(field: &Field, value: usize) -> bool {
    let in_range_a = value >= field.range_a.lower && value <= field.range_a.upper;
    let in_range_b = value >= field.range_b.lower && value <= field.range_b.upper;

    in_range_a || in_range_b
}

pub fn solve(input: &str) -> usize {
    let (fields, my_ticket, tickets) = process(input);
    let mut valid_tickets = filter_valid(&fields, &tickets);
    valid_tickets.push(my_ticket.clone());

    let field_values = get_field_values(&valid_tickets);
    let valid_fields_for_values: Vec<Vec<String>> = field_values
        .iter()
        .map(|values| get_valid_fields(&fields, &values))
        .collect();

    let mut set_fields = 0;
    let mut field_names: Vec<Option<String>> = vec![None; fields.len()];

    while set_fields < fields.len() {
        valid_fields_for_values
            .iter()
            .enumerate()
            .for_each(|(i, valid_fields)| {
                let unset_valid_fields: Vec<String> = valid_fields
                    .iter()
                    .cloned()
                    .filter(|field| !field_names.contains(&Some(field.clone())))
                    .collect();

                if unset_valid_fields.len() == 1 {
                    field_names[i] = Some(unset_valid_fields[0].clone());
                    set_fields += 1;
                }
            })
    }

    let field_names = field_names.iter().map(|field_name| field_name.clone().unwrap()).collect();

    get_departure_values(&my_ticket, &field_names)
        .iter()
        .fold(1, |product, value| product * value)
}

fn filter_valid(fields: &Vec<Field>, tickets: &Vec<Vec<usize>>) -> Vec<Vec<usize>> {
    tickets
        .iter()
        .cloned()
        .filter(|ticket| {
            match get_error(fields, ticket) {
                0 => true,
                _ => false,
            }
        })
        .collect()
}

fn get_field_values(tickets: &Vec<Vec<usize>>) -> Vec<Vec<usize>> {
    let mut field_values = vec![Vec::new(); tickets[0].len()];
    tickets
        .iter()
        .for_each(|ticket| ticket
            .iter()
            .enumerate()
            .for_each(|(i, &value)| field_values[i].push(value))
        );

    field_values
}

fn get_valid_fields(fields: &Vec<Field>, values: &Vec<usize>) -> Vec<String> {
    fields
        .iter()
        .cloned()
        .filter(|field| {
            values
                .iter()
                .fold(true, |valid, &value| {
                    match validate(field, value) {
                        true => valid,
                        false => false,
                    }
                })
        })
        .map(|field| field.name)
        .collect()
}

fn get_departure_values(ticket: &Vec<usize>, field_names: &Vec<String>) -> Vec<usize> {
    ticket
        .iter()
        .cloned()
        .enumerate()
        .filter(|(i, _)| {
            let field_name = &field_names[*i];
            field_name.len() > 8 && &field_name[..9] == "departure"
        })
        .map(|(_, value)| value)
        .collect()
}
