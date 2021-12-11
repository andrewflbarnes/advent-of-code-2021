use crate::utils;

pub fn solve(input1: String, _: String, _: &[String]) {
    let lines = utils::read_file_lines(&input1);

    let cave = lines.iter()
        .map(|l| l.chars()
            .map(|c| c.to_string().parse::<i32>().unwrap())
            .collect::<Vec<_>>())
        .collect::<Vec<_>>();

    // println!("{:?}", cave);

    let lps = find_low_points(&cave);
    // println!("Low points: {:?}", lps);

    let result = lps.iter()
        .map(|p| cave[p.0][p.1] + 1)
        .sum::<i32>();
    println!("Low point sum: {:?}", result);

    let mut basins = find_basins(&cave, &lps);
    // println!("{:?}", basins);

    basins.sort();

    let mut basin_sizes = basins.iter().map(|b| b.1.len()).collect::<Vec<_>>();
    basin_sizes.sort();
    let result = basin_sizes.iter().rev().take(3).product::<usize>();
    println!("Basin product: {}", result);

}

fn find_low_points(cave: &Vec<Vec<i32>>) -> Vec<(usize, usize)> {
    let mut lps = vec![];

    let rowmax = cave.len() - 1;
    let colmax = cave[0].len() - 1;

    for i in 0..=rowmax {
        for j in 0..=colmax {
            let height = cave[i][j];
            if (i == 0 || height < cave[i - 1][j]) &&
                (i == rowmax || height < cave[i + 1][j]) &&
                (j == 0 || height < cave[i][j - 1]) &&
                (j == colmax || height < cave[i][j + 1]) {
                lps.push((i, j))
            }
        }
    }

    lps
}

fn find_basins(cave: &Vec<Vec<i32>>, low_points: &Vec<(usize, usize)>) -> Vec<((usize, usize), Vec<(usize, usize)>)> {
    let mut basins = vec![];

    low_points.iter()
        .for_each(|lp| basins.push((lp.clone(), basin_area(&cave, &lp))));

    basins
}

fn basin_area(cave: &Vec<Vec<i32>>, low_point: &(usize, usize)) -> Vec<(usize, usize)> {
    basin_area_acc(&cave, low_point, vec![low_point.clone()])
}

fn basin_area_acc(cave: &Vec<Vec<i32>>, point: &(usize, usize), mut tracker: Vec<(usize, usize)>) -> Vec<(usize, usize)> {
    let rowmax = cave.len() - 1;
    let colmax = cave[0].len() - 1;
    let row = point.0;
    let col = point.1;

    if row != 0 && !tracker.contains(&(row - 1, col)) && cave[row - 1][col] != 9 {
        let next = (row - 1, col);
        tracker.push(next);
        tracker = basin_area_acc(&cave, &next, tracker);
    }

    if row != rowmax && !tracker.contains(&(row + 1, col)) && cave[row + 1][col] != 9 {
        let next = (row + 1, col);
        tracker.push(next);
        tracker = basin_area_acc(&cave, &next, tracker);
    }

    if col != 0 && !tracker.contains(&(row , col - 1)) && cave[row][col - 1] != 9 {
        let next = (row , col - 1);
        tracker.push(next);
        tracker = basin_area_acc(&cave, &next, tracker);
    }

    if col != colmax && !tracker.contains(&(row , col + 1)) && cave[row][col + 1] != 9 {
        let next = (row , col + 1);
        tracker.push(next);
        tracker = basin_area_acc(&cave, &next, tracker);
    }

    tracker
}