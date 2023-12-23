use std::fs;
use std::cmp::Ordering;

fn main() {
    let file = fs::read_to_string("src/test.txt").unwrap();
    let lines: Vec<&str> = file.split_terminator("\n").collect();
    let mut map: Vec<Vec<Node>> = Vec::new();
 
    // Create nodes.
    for i in 0..lines.len() {
        map.push(Vec::new());
        for j in 0..lines[0].len() {
            map[i].push(Node { distance_from_neighbours: String::from(lines[i].chars().collect::<Vec<char>>()[j]).parse::<u8>().unwrap(), distance_from_start: isize::MAX, location: Location { row: i, col: j } });
        }
    }

    map[0][0].distance_from_start = 0;
    map[0][1].distance_from_start = map[0][1].distance_from_neighbours as isize;
    map[1][0].distance_from_start = map[1][0].distance_from_neighbours as isize;

    let mut min_heap: Vec<Node> = Vec::new();
    let mut known_nodes: Vec<Node> = Vec::new();

    for row in map.iter() {
        for node in row.iter() {
            if node.distance_from_start != 0 {
                min_heap.push(node.to_owned());
            } else {
                known_nodes.push(node.to_owned());
            }
        }
    }

    drop(map);

    let mut shortest_distance: isize = -1;
    while shortest_distance == -1 {
        min_heap.sort();

        let next_node = min_heap.remove(0);

        if next_node.location.row == lines.len() - 1 && next_node.location.col == lines[0].len() - 1 {
            shortest_distance = next_node.distance_from_start;
        }

        let neighbours = get_unknown_neighbours_of_node(&min_heap, &next_node, lines.len(), lines[0].len());
        
        for neighbour in neighbours.into_iter() {
            if min_heap.contains(&neighbour) {
                let neighbour_index = get_node_index(&min_heap, &neighbour).unwrap();
                min_heap[neighbour_index].distance_from_start = next_node.distance_from_start + neighbour.distance_from_neighbours as isize;
            }
        }
    }

    println!("{}", shortest_distance);
}

fn get_node_index(nodes: &Vec<Node>, node: &Node) -> Option<usize> {
    for index in 0..nodes.len() {
        if node.location == nodes[index].location {
            return Some(index);
        }
    }

    None
}

fn get_node_ref<'n>(nodes: &'n Vec<Node>, node: &Node) -> Option<&'n Node> {
    let index = get_node_index(nodes, node);

    if index.is_some() {
        return nodes.get(index.unwrap());
    }

    None
}

fn get_unknown_neighbours_of_node<'n>(unknown_nodes: &Vec<Node>, node: &Node, rows: usize, cols: usize) -> Vec<Node> {
    let mut neighbours :Vec<Node> = Vec::new();
    
    let mut location_node = Node { distance_from_start: 0, distance_from_neighbours: 0, location: Location { row: node.location.row, col: node.location.col } };

    if node.location.row as isize - 1 >= 0 {
        location_node.location.row -= 1;

        let neighbour = get_node_ref(unknown_nodes, &location_node);

        if neighbour.is_some() {
            neighbours.push(neighbour.unwrap().clone());
        }

        location_node.location.row += 1;
    }
    
    if node.location.row + 1 < rows {
        location_node.location.row += 1;

        let neighbour = get_node_ref(unknown_nodes, &location_node);

        if neighbour.is_some() {
            neighbours.push(neighbour.unwrap().clone());
        }

        location_node.location.row -= 1;
    }
    
    if node.location.col as isize - 1 >= 0 {
        location_node.location.col -= 1;

        let neighbour = get_node_ref(unknown_nodes, &location_node);

        if neighbour.is_some() {
            neighbours.push(neighbour.unwrap().clone());
        }

        location_node.location.col += 1;
    }

    if node.location.col + 1 < cols {
        location_node.location.col += 1;

        let neighbour = get_node_ref(unknown_nodes, &location_node);

        if neighbour.is_some() {
            neighbours.push(neighbour.unwrap().clone());
        }

        location_node.location.row -= 1;
    }

    neighbours
}

#[derive(Debug, Clone)]
struct Node {
    distance_from_neighbours: u8,
    distance_from_start: isize,
    location: Location
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Location {
    row: usize,
    col: usize
}

impl PartialEq for Node {
    fn eq(&self, other: &Self) -> bool {
        &self.distance_from_start == &other.distance_from_start
    }
}

impl Eq for Node {}

impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Node {
    fn cmp(&self, other: &Self) -> Ordering {
        self.distance_from_start.cmp(&other.distance_from_start)
    }
}