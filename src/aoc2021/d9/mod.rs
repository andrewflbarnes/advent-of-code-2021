use crate::utils;

pub fn solve(input1: String, _: String, _: &[String]) {
    let lines = utils::read_file_lines(&input1);

    let cave = lines.iter()
        .map(|l| l.chars()
            .map(|c| c.to_string().parse::<u32>().unwrap())
            .collect::<Vec<_>>())
        .collect::<Vec<_>>();

    // println!("{:?}", cave);

    let lps = find_low_points(&cave);
    println!("Low points: {:?}", lps);

    let result = lps.iter()
        .map(|p| cave[p.0][p.1] + 1)
        .sum::<u32>();
    println!("Low point sum: {:?}", result);

    let basins = find_basins(&cave, &lps);
    println!("{:?}", basins);
}

fn find_low_points(cave: &Vec<Vec<u32>>) -> Vec<(usize, usize)> {
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

fn find_basins(cave: &Vec<Vec<u32>>, low_points: &Vec<(usize, usize)>) -> Vec<(usize, usize, u32)> {
    let mut basins = vec![];

    basins
}