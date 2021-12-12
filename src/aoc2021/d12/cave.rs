use std::cmp::Ordering;

#[derive(Debug, Eq, PartialEq)]
pub struct Cave {
    id: String,
    connections: Vec<String>,
}

impl Ord for Cave {
    fn cmp(&self, other: &Self) -> Ordering {
        self.id.cmp(&other.id)
    }
}

impl PartialOrd for Cave {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Cave {
pub fn new(id: String) -> Self {
        Cave {
            id,
            connections: vec![],
        }
    }

    pub fn add_connection(&mut self, cave_id: &String) {
        self.connections.push(cave_id.clone())
    }

    pub fn id(&self) -> String {
        self.id.clone()
    }

    pub fn connections(&self) -> &Vec<String> {
        &self.connections
    }
}