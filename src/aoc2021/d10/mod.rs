use crate::utils;

pub fn solve(input1: String, _: String, _: &[String]) {
    let lines = utils::read_file_lines(&input1);

    let (errors, mut incompletes): (Vec<i64>, Vec<i64>) = lines.iter()
        .map(|l| validate_syntax(l))
        .partition(|s| *s >= 0);

    println!("Error lines score: {:?}", errors.iter().sum::<i64>());

    incompletes.sort();

    println!("Incomplete lines score: {:?}", -incompletes.get(incompletes.len() / 2).unwrap());
}

fn validate_syntax(line: &String) -> i64 {
    let mut stack = vec![];
    for c in line.chars() {
        let valid = match c {
            '[' | '{' | '<' | '(' => {
                stack.push(c);
                true
            },
            _ => {
                let open = match c {
                    ']' => '[',
                    '}' => '{',
                    '>' => '<',
                    ')' => '(',
                    _ => panic!("Invalid closing paren: {}", c),
                };

                let last = stack.pop();
                let ret = match last {
                    Some(l) => l == open,
                    None => false,
                };

                ret
            }
        };

        if !valid {
            return match c {
                ']' => 57,
                '}' => 1197,
                '>' => 25137,
                ')' => 3,
                _ => panic!("Invalid closing paren: {}", c),
            };
        }
    }

    if stack.len() == 0 {
        return 0;
    }
    
    let mut syntax_score = 0;
    for c in stack.iter().rev() {
        syntax_score *= 5;
        syntax_score += match c {
            '(' => 1,
            '[' => 2,
            '{' => 3,
            '<' => 4,
            _ => panic!("Invalid opening paren: {}", c),
        }
    }

    -syntax_score
}