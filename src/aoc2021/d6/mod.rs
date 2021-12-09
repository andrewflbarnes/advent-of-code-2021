use crate::utils;

pub fn solve(input1: String, _: String, _: &[String]) {
    let lines = utils::read_file_lines(&input1);

    let mut track = vec![0; 9];
    
    lines.get(0).unwrap()
        .split(",")
        .map(|n| n.parse::<usize>().unwrap())
        .for_each(|n| track[n] += 1);

    println!("{:?}", track);

    let mut birthers: i32;
    for _ in 1..=80 {
        birthers = track[0];
        for j in 1..=8 {
            track[j-1] = track[j];
        }
        track[8] = birthers;
        track[6] += birthers;
    }
    println!("Day 80 fish: {}", track.iter().sum::<i32>());

}