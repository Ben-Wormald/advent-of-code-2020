#[derive(Clone, Copy, PartialEq)]
enum Cell {
    Floor,
    Empty,
    Occupied,
}

pub fn solve(input: &str) -> usize {
    let mut grid = process(input);
    let mut is_stable = false;

    while !is_stable {
        let mut grid_new = grid.clone();
        is_stable = true;

        for y in 0..grid.len() {
            for x in 0.. grid[y].len() {
                grid_new[y][x] = new_state(&grid, x, y);

                if grid_new[y][x] != grid[y][x] {
                    is_stable = false;
                }
            }
        }
        grid = grid_new;
    }
    
    count_occupied(&grid)
}

fn process(input: &str) -> Vec<Vec<Cell>> {
    input
        .lines()
        .map(|line| line
            .chars()
            .map(|c| match c {
                '.' => Cell::Floor,
                'L' => Cell::Empty,
                '#' => Cell::Occupied,
                _ => panic!("unrecognised char in input"),
            })
            .collect()
        )
        .collect()
}

fn new_state(grid: &Vec<Vec<Cell>>, x: usize, y: usize) -> Cell {
    let x_min = if x == 0 { 0 } else { x - 1 };
    let x_max = if x + 1 == grid[0].len() { x + 1 } else { x + 2 };
    let y_min = if y == 0 { 0 } else { y - 1 };
    let y_max = if y + 1 == grid.len() { y + 1 } else { y + 2 };

    let mut occupied = 0;
    for i in y_min..y_max {
        for j in x_min..x_max {
            if !(i == y && j == x) && grid[i][j] == Cell::Occupied {
                occupied += 1;
            }
        }
    }

    match grid[y][x] {
        Cell::Empty => if occupied == 0 { Cell::Occupied } else { grid[y][x] },
        Cell::Occupied => if occupied >= 4 { Cell::Empty } else { grid[y][x] },
        _ => grid[y][x],
    }
}

fn count_occupied(grid: &Vec<Vec<Cell>>) -> usize {
    grid
        .into_iter()
        .fold(0, |sum, row| {
            sum + row
                .into_iter()
                .fold(0, |row_sum, cell| {
                    match cell {
                        Cell::Occupied => row_sum + 1,
                        _ => row_sum,
                    }
                })
        })
}
