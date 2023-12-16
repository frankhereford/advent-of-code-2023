#![allow(unreachable_code)]
#![allow(unused_variables)]
#![allow(dead_code)]
use web_sys::console;
use wasm_bindgen::prelude::*;
use indexmap::IndexMap;

#[wasm_bindgen(module = "/src/solutions/workerHelpers.js")]
extern "C" {
    fn postMessageToWorker(do_print: bool, message: &str);
}

#[derive(Debug)]
struct Connection {
    cxn_one: Option<(isize, isize)>,
    cxn_two: Option<(isize, isize)>,
    character: char,
}
type Row = Vec<Connection>;
type Grid = Vec<Row>; // ! Remember, Gird[Y][X] is the correct way to access the grid, not [X][Y]

pub fn solution_part_1() -> () {
    return;
    postMessageToWorker(true, "Part 1: \n");
    let mut iteration = 0;
    //let content = include_str!("input/day_10_part_1_test_input_2.txt");
    let content = include_str!("input/day_10_part_1_input.txt");

    let mut grid: Grid = Vec::new();
    let mut start: (isize, isize) = (0, 0);


    content.lines().for_each(|line| {
        let mut row: Row = Vec::new();
        let characters: Vec<_> = line.chars().collect();

        for (index, character) in characters.iter().enumerate() {
            //postMessageToWorker(show_message, &format!("Character: {} / {} / {}", character, iteration, index));

            let index = index as isize; 

            let mut cxn_one: Option<(isize, isize)> = None;
            let mut cxn_two: Option<(isize, isize)> = None;
            match *character {
                'S' => {
                    start = (index, iteration);
                },
                'J' => {
                    cxn_one = Some((index, iteration - 1)); //north
                    cxn_two = Some((index - 1, iteration)); //west
                },
                'L' => {
                    cxn_one = Some((index, iteration - 1)); // north
                    cxn_two = Some((index + 1, iteration)); // east
                },
                'F' => {
                    cxn_one = Some((index, iteration + 1)); // south
                    cxn_two = Some((index + 1, iteration)); // east
                },
                '7' => {
                    cxn_one = Some((index, iteration + 1)); // south
                    cxn_two = Some((index - 1, iteration)); // west
                },
                '|' => {
                    cxn_one = Some((index, iteration - 1)); // north
                    cxn_two = Some((index, iteration + 1)); // south
                },
                '-' => {
                    cxn_one = Some((index - 1, iteration)); // east
                    cxn_two = Some((index + 1, iteration)); // west
                },
                _ => {}
            }
            //postMessageToWorker(show_message, &format!("letter: {}, cxn_one: {:?}, cxn_two: {:?}", character, cxn_one, cxn_two));
            let connection = Connection {
                cxn_one: cxn_one,
                cxn_two: cxn_two,
                character: *character,
            };

            row.push(connection);

        }
    grid.push(row);
    iteration += 1;
    });

    //postMessageToWorker(true, &format!("Grid: {:?}", grid));
    //postMessageToWorker(true, &format!("Cell 0,0: {:?}", grid[0][0]));
    //postMessageToWorker(true, &format!("Cell 4,4: {:?}", grid[4][4]));
    //postMessageToWorker(true, &format!("Cell 1,2: {:?}", grid[1][2]));
    //postMessageToWorker(true, &format!("Start: {:?}", start));

    postMessageToWorker(true, &format!("Start x, y: {:?}, {:?}", start.0, start.1));
    let start_x = start.0;
    let start_y = start.1;

    let mut initial_connection: Option<(isize, isize)> = None;
    // check north
    if start_y > 0 && !initial_connection.is_some() { // no underflow
        let north_x: usize = start_x as usize;
        let north_y: usize = start_y as usize - 1;
        //postMessageToWorker(true, &format!("cell to the north: {:?}", grid[north_y][north_x]));
        if grid[north_y][north_x].cxn_one == Some((start_x, start_y)) || grid[north_y][north_x].cxn_two == Some((start_x, start_y)) {
            postMessageToWorker(true, &format!("Found connection to the north: {:?}", grid[north_y][north_x]));
            initial_connection = Some((north_x as isize, north_y as isize));
        }
    }

    // check east
    if start_x < grid[0].len() as isize - 1 && !initial_connection.is_some() { // no overflow
        let east_x: usize = start_x as usize + 1;
        let east_y: usize = start_y as usize;
        //postMessageToWorker(true, &format!("cell to the east: {:?}", grid[east_y][east_x]));
        if grid[east_y][east_x].cxn_one == Some((start_x, start_y)) || grid[east_y][east_x].cxn_two == Some((start_x, start_y)) {
            postMessageToWorker(true, &format!("Found connection to the east: {:?}", grid[east_y][east_x]));
            initial_connection = Some((east_x as isize, east_y as isize));
        }
    }

    // check south
    if start_y < grid.len() as isize - 1 && !initial_connection.is_some() { // no overflow
        let south_x: usize = start_x as usize;
        let south_y: usize = start_y as usize + 1;
        //postMessageToWorker(true, &format!("cell to the south: {:?}", grid[south_y][south_x]));
        if grid[south_y][south_x].cxn_one == Some((start_x, start_y)) || grid[south_y][south_x].cxn_two == Some((start_x, start_y)) {
            postMessageToWorker(true, &format!("Found connection to the south: {:?}", grid[south_y][south_x]));
            initial_connection = Some((south_x as isize, south_y as isize));
        }
    }

    // check west
    if start_x > 0 && !initial_connection.is_some() { // no underflow
        let west_x: usize = start_x as usize - 1;
        let west_y: usize = start_y as usize;
        //postMessageToWorker(true, &format!("cell to the west: {:?}", grid[west_y][west_x]));
        if grid[west_y][west_x].cxn_one == Some((start_x, start_y)) || grid[west_y][west_x].cxn_two == Some((start_x, start_y)) {
            postMessageToWorker(true, &format!("Found connection to the west: {:?}", grid[west_y][west_x]));
            initial_connection = Some((west_x as isize, west_y as isize));
        }
    }

    let mut path: IndexMap<(isize, isize), isize> = IndexMap::new();
    let mut distance_travelled: isize = 0;
    path.insert(start, distance_travelled);

    let mut current_location = start;
    let mut next_location = initial_connection.unwrap();
    let mut next_next_location = pick_outgoing_connection(current_location, &grid[next_location.1 as usize][next_location.0 as usize]);
    distance_travelled += 1;
    path.insert(next_location, distance_travelled);
    //postMessageToWorker(true, &format!("Current location: {:?}, {:?}", current_location, grid[current_location.1 as usize][current_location.0 as usize]));
    //postMessageToWorker(true, &format!("Next location: {:?}, {:?}", next_location, grid[next_location.1 as usize][next_location.0 as usize]));
    //postMessageToWorker(true, &format!("Next next location: {:?}, {:?}", next_next_location, grid[next_next_location.unwrap().1 as usize][next_next_location.unwrap().0 as usize]));
    //postMessageToWorker(true, &format!("Path: {:?}", path));

    loop {
        current_location = next_location;
        next_location = next_next_location.unwrap();
        next_next_location = pick_outgoing_connection(current_location, &grid[next_location.1 as usize][next_location.0 as usize]);
        distance_travelled += 1;
        path.insert(next_location, distance_travelled);
        //postMessageToWorker(true, &format!("Current location: {:?}, {:?}", current_location, grid[current_location.1 as usize][current_location.0 as usize]));
        //postMessageToWorker(true, &format!("Next location: {:?}, {:?}", next_location, grid[next_location.1 as usize][next_location.0 as usize]));
        //postMessageToWorker(true, &format!("Next next location: {:?}, {:?}", next_next_location, grid[next_next_location.unwrap().1 as usize][next_next_location.unwrap().0 as usize]));
        //postMessageToWorker(true, &format!("Path: {:?}", path));
        if next_next_location.unwrap() == start {
            postMessageToWorker(true, &format!("Found the start again!"));
            break;
        }
    }
    
    let mut midpoint: isize = 0;
    if let Some((_, last_value)) = path.get_index(path.len() - 1) {
        //postMessageToWorker(true, &format!("Last value: {}", last_value));
        midpoint = (last_value + 1) / 2;
    }
    postMessageToWorker(true, &format!("Midpoint: {}", midpoint));

    let mut distances = Vec::new();
    for (index, node) in path.iter().enumerate() {
        if index == 0 {
            continue;
        }
        //postMessageToWorker(true, &format!("Node: {:?}", node));
        let location = node.0;
        let mut distance = node.1;
        if distance > &midpoint {
            distances.push((location, midpoint - (distance - midpoint)));
        }
        else {
            distances.push((location, *distance));
        }
    }
    
    //postMessageToWorker(true, &format!("Distances: {:?}", distances));
}

