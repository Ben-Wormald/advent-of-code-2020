enum Bus {
    InService(usize),
    OutOfService,
}

pub fn solve_part_one(input: &str) -> usize {
    let lines: Vec<&str> = input.lines().collect();
    let arrival: usize = lines[0].parse().unwrap();

    let buses: Vec<Bus> = lines[1]
        .split(",")
        .map(|bus| match bus {
            "x" => Bus::OutOfService,
            time => Bus::InService(time.parse().unwrap()),
        })
        .collect();
    
    let mut optimal_bus: Option<usize> = None;
    for bus in buses {
        match bus {
            Bus::InService(bus_time) => optimal_bus =
                if optimal_bus == None {
                    Some(bus_time)
                }
                else if wait_time(arrival, bus_time) < wait_time(arrival, optimal_bus.unwrap()) {
                    Some(bus_time)
                }
                else {
                    optimal_bus
                },
            Bus::OutOfService => (),
        }
    }

    optimal_bus.unwrap() * wait_time(arrival, optimal_bus.unwrap())
}

fn wait_time(arrival: usize, bus_time: usize) -> usize {
    bus_time - (arrival % bus_time)
}

#[derive(Clone, Copy, Debug)]
struct BusTime {
    time: u128,
    offset: u128,
}

pub fn solve(input: &str) -> u128 {
    let lines: Vec<&str> = input.lines().collect();

    let buses: Vec<BusTime> = lines[1]
        .split(",")
        .enumerate()
        .map(|(i, bus)| BusTime {
            time: match bus {
                "x" => 0,
                time => time.parse().unwrap(),
            },
            offset: i as u128,
        })
        .filter(|bus| match bus.time {
            0 => false,
            _ => true,
        })
        .collect();

    let mut t = 0;
    loop {
        let matches: Vec<&BusTime> = buses.iter().filter(|bus| (t + bus.offset) % bus.time == 0).collect();

        if matches.len() == buses.len() {
            break t
        }

        t += matches.iter().fold(1, |product, bus| product * bus.time);
    }
}
