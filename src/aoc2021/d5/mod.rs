use crate::utils;

type VentVecs = Vec<((usize, usize), (usize, usize))>;

pub fn solve(input1: String, _: String, _: &[String]) {
    let lines = utils::read_file_lines(&input1);

    let mut field: Vec<Vec<u32>> = vec![vec![0; 1000]; 1000];

    let vecs: VentVecs = get_vecs(&lines);

    // vecs.iter().for_each(|v| println!("{:?}", v));

    apply_orth_vecs(&mut field, &vecs);
    println!("Orthoganal danger zones: {}", danger_zones(&field));

    apply_diag_vecs(&mut field, &vecs);
    println!("Full danger zones: {}", danger_zones(&field));
}

fn get_vecs(lines: &Vec<String>) -> VentVecs {
    lines.iter().map(|l| {
        let points = l.split(" -> ")
            .map(|p| {
                let coords = p.split(",")
                    .map(|i| i.parse::<usize>().unwrap())
                    .collect::<Vec<_>>();
                (coords[0], coords[1])
            })
            .collect::<Vec<_>>();

        let p1 = points[0];
        let p2 = points[1];

        return if p1.0 < p2.0 || p1.1 < p2.1 {
            (p1, p2)
        } else {
            (p2, p1)
        }
    })
    .collect::<Vec<_>>()
}

fn apply_orth_vecs(field: &mut Vec<Vec<u32>>, vecs: &VentVecs) {
    vecs.iter()
        .filter(|v| v.0.0 == v.1.0 || v.0.1 == v.1.1)
        .for_each(|v| {
            if v.0.0 == v.1.0 {
                let x = v.0.0;
                for y in v.0.1..=v.1.1 {
                    field[x][y] += 1;
                }
            } else {
                let y = v.0.1;
                for x in v.0.0..=v.1.0 {
                    field[x][y] += 1;
                }
            }
        })
}

type Next = Box<dyn Fn(usize) -> usize>;

fn apply_diag_vecs(field: &mut Vec<Vec<u32>>, vecs: &VentVecs) {
    let incrementer: Next = Box::new(|x: usize| x + 1);
    let decrementer: Next = Box::new(|x: usize| x - 1);
    vecs.iter()
        .filter(|v| v.0.0 != v.1.0 && v.0.1 != v.1.1)
        .map(|v| if v.0.0 < v.1.0 {
            (v.0, v.1)
        } else {
            (v.1, v.0)
        })
        .for_each(|v| {
            // println!("Processing {:?}", v);
            let x_next: &Next;
            if v.0.0 < v.1.0 {
                x_next = &incrementer;
            } else {
                x_next = &decrementer;
            }
            let y_next: &Next;
            if v.0.1 < v.1.1 {
                y_next = &incrementer;
            } else {
                y_next = &decrementer;
            }
            let mut x = v.0.0;
            let mut y = v.0.1;
            for _ in v.0.0..=v.1.0  {
                // println!("=> Processing {},{}", x, y);
                field[x][y] += 1;
                x = x_next(x);
                y = y_next(y);
            }
        })
}

fn danger_zones(field: &Vec<Vec<u32>>) -> usize {
    field.iter()
        .flat_map(|row| row.iter())
        .filter(|p| **p > 1)
        .count()
}