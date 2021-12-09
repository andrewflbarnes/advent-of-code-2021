use crate::utils;

pub fn solve(input1: String, _: String, _: &[String]) {
    let lines = utils::read_file_lines(&input1);
    let start = lines.iter()
        .flat_map(|l| l.split(","))
        .map(|i| i.parse::<usize>().unwrap())
        .collect::<Vec<_>>();
    
    let max = start.iter().max().unwrap();
    let mut track: Vec<usize> = vec![0; *max as usize + 1];

    start.iter().for_each(|i| track[*i] += 1);

    // println!("Start: {:?}", start);
    // println!("Track: {:?}", track);

    let mut lowest_pos = 0;
    let mut lowest_cost = 999999999;
    let mut lowest_pos_2 = 0;
    let mut lowest_cost_2 = 999999999;
    for i in 0..=*max {
        let mut cost = 0;
        let mut cost_2 = 0;
        for j in 0..i {
            cost += (i-j) * track[j];
            cost_2 += travel_cost_2(i-j) * track[j];
        }
        for j in (i + 1)..=*max {
            cost += (j-i) * track[j];
            cost_2 += travel_cost_2(j-i) * track[j];
        }

        if cost < lowest_cost {
            lowest_cost = cost;
            lowest_pos = i;
        }

        if cost_2 < lowest_cost_2 {
            lowest_cost_2 = cost_2;
            lowest_pos_2 = i;
        }
    }

    println!("Lowest cost at position {} with {}", lowest_pos, lowest_cost);
    println!("Lowest cost 2 at position {} with {}", lowest_pos_2, lowest_cost_2);
}

fn travel_cost_2(dist: usize) -> usize {
    ((dist * dist) + dist) / 2
}