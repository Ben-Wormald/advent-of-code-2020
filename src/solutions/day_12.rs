#[derive(Debug)]
enum Action {
    N(isize),
    E(isize),
    S(isize),
    W(isize),
    L(usize),
    R(usize),
    F(isize),
}

#[derive(Clone, Copy, Debug)]
struct Pos {
    ns: isize,
    ew: isize,
}
impl Pos {
    fn add(&self, other: Pos) -> Pos {
        Pos {
            ns: self.ns + other.ns,
            ew: self.ew + other.ew,
        }
    }
    fn times(&self, multiplyer: isize) -> Pos {
        Pos {
            ns: self.ns * multiplyer,
            ew: self.ew * multiplyer,
        }
    }
}

#[derive(Clone, Copy, PartialEq, Debug)]
enum Bearing {
    North,
    East,
    South,
    West,
}

pub fn solve_part_one(input: &str) -> isize {
    let actions = process(input);

    let mut pos = Pos { ns: 0, ew: 0 };
    let mut bearing = Bearing::East;

    for action in actions {
        match action {
            Action::N(dn) => pos.ns += dn,
            Action::E(de) => pos.ew += de,
            Action::S(ds) => pos.ns -= ds,
            Action::W(dw) => pos.ew -= dw,
            Action::L(_) | Action::R(_) => bearing = rotate(bearing, action),
            Action::F(d) => match bearing {
                Bearing::North => pos.ns += d,
                Bearing::East => pos.ew += d,
                Bearing::South => pos.ns -= d,
                Bearing::West => pos.ew -= d,
            }
        }
    }
    pos.ns.abs() + pos.ew.abs()
}

fn process(input: &str) -> Vec<Action> {
    input
        .lines()
        .map(|line| match &line[..1] {
            "N" => Action::N(line[1..].parse().unwrap()),
            "E" => Action::E(line[1..].parse().unwrap()),
            "S" => Action::S(line[1..].parse().unwrap()),
            "W" => Action::W(line[1..].parse().unwrap()),
            "L" => Action::L(line[1..].parse().unwrap()),
            "R" => Action::R(line[1..].parse().unwrap()),
            "F" => Action::F(line[1..].parse().unwrap()),
            _ => panic!("unrecognised input"),
        })
        .collect()
}

fn rotate(bearing: Bearing, rotation: Action) -> Bearing {
    let mut bearings = [Bearing::North, Bearing::East, Bearing::South, Bearing::West];
    let angle = match rotation {
        Action::L(a) => {
            bearings.reverse();
            a
        }
        Action::R(a) => a,
        _ => panic!("unrecognised action"),
    };
    let current_idx = bearings.iter().position(|&b| b == bearing).unwrap();
    let new_idx = (current_idx + (angle / 90)) % 4;
    bearings[new_idx]
}

pub fn solve(input: &str) -> isize {
    let actions = process(input);

    let mut pos = Pos { ns: 0, ew: 0 };
    let mut waypoint = Pos { ns: 1, ew: 10 };

    for action in actions {
        match action {
            Action::N(dn) => waypoint.ns += dn,
            Action::E(de) => waypoint.ew += de,
            Action::S(ds) => waypoint.ns -= ds,
            Action::W(dw) => waypoint.ew -= dw,
            Action::L(_) | Action::R(_) => waypoint = rotate_waypoint(waypoint, action),
            Action::F(d) => pos = pos.add(waypoint.times(d)),
        }
    }
    pos.ns.abs() + pos.ew.abs()
}

fn rotate_waypoint(waypoint: Pos, rotation: Action) -> Pos {
    match rotation {
        Action::L(90) | Action::R(270) => Pos {
            ns: waypoint.ew,
            ew: -waypoint.ns,
        },
        Action::L(180) | Action::R(180) => Pos {
            ns: -waypoint.ns,
            ew: -waypoint.ew,
        },
        Action::L(270) | Action::R(90) => Pos {
            ns: -waypoint.ew,
            ew: waypoint.ns,
        },
        _ => panic!("unrecognised action"),
    }
}