fn pick_outgoing_connection(current_location: (isize, isize), cell: &Connection) -> Option<(isize, isize)> {
    if cell.cxn_one == Some(current_location) {
        cell.cxn_two
    } else if cell.cxn_two == Some(current_location) {
        cell.cxn_one
    } else {
        None
    }
}






pub fn solution_part_2() -> () {
    postMessageToWorker(true, "Part 2: \n");
    let mut iteration = 0;
    //let content = include_str!("input/day_10_part_2_test_input_3.txt");
    let content = include_str!("input/day_10_part_1_input.txt");

    let mut grid: Grid = Vec::new();
    let mut start: (isize, isize) = (0, 0);

    let mut input: Vec<Vec<char>> = Vec::new();

    //content.lines().for_each(|line| {
    for (index, line) in content.lines().enumerate() {
        let mut input_row: Vec<char> = Vec::new();

        let mut row: Row = Vec::new();
        let characters: Vec<_> = line.chars().collect();

        for (index, character) in characters.iter().enumerate() {
            if *character == 'S' { // ! Remember to set this to the right character for the input! hacky but easy
                input_row.push('7');
            } else {
                input_row.push(character.clone());
            }
            //postMessageToWorker(show_message, &format!("Character: {} / {} / {}", character, iteration, index));

            let index = index as isize; 

            let mut cxn_one: Option<(isize, isize)> = None;
            let mut cxn_two: Option<(isize, isize)> = None;
            match *character {
                'S' => {
                    start = (index, iteration);
                },
                'J' => {
                    cxn_one = Some((index, iteration - 1)); //north
                    cxn_two = Some((index - 1, iteration)); //west
                },
                'L' => {
                    cxn_one = Some((index, iteration - 1)); // north
                    cxn_two = Some((index + 1, iteration)); // east
                },
                'F' => {
                    cxn_one = Some((index, iteration + 1)); // south
                    cxn_two = Some((index + 1, iteration)); // east
                },
                '7' => {
                    cxn_one = Some((index, iteration + 1)); // south
                    cxn_two = Some((index - 1, iteration)); // west
                },
                '|' => {
                    cxn_one = Some((index, iteration - 1)); // north
                    cxn_two = Some((index, iteration + 1)); // south
                },
                '-' => {
                    cxn_one = Some((index - 1, iteration)); // east
                    cxn_two = Some((index + 1, iteration)); // west
                },
                _ => {}
            }
            //postMessageToWorker(show_message, &format!("letter: {}, cxn_one: {:?}, cxn_two: {:?}", character, cxn_one, cxn_two));
            let connection = Connection {
                cxn_one: cxn_one,
                cxn_two: cxn_two,
                character: *character,
            };

            row.push(connection);

        }
    grid.push(row);
    input.push(input_row.clone());
    iteration += 1;
    }

    //postMessageToWorker(true, &format!("Input: {:?}", input));

    //postMessageToWorker(true, &format!("Start x, y: {:?}, {:?}", start.0, start.1));
    let start_x = start.0;
    let start_y = start.1;

    let mut initial_connection: Option<(isize, isize)> = None;
    // check north
    if start_y > 0 && !initial_connection.is_some() { // no underflow
        let north_x: usize = start_x as usize;
        let north_y: usize = start_y as usize - 1;
        //postMessageToWorker(true, &format!("cell to the north: {:?}", grid[north_y][north_x]));
        if grid[north_y][north_x].cxn_one == Some((start_x, start_y)) || grid[north_y][north_x].cxn_two == Some((start_x, start_y)) {
            postMessageToWorker(true, &format!("Found connection to the north: {:?}", grid[north_y][north_x]));
            initial_connection = Some((north_x as isize, north_y as isize));
        }
    }

    // check east
    if start_x < grid[0].len() as isize - 1 && !initial_connection.is_some() { // no overflow
        let east_x: usize = start_x as usize + 1;
        let east_y: usize = start_y as usize;
        //postMessageToWorker(true, &format!("cell to the east: {:?}", grid[east_y][east_x]));
        if grid[east_y][east_x].cxn_one == Some((start_x, start_y)) || grid[east_y][east_x].cxn_two == Some((start_x, start_y)) {
            postMessageToWorker(true, &format!("Found connection to the east: {:?}", grid[east_y][east_x]));
            initial_connection = Some((east_x as isize, east_y as isize));
        }
    }

    // check south
    if start_y < grid.len() as isize - 1 && !initial_connection.is_some() { // no overflow
        let south_x: usize = start_x as usize;
        let south_y: usize = start_y as usize + 1;
        //postMessageToWorker(true, &format!("cell to the south: {:?}", grid[south_y][south_x]));
        if grid[south_y][south_x].cxn_one == Some((start_x, start_y)) || grid[south_y][south_x].cxn_two == Some((start_x, start_y)) {
            postMessageToWorker(true, &format!("Found connection to the south: {:?}", grid[south_y][south_x]));
            initial_connection = Some((south_x as isize, south_y as isize));
        }
    }

    // check west
    if start_x > 0 && !initial_connection.is_some() { // no underflow
        let west_x: usize = start_x as usize - 1;
        let west_y: usize = start_y as usize;
        //postMessageToWorker(true, &format!("cell to the west: {:?}", grid[west_y][west_x]));
        if grid[west_y][west_x].cxn_one == Some((start_x, start_y)) || grid[west_y][west_x].cxn_two == Some((start_x, start_y)) {
            postMessageToWorker(true, &format!("Found connection to the west: {:?}", grid[west_y][west_x]));
            initial_connection = Some((west_x as isize, west_y as isize));
        }
    }

    let mut path: IndexMap<(isize, isize), isize> = IndexMap::new();
    let mut distance_travelled: isize = 0;
    path.insert(start, distance_travelled);

    let mut current_location = start;
    let mut next_location = initial_connection.unwrap();
    let mut next_next_location = pick_outgoing_connection(current_location, &grid[next_location.1 as usize][next_location.0 as usize]);
    distance_travelled += 1;
    path.insert(next_location, distance_travelled);

    loop {
        current_location = next_location;
        next_location = next_next_location.unwrap();
        next_next_location = pick_outgoing_connection(current_location, &grid[next_location.1 as usize][next_location.0 as usize]);
        distance_travelled += 1;
        path.insert(next_location, distance_travelled);
        if next_next_location.unwrap() == start {
            postMessageToWorker(true, &format!("Found the start again!"));
            break;
        }
    }

    //postMessageToWorker(true, &format!("Path: {:?}", path));

    let mut y = 0;
    let mut inside_count = 0;
    let north_connections = vec!['L', 'J'];
    let south_connections = vec!['F', '7'];
    //vertical_node_designators.contains(&input[y][x])
    while y < input.len() {
        console::log_1(&format!("\nLine {:?}", y).into());
        let mut x = 0;
        let mut is_inside = false;
        let mut line = String::new();
        let mut seen_north_connection = false;
        let mut seen_south_connection = false;
        while x < input[y].len() {

            let character = input[y][x];

            if check_if_node(x, y, &path) && character == '|' {
                // we have crossed a vertical path line
                is_inside = !is_inside;
                //console::log_1(&"Flipping for |".into());
                line.push(character);
            } else if check_if_node(x, y, &path) && north_connections.contains(&character) {
                // we're entering or exiting a horizontal stretch from the north 
                seen_north_connection = !seen_north_connection;
                if seen_north_connection && seen_south_connection {
                    is_inside = !is_inside;
                    seen_north_connection = false;
                    seen_south_connection = false;
                    //console::log_1(&format!("Flipping to {} after {}", is_inside, character).into());
                }
                line.push('N');
            } else if check_if_node(x, y, &path) && south_connections.contains(&character) {
                // we're entering or exiting a horizontal stretch from the south
                seen_south_connection = !seen_south_connection;
                if seen_north_connection && seen_south_connection {
                    is_inside = !is_inside;
                    seen_north_connection = false;
                    seen_south_connection = false;
                    //console::log_1(&format!("Flipping to {} after {}", is_inside, character).into());
                }
                line.push('S');
            } else if check_if_node(x, y, &path) {
                line.push(character);
            } else if is_inside {
                line.push('I');
                inside_count += 1;
            } else {
                line.push('0');
            }
            
            //console::log_1(&format!("after character: {:?}, is_inside {:?}, seen_north_connection {:?}, seen_south_connection {:?}", character, is_inside, seen_north_connection, seen_south_connection).into());
            
            x += 1;
        }
        postMessageToWorker(true, &format!("Line {:?}: {}", y, line));
        y += 1;
    }
    postMessageToWorker(true, &format!("Inside count: {}", inside_count));

}

