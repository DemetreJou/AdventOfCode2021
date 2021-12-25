use std::collections::{HashMap, HashSet};
use std::fs;

static INPUT_FILE: &str = "test_input.txt";

fn create_adjacency_list(raw_input: &str) -> HashMap<&str, Vec<&str>> {
    let mut adjacency_list: HashMap<&str, Vec<&str>> = HashMap::new();
    raw_input.lines().for_each(|line| {
        let mut split = line.split("-");
        let from = split.next().unwrap();
        let to = split.next().unwrap();

        match adjacency_list.get_mut(from) {
            Some(list) => list.push(to),
            None => {
                let mut list = Vec::new();
                list.push(to);
                adjacency_list.insert(from, list);
            }
        }

        match adjacency_list.get_mut(to) {
            Some(list) => list.push(from),
            None => {
                let mut list = Vec::new();
                list.push(from);
                adjacency_list.insert(to, list);
            }
        }
    });
    adjacency_list
}

fn recursive_dfs_pt_1<'a, 'b>(
    source: &'a str,
    target: &str,
    visited: &'b mut HashSet<&'a str>,
    path_count: &mut i32,
    adjacency_list: &HashMap<&str, Vec<&'a str>>,
) {
    visited.insert(source);
    if source == target {
        *path_count += 1;
    } else {
        if !adjacency_list.contains_key(source) {
            return;
        }
        for neighbour in adjacency_list[source].iter() {
            if !visited.contains(*neighbour) || neighbour.chars().all(char::is_uppercase) {
                recursive_dfs_pt_1(neighbour, target, visited, path_count, adjacency_list);
            }
        }
    }
    visited.remove(source);
}

fn part_1(raw_input: &str) {
    let adjacency_list = create_adjacency_list(raw_input);
    let mut visited = HashSet::new();
    let mut count = 0;
    recursive_dfs_pt_1("start", "end", &mut visited, &mut count, &adjacency_list);
    println!("{}", count);
}

fn recursive_dfs_pt_2<'a, 'b>(
    source: &'a str,
    target: &str,
    visited_to_count: &'b mut HashMap<&'a str, i32>,
    path_count: &mut i32,
    adjacency_list: &HashMap<&str, Vec<&'a str>>,
) {
    let curr_count = *visited_to_count.entry(source).or_insert(0);
    visited_to_count.insert(source, curr_count + 1);
    if source == target {
        *path_count += 1;
    } else {
        if !adjacency_list.contains_key(source) {
            return;
        }
        adjacency_list[source].iter().for_each(|neighbour| {
            if visited_to_count[source] < 2
                || neighbour.chars().all(char::is_uppercase)
            {
                recursive_dfs_pt_2(
                    neighbour,
                    target,
                    visited_to_count,
                    path_count,
                    adjacency_list,
                );
            }
        });
    }
    let curr_count = visited_to_count[source];
    visited_to_count.insert(source, curr_count - 1);
}

fn part_2(raw_input: &str) {
    let adjacency_list = create_adjacency_list(raw_input);
    let mut visited_to_count = HashMap::new();
    let mut count = 0;
    recursive_dfs_pt_2(
        "start",
        "end",
        &mut visited_to_count,
        &mut count,
        &adjacency_list,
    );
    println!("\n{}", count);
}

fn main() {
    let raw_input = fs::read_to_string(INPUT_FILE).expect("Failed to read input.txt");
    part_1(&raw_input);
    part_2(&raw_input);
}
