#[derive(Clone, PartialEq)]
enum Cube {
    Active,
    Inactive,
}

const CYCLES: usize = 6;

pub fn solve(input: &str) -> usize {
    let mut grid = process(input);
    
    for _ in 0..CYCLES {
        if cubes_at_edge(&grid) {
            grid = expand_grid(&grid);
        }
        grid = new_grid(&grid);
    }
    count_active(&grid)
}

fn process(input: &str) -> Vec<Vec<Vec<Cube>>> {
    input
        .lines()
        .map(|x| x
            .chars()
            .map(|y| match y {
                '#' => vec![Cube::Active],
                _ => vec![Cube::Inactive],
            })
            .collect()
        )
        .collect()
}

fn cubes_at_edge(grid: &Vec<Vec<Vec<Cube>>>) -> bool {
    let mut at_edge = false;
    for x in 0..grid.len() {
        let x_edge = x == 0 || x == grid.len() - 1;
        for y in 0..grid[x].len() {
            let y_edge = y == 0 || y == grid[x].len() - 1;
            for z in 0..grid[x][y].len() {
                let z_edge = z == 0 || z == grid[x][y].len() - 1;

                if (x_edge || y_edge || z_edge) && grid[x][y][z] == Cube::Active {
                    at_edge = true;
                }
            }
        }
    }
    at_edge
}

fn expand_grid(grid: &Vec<Vec<Vec<Cube>>>) -> Vec<Vec<Vec<Cube>>> {
    let mut new_grid: Vec<Vec<Vec<Cube>>> = grid
        .iter()
        .cloned()
        .map(|x| {
            let mut new_x: Vec<Vec<Cube>> = x
                .iter()
                .cloned()
                .map(|y| {
                    let mut new_y = y;
                    new_y.insert(0, Cube::Inactive);
                    new_y.push(Cube::Inactive);
                    new_y
                })
                .collect();
            new_x.insert(0, vec![Cube::Inactive; new_x[0].len()]);
            new_x.push(vec![Cube::Inactive; new_x[0].len()]);
            new_x
        })
        .collect();
    new_grid.insert(0, vec![vec![Cube::Inactive; new_grid[0][0].len()]; new_grid[0].len()]);
    new_grid.push(vec![vec![Cube::Inactive; new_grid[0][0].len()]; new_grid[0].len()]);
    new_grid
}

fn count_active(grid: &Vec<Vec<Vec<Cube>>>) -> usize {
    grid
        .iter()
        .fold(0, |x_sum, x| x_sum + x
            .iter()
            .fold(0, |y_sum, y| y_sum + y
                .iter()
                .fold(0, |z_sum, z| z_sum + match z {
                    Cube::Active => 1,
                    Cube::Inactive => 0,
                })
            )
        )
}

fn new_grid(grid: &Vec<Vec<Vec<Cube>>>) -> Vec<Vec<Vec<Cube>>> {
    grid
        .iter()
        .cloned()
        .enumerate()
        .map(|(i_x, x)| x
            .iter()
            .cloned()
            .enumerate()
            .map(|(i_y, y)| y
                .iter()
                .cloned()
                .enumerate()
                .map(|(i_z, _)| new_state(grid, i_x, i_y, i_z))
                .collect()
            )
            .collect()
        )
        .collect()
}

fn new_state(grid: &Vec<Vec<Vec<Cube>>>, x: usize, y: usize, z: usize) -> Cube {
    let x_min = if x == 0 { 0 } else { x - 1 };
    let x_max = if x + 1 == grid.len() { x + 1 } else { x + 2 };

    let y_min = if y == 0 { 0 } else { y - 1 };
    let y_max = if y + 1 == grid[0].len() { y + 1 } else { y + 2 };

    let z_min = if z == 0 { 0 } else { z - 1 };
    let z_max = if z + 1 == grid[0][0].len() { z + 1 } else { z + 2 };

    let mut active_neighbours = 0;
    for i in x_min..x_max {
        for j in y_min..y_max {
            for k in z_min..z_max {
                if !(i == x && j == y && k == z) && grid[i][j][k] == Cube::Active {
                    active_neighbours += 1;
                }
            }
        }
    }

    match grid[x][y][z] {
        Cube::Active => if active_neighbours < 2 || active_neighbours > 3 { Cube::Inactive } else { Cube::Active },
        Cube::Inactive => if active_neighbours == 3 { Cube::Active } else { Cube::Inactive },
    }
}

fn print_grid(grid: &Vec<Vec<Vec<Cube>>>) -> () {
    for x in grid {
        for y in x {
            for z in y {
                match z {
                    Cube::Active => print!("#"),
                    Cube::Inactive => print!("."),
                }
            }
            print!("\n");
        }
        print!("\n\n");
    }
}
