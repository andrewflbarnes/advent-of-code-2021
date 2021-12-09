use crate::utils;

pub fn solve(input1: String, _: String, _: &[String]) {
    let lines = utils::read_file_lines(&input1);
    let start = lines.iter()
        .flat_map(|l| l.split(","))
        .map(|i| i.parse::<usize>().unwrap())
        .collect::<Vec<_>>();
    
    let count = start.iter().count();
    let sum = start.iter().sum::<usize>();
    let max = start.iter().max().unwrap();
    let mut track: Vec<usize> = vec![0; *max as usize + 1];

    start.iter().for_each(|i| track[*i] += 1);

    // println!("Start: {:?}", start);
    // println!("Track: {:?}", track);

    let mut lowest_pos = 0;
    let mut lowest_cost = 999999999;
    for i in 0..=*max {
        let mut cost = 0;
        for j in 0..i {
            cost += (i-j) * track[j];
        }
        for j in (i + 1)..=*max {
            cost += (j-i) * track[j];
        }

        if cost < lowest_cost {
            lowest_cost = cost;
            lowest_pos = i;
        }
    }

    println!("Lowest cost at position {} with {}", lowest_pos, lowest_cost);
}