fn check_if_node(x: usize, y: usize, path: &IndexMap<(isize, isize), isize>) -> bool {
    for (index, node) in path.iter().enumerate() {
        let node_x = node.0 .0 as usize;
        let node_y = node.0 .1 as usize;
        if node_x == x && node_y == y {
            return true
        }
    }
    return false
}





pub fn solution_part_2_shoelace() -> () {
    postMessageToWorker(true, "Part 2: \n");
    let mut iteration = 0;
    let content = include_str!("input/day_10_part_1_test_input_1.txt");
    //let content = include_str!("input/day_10_part_1_input.txt");

    let mut grid: Grid = Vec::new();
    let mut start: (isize, isize) = (0, 0);


    content.lines().for_each(|line| {
        let mut row: Row = Vec::new();
        let characters: Vec<_> = line.chars().collect();

        for (index, character) in characters.iter().enumerate() {
            //postMessageToWorker(show_message, &format!("Character: {} / {} / {}", character, iteration, index));

            let index = index as isize; 

            let mut cxn_one: Option<(isize, isize)> = None;
            let mut cxn_two: Option<(isize, isize)> = None;
            match *character {
                'S' => {
                    start = (index, iteration);
                },
                'J' => {
                    cxn_one = Some((index, iteration - 1)); //north
                    cxn_two = Some((index - 1, iteration)); //west
                },
                'L' => {
                    cxn_one = Some((index, iteration - 1)); // north
                    cxn_two = Some((index + 1, iteration)); // east
                },
                'F' => {
                    cxn_one = Some((index, iteration + 1)); // south
                    cxn_two = Some((index + 1, iteration)); // east
                },
                '7' => {
                    cxn_one = Some((index, iteration + 1)); // south
                    cxn_two = Some((index - 1, iteration)); // west
                },
                '|' => {
                    cxn_one = Some((index, iteration - 1)); // north
                    cxn_two = Some((index, iteration + 1)); // south
                },
                '-' => {
                    cxn_one = Some((index - 1, iteration)); // east
                    cxn_two = Some((index + 1, iteration)); // west
                },
                _ => {}
            }
            //postMessageToWorker(show_message, &format!("letter: {}, cxn_one: {:?}, cxn_two: {:?}", character, cxn_one, cxn_two));
            let connection = Connection {
                cxn_one: cxn_one,
                cxn_two: cxn_two,
                character: *character,
            };

            row.push(connection);

        }
    grid.push(row);
    iteration += 1;
    });

    postMessageToWorker(true, &format!("Start x, y: {:?}, {:?}", start.0, start.1));
    let start_x = start.0;
    let start_y = start.1;

    let mut initial_connection: Option<(isize, isize)> = None;
    // check north
    if start_y > 0 && !initial_connection.is_some() { // no underflow
        let north_x: usize = start_x as usize;
        let north_y: usize = start_y as usize - 1;
        //postMessageToWorker(true, &format!("cell to the north: {:?}", grid[north_y][north_x]));
        if grid[north_y][north_x].cxn_one == Some((start_x, start_y)) || grid[north_y][north_x].cxn_two == Some((start_x, start_y)) {
            postMessageToWorker(true, &format!("Found connection to the north: {:?}", grid[north_y][north_x]));
            initial_connection = Some((north_x as isize, north_y as isize));
        }
    }

    // check east
    if start_x < grid[0].len() as isize - 1 && !initial_connection.is_some() { // no overflow
        let east_x: usize = start_x as usize + 1;
        let east_y: usize = start_y as usize;
        //postMessageToWorker(true, &format!("cell to the east: {:?}", grid[east_y][east_x]));
        if grid[east_y][east_x].cxn_one == Some((start_x, start_y)) || grid[east_y][east_x].cxn_two == Some((start_x, start_y)) {
            postMessageToWorker(true, &format!("Found connection to the east: {:?}", grid[east_y][east_x]));
            initial_connection = Some((east_x as isize, east_y as isize));
        }
    }

    // check south
    if start_y < grid.len() as isize - 1 && !initial_connection.is_some() { // no overflow
        let south_x: usize = start_x as usize;
        let south_y: usize = start_y as usize + 1;
        //postMessageToWorker(true, &format!("cell to the south: {:?}", grid[south_y][south_x]));
        if grid[south_y][south_x].cxn_one == Some((start_x, start_y)) || grid[south_y][south_x].cxn_two == Some((start_x, start_y)) {
            postMessageToWorker(true, &format!("Found connection to the south: {:?}", grid[south_y][south_x]));
            initial_connection = Some((south_x as isize, south_y as isize));
        }
    }

    // check west
    if start_x > 0 && !initial_connection.is_some() { // no underflow
        let west_x: usize = start_x as usize - 1;
        let west_y: usize = start_y as usize;
        //postMessageToWorker(true, &format!("cell to the west: {:?}", grid[west_y][west_x]));
        if grid[west_y][west_x].cxn_one == Some((start_x, start_y)) || grid[west_y][west_x].cxn_two == Some((start_x, start_y)) {
            postMessageToWorker(true, &format!("Found connection to the west: {:?}", grid[west_y][west_x]));
            initial_connection = Some((west_x as isize, west_y as isize));
        }
    }


    let mut current_location = start;
    let mut next_location = initial_connection.unwrap();
    let mut next_next_location = pick_outgoing_connection(current_location, &grid[next_location.1 as usize][next_location.0 as usize]);

    let mut sigma: Vec<isize> = Vec::new();

    postMessageToWorker(true, &format!("Current / next location: {:?}, {:?}", current_location, next_location));
    

    let left_term = current_location.0 * next_location.1;
    let right_term = current_location.1 * next_location.0;
    let sigma_term = left_term - right_term;
    sigma.push(sigma_term);
    postMessageToWorker(true, &format!("left term, right term: {:?} {:?}", left_term, right_term ));
    postMessageToWorker(true, &format!("Sigma term: {:?}", sigma_term));
    postMessageToWorker(true, &format!("Sigma: {:?}", sigma));


    loop {
        current_location = next_location;
        next_location = next_next_location.unwrap();
        next_next_location = pick_outgoing_connection(current_location, &grid[next_location.1 as usize][next_location.0 as usize]);
        postMessageToWorker(true, &format!("Current / next location: {:?}, {:?}", current_location, next_location));

        
        let left_term = current_location.0 * next_location.1;
        let right_term = current_location.1 * next_location.0;
        let sigma_term = left_term - right_term;
        sigma.push(sigma_term);
        postMessageToWorker(true, &format!("left term, right term: {:?} {:?}", left_term, right_term ));
        postMessageToWorker(true, &format!("Sigma term: {:?}", sigma_term));
        postMessageToWorker(true, &format!("Sigma: {:?}", sigma));


        if next_next_location.unwrap() == start {
            

            postMessageToWorker(true, &format!("Current / next location: {:?}, {:?}", next_location, next_next_location.unwrap()));
            let left_term = next_location.0 * next_next_location.unwrap().1;
            let right_term = next_location.1 * next_next_location.unwrap().0;
            let sigma_term = left_term - right_term;
            sigma.push(sigma_term);
            postMessageToWorker(true, &format!("left term, right term: {:?} {:?}", left_term, right_term ));
            postMessageToWorker(true, &format!("Sigma term: {:?}", sigma_term));
            postMessageToWorker(true, &format!("Sigma: {:?}", sigma));
            

            break;
        }
    }

    postMessageToWorker(true, &format!("Sigma: {:?}", sigma));

    let sum: isize = sigma.iter().sum();
    let abs = sum.abs();
    let answer = abs / 2;

    postMessageToWorker(true, &format!("sum, abs, answer: {:?}, {:?}, {:?}", sum, abs, answer));

}