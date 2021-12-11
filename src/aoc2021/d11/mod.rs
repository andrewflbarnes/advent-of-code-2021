use crate::utils;

pub fn solve(input1: String, _: String, _: &[String]) {
    let lines = utils::read_file_lines(&input1);

    let mut grid = lines.iter()
        .map(|l| l.chars().map(|i| i.to_string().parse::<u8>().unwrap()).collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let mut flashed = 0;
    for _ in 0..100 {
        flashed += tick(&mut grid).len();
    }
    println!("{} flashes", flashed);
    print_grid(&grid);
}

fn tick(grid: &mut Vec<Vec<u8>>) -> Vec<(usize, usize)> {
    grid.iter_mut().for_each(|row| row.iter_mut().for_each(|cell| *cell += 1));

    let mut flashed = vec![];

    loop {
        let mut flashed_count = 0;
        
        for i in 0..grid.len() {
            for j in 0..grid[i].len() {
                let pos = (i, j);
                let cell = grid[i][j];
                if cell > 9 && !flashed.contains(&pos) {
                    flashed.push(pos);
                    flashed_count += 1;
                    propagate_flash(grid, i, j);
                }
            }
        }

        if flashed_count == 0 {
            grid.iter_mut().for_each(|row| row.iter_mut().for_each(|cell| {
                if *cell > 9 {
                    *cell = 0;
                }
            }));
            return flashed;
        }
    }
}

fn propagate_flash(grid: &mut Vec<Vec<u8>>, row: usize, col: usize) {
    let rowbound = if row == 0 {0} else {row-1};
    let colbound = if col == 0 {0} else {col-1};
    for i in rowbound..=(row+1) {
        for j in colbound..=(col+1) {
            if i != row || j != col {
                match grid.get_mut(i).and_then(|r| r.get_mut(j)) {
                    Some(cell) => *cell +=1,
                    None => {},
                }
            }
        }
    }
}

fn print_grid(grid: &Vec<Vec<u8>>) {
    println!("");
    for row in grid.iter() {
        row.iter().for_each(|c| print!("{}", c));
        println!("");
    }
}