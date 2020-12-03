struct Delta {
    x: usize,
    y: usize,
}

const DELTAS: [Delta; 5] = [
    Delta { x: 1, y: 1 },
    Delta { x: 3, y: 1 },
    Delta { x: 5, y: 1 },
    Delta { x: 7, y: 1 },
    Delta { x: 1, y: 2 },
];

fn sum_trees(grid: &Vec<Vec<usize>>, d: &Delta) -> usize {
    let mut sum = 0;
    let mut x = 0;
    let mut y = 0;

    while y < grid.len() {
        sum += grid[y][x];
        x = (x + d.x) % grid[y].len();
        y += d.y;
    }
    sum
}

pub fn solve(input: &str) -> usize {
    let grid: Vec<Vec<usize>> = input
        .split('\n')
        .filter(|line| line.len() > 0)
        .map(|line| {
            line.chars().map(|character| match character {
                '#' => 1,
                _ => 0,
            }).collect()
        })
        .collect();

    DELTAS
        .iter()
        .fold(1, |total, d| total * sum_trees(&grid, &d))
}
