use crate::utils;
mod cave;
use cave::Cave;
use std::collections::HashMap;
use std::cell::RefCell;

pub fn solve(input1: String, _: String, _: &[String]) {
    let lines = utils::read_file_lines(&input1);

    let mut caves: HashMap<String, RefCell<Cave>> = HashMap::new();
    let add_caves = cave_adder(&mut caves);
    lines.iter()
        .for_each(add_caves);

    let all_routes = get_all_routes(&caves);
    println!("{} routes", all_routes.len());

    let all_routes = get_all_routes_2(&caves);
    println!("{} routes with small cave re-visit", all_routes.len());
}

fn cave_adder<'a>(caves: &'a mut HashMap<String, RefCell<Cave>>) -> Box<dyn FnMut(&String) + 'a> {
    Box::new(move |line| {
        let mut ids = line.split("-");
        let link = (ids.nth(0).unwrap().to_string(), ids.nth(0).unwrap().to_string());
        
        add_cave(caves, &link.0);
        add_cave(caves, &link.1);

        let mut cave_1 = caves.get(&link.0).unwrap().borrow_mut();
        let mut cave_2 = caves.get(&link.1).unwrap().borrow_mut();

        cave_1.add_connection(&cave_2.id());
        cave_2.add_connection(&cave_1.id());
    })
}

fn add_cave(caves: &mut HashMap<String, RefCell<Cave>>, id: &String) {
    if !caves.contains_key(id) {
        let cave = Cave::new(id.clone());
        caves.insert(id.clone(), RefCell::from(cave));
    }
}

fn get_all_routes(caves: &HashMap<String, RefCell<Cave>>) -> Vec<Vec<String>> {
    get_routes(caves, vec!["start".to_string()], true)
}

fn get_all_routes_2(caves: &HashMap<String, RefCell<Cave>>) -> Vec<Vec<String>> {
    get_routes(caves, vec!["start".to_string()], false)
}

fn get_routes(caves: &HashMap<String, RefCell<Cave>>, path: Vec<String>, had_second_visit: bool) -> Vec<Vec<String>> {
    let mut paths: Vec<Vec<String>> = vec![];

    let last_cave = caves.get(path.last().unwrap()).unwrap().borrow();
    if last_cave.id() == "end" {
        return vec![path];
    }
    
    let last_cave_connections = last_cave.connections();
    for next in last_cave_connections {
        let uppercase = next.chars().all(|c| c.is_uppercase());
        let visited = path.contains(next);
        let can_second_visit = !had_second_visit && next != "end" && next != "start";

        if uppercase || !visited || can_second_visit {
            let mut new_path = path.clone();
            new_path.push(next.clone());

            if next == "end" {
                paths.push(new_path);
            } else {
                for onward_path in get_routes(caves, new_path, had_second_visit || (!uppercase && visited)) {
                    paths.push(onward_path);
                }
            }
        }

    }

    paths
}