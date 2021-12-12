use crate::utils;
mod cave;
use cave::Cave;
use std::collections::HashMap;
use std::cell::RefCell;

macro_rules! add_cave {
    ( $caves:ident, $id:expr ) => {
        let id_str = $id;
        if !$caves.contains_key(id_str) {
            let cave = Cave::new(id_str.clone());
            $caves.insert(id_str.clone(), RefCell::from(cave));
        }
    };
}

pub fn solve(input1: String, _: String, _: &[String]) {
    let lines = utils::read_file_lines(&input1);

    let mut caves: HashMap<String, RefCell<Cave>> = HashMap::new();
    lines.iter()
        .map(|l| l.split("-"))
        .map(|mut ids| (ids.nth(0).unwrap().to_string(), ids.nth(0).unwrap().to_string()))
        .for_each(|link| {
            add_cave!(caves, &link.0);
            add_cave!(caves, &link.1);
            let mut cave_1 = caves.get(&link.0).unwrap().borrow_mut();
            let mut cave_2 = caves.get(&link.1).unwrap().borrow_mut();
            cave_1.add_connection(&cave_2.id());
            cave_2.add_connection(&cave_1.id());
        });

    let all_routes = get_all_routes(&caves);
    println!("{} routes", all_routes.len());

    let all_routes = get_all_routes_2(&caves);
    println!("{} routes with small cave re-visit", all_routes.len());
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