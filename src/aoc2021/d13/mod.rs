use crate::utils;
mod fold;
use fold::Fold;

pub fn solve(input1: String, _: String, _: &[String]) {
    let lines = utils::read_file_lines(&input1);

    let break_line = lines.iter()
        .position(|line| line == "").unwrap();
    
    let mut dots = lines[0..break_line].iter()
        .map(|line| line.split(",").map(|coord| coord.parse::<u16>().unwrap()))
        .map(|mut coords| (coords.next().unwrap(), coords.next().unwrap()))
        .collect::<Vec<_>>();

    // dots.iter().for_each(|dot| println!("{:?}", dot));

    let folds = lines[(break_line + 1)..].iter()
        .map(|line| line.parse::<Fold>().unwrap())
        .collect::<Vec<_>>();
    
    // folds.iter().for_each(|fold| println!("{:?}", fold));

    let mut first = true;
    folds.iter()
        .for_each(|fold| {
            dots = dots.iter()
                .map(|dot| fold.reflect_point(dot))
                .collect::<Vec<_>>();
            dots.sort();
            dots.dedup();
            if first {
                first = false;
                println!("Dots after single iteration: {}", dots.len());
            }
        });

    // dots.iter().for_each(|dot| println!("{:?}", dot));

    print_dots(&dots);
}

fn print_dots(dots: &Vec<(u16, u16)>) {
    for i in 0..=5 {
        for j in 0..=38 {
            print!("{}", if dots.contains(&(j, i)) {"#"} else {"."});
        }
        println!("")
    }
}