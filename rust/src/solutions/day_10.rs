#![allow(unreachable_code)]
#![allow(unused_variables)]
//use web_sys::console;
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
    postMessageToWorker(true, "Part 1: \n");
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

    //postMessageToWorker(true, &format!("Grid: {:?}", grid));
    //postMessageToWorker(true, &format!("Cell 0,0: {:?}", grid[0][0]));
    //postMessageToWorker(true, &format!("Cell 4,4: {:?}", grid[4][4]));
    //postMessageToWorker(true, &format!("Cell 1,2: {:?}", grid[1][2]));
    //postMessageToWorker(true, &format!("Start: {:?}", start));

    postMessageToWorker(true, &format!("Start x, y: {:?}, {:?}", start.0, start.1));
    let start_x = start.0;
    let start_y = start.1;

    let mut found_connection: bool = false;
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
    postMessageToWorker(true, &format!("Current location: {:?}, {:?}", current_location, grid[current_location.1 as usize][current_location.0 as usize]));
    postMessageToWorker(true, &format!("Next location: {:?}, {:?}", next_location, grid[next_location.1 as usize][next_location.0 as usize]));
    postMessageToWorker(true, &format!("Next next location: {:?}, {:?}", next_next_location, grid[next_next_location.unwrap().1 as usize][next_next_location.unwrap().0 as usize]));
    postMessageToWorker(true, &format!("Path: {:?}", path));

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
        postMessageToWorker(true, &format!("Last value: {}", last_value));
        midpoint = (last_value + 1) / 2;
    }
    postMessageToWorker(true, &format!("Midpoint: {}", midpoint));

    
    //let mut current_location = initial_connection.unwrap();
    //postMessageToWorker(true, &format!("Current location: {:?}", current_location));

    //let mut next_location = pick_outgoing_connection(current_location, &grid[initial_connection.unwrap().1 as usize][initial_connection.unwrap().0 as usize]);
    //postMessageToWorker(true, &format!("Next location: {:?}", next_location));

    //current_location = pick_outgoing_connection(current_location, &grid[initial_connection.unwrap().1 as usize][initial_connection.unwrap().0 as usize]).unwrap();
    //postMessageToWorker(true, &format!("New location: {:?}", current_location));

    //current_location = pick_outgoing_connection(current_location, &grid[initial_connection.unwrap().1 as usize][initial_connection.unwrap().0 as usize]).unwrap();
    //postMessageToWorker(true, &format!("New location: {:?}", current_location));

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
    return;
    postMessageToWorker(true, "Part 2: \n");
    let mut iteration = -1;
    let content = include_str!("input/day_10_part_1_test_input_1.txt");
    // let content = include_str!("input/day_10_part_1_input.txt");

    content.lines().for_each(|line| {
        // Provide a mechanism to limit the volume of output on the console.
        iteration += 1;
        let mut show_message = false;
        if (iteration) % 300 == 0  {
            show_message = true;
        }

        let characters: Vec<_> = line.chars().collect();
        if characters[0] == '#' {
            // postMessageToWorker(show_message, "Skipping line because it is a comment.");
            return;
        }

        postMessageToWorker(show_message, " ");
        postMessageToWorker(show_message, &format!("Iteration: {}, input: {}", iteration, line));
    });
}