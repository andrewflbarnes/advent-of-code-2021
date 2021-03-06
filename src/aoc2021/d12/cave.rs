#[derive(Debug)]
pub struct Cave {
    id: String,
    connections: Vec<String>,
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