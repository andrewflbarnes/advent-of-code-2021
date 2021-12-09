use crate::utils;

pub fn solve(input1: String, _: String, _: &[String]) {
    let lines = utils::read_file_lines(&input1);

    let inout = lines.iter()
        .map(|l| {
            let mut io = l.split(" | ");
            (io.nth(0).unwrap(), io.nth(0).unwrap())
        })
        .collect::<Vec<_>>();

    let outputs_1478 = inout.iter()
        .map(|io| io.1)
        .flat_map(|l| l.split(" "))
        .filter(|o| match o.len() {
            2 | 3 | 4 | 7 => true,
            _ => false,
        })
        .count();

    println!("1478 Count: {:?}", outputs_1478);

    let sum = inout.iter()
        .map(|io| {
            let mapping = mapping(&io.0.to_string());
            io.1.split(" ")
                .map(|d| {
                    for (i, m) in mapping.iter().enumerate() {
                        if m.len() == d.len() && m.chars().all(|c| d.contains(c)) {
                            return i.to_string();
                        }
                    }
                    panic!("Couldn't find mapping for {} in {:?}", d, mapping);
                })
                .reduce(|acc, next| acc + &next)
                .unwrap()
                .parse::<u32>()
                .unwrap()
        })
        .sum::<u32>();

    println!("Sum or outputs: {:?}", sum);
}

fn mapping(input: &String) -> Vec<String> {
    let mut mapping = vec!["".to_string(); 10];
    let inputs = input.split(" ")
        .collect::<Vec<_>>();

        let mut m1 = String::new();
        let mut m4 = String::new();
        let mut m7 = String::new();
        let mut m8 = String::new();
    for i in inputs.iter()  {
        let i_str = i.to_string();
        match i.len() {
            2 => m1 = i_str,
            3 => m7 = i_str,
            4 => m4 = i_str,
            7 => m8 = i_str,
            _ => {},
        };
    }

    mapping[1] = m1.clone();
    mapping[4] = m4.clone();
    mapping[7] = m7.clone();
    mapping[8] = m8.clone();
    
    for i in inputs.iter() {
        let i_str = i.to_string();
        if i.len() == 6 {
            if m4.chars().all(|c| i_str.contains(c)){
                mapping[9] = i_str;
            } else if m1.chars().all(|c| i_str.contains(c)) {
                mapping[0] = i_str;
            } else {
                mapping[6] = i_str;
            }
        } else if i.len() == 5 {
            if m1.chars().all(|c| i_str.contains(c)) {
                mapping[3] = i_str;
            } else if m4.chars().filter(|c| i_str.contains(*c)).count() == 3 {
                mapping[5] = i_str
            } else {
                mapping[2] = i_str
            }
        }
    }

    mapping
}