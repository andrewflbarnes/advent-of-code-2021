use crate::utils;

pub fn solve(input1: String, _: String, _: &[String]) {
    let lines = utils::read_file_lines(&input1);

    let outputs = lines.iter()
        .filter_map(|l| l.split(" | ").nth(1))
        .flat_map(|l| l.split(" "))
        .collect::<Vec<_>>();

    let outputs_1478 = outputs.iter()
        .filter(|o| match o.len() {
            2 | 3 | 4 | 7 => true,
            _ => false,
        })
        .count();

    println!("{:?}", outputs_1478);
}