#[derive(Debug, Eq, PartialEq, Clone)]
pub struct Cell {
    number: String,
    marked: bool,
}

impl Cell {
    pub fn new(number: String) -> Self {
        Cell {
            number,
            marked: false,
        }
    }
    pub fn mark(&mut self) {
        self.marked = true;
    }
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct Board {
    entries: Vec<Vec<Cell>>
}

impl Board {
    pub fn new(entries: Vec<Vec<Cell>>) -> Self {
        Board {
            entries
        }
    }

    pub fn check_mark(&mut self, number: &String) {
        for row in self.entries.iter_mut() {
            for cell in row.iter_mut() {
                if cell.number == *number {
                    cell.mark();
                }
            }
        }
    }

    pub fn is_finished(&self) -> bool {
        self.is_finished_any_row() || self.is_finished_any_column()
    }

    fn is_finished_any_row(&self) -> bool {
        self.entries.iter()
            .any(|row| row.iter().all(|cell| cell.marked))
    }

    fn is_finished_any_column(&self) -> bool {
        let mut cols_finished: Vec<bool> = Vec::new();
        for _ in 0..self.entries.get(0).unwrap().len() {
            cols_finished.push(true);
        }
        self.entries.iter()
            .for_each(|row| row.iter()
                .enumerate()
                .for_each(|(i, cell)| cols_finished[i] &= cell.marked));
            // .any(|row| row.iter().all(|cell| cell.marked))

        cols_finished.iter().any(|c| *c)
    }

    pub fn unchecked_sum(&self) -> i32 {
        self.entries.iter()
            .flat_map(|row| row.iter())
            .filter(|cell| !cell.marked)
            .map(|cell| cell.number.parse::<i32>().unwrap())
            .reduce(|acc, next| acc + next)
            .unwrap()
    }
}