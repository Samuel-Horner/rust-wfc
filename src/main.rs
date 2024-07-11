use core::panic;
use std::env;
use rand::prelude::*;

const NUM_TILES: usize = 5;

struct Tile<'a> { // Tile Struct
    str: &'a str,
    rules: [bool; NUM_TILES]
}

const TILES: [Tile; NUM_TILES] = [ // Initialises tile set
    Tile{str: "\x1b[1;34m~\x1b[0m", rules: [true, true, true, false, false]},
    Tile{str: "\x1b[33m=\x1b[0m", rules: [false, false, true, false, false]},
    Tile{str: "\x1b[1;32m#\x1b[0m", rules: [false, true, true, true, true]},
    Tile{str: "\x1b[32m|\x1b[0m", rules: [false, false, true, true, true]},
    Tile{str: "\x1b[37m^\x1b[0m", rules: [false, false, false, true, false]}
];

struct Pos { // Pos Struct
    x: i32,
    y: i32
}

const OFFSETS: [Pos; 4] = [Pos{x: 0, y: -1}, Pos{x: 1, y: 0}, Pos{x: 0, y: 1}, Pos{x: -1, y: 0}];

fn add_pos (a: &Pos, b: &Pos) -> Pos { // Adds two pos's together
    return Pos{x: a.x + b.x, y: a.y + b.y};
}

#[derive(Clone)]
struct Cell {
    id: usize,
    visited: bool
}

fn generate(map: &mut Vec<Vec<Cell>>, width: usize, height: usize) { // Performs the wfc on map
    // Start cell
    let mut rng = rand::thread_rng();
    let mut prop_pos = Pos{x: rng.gen_range(0 ..= width - 1) as i32, y: rng.gen_range(0 ..= height - 1) as i32};
    map[prop_pos.y as usize][prop_pos.x as usize].id = NUM_TILES; // Forces the first cell to be the last tile
    // Nice for this specific tile set, comment out for other tile sets.
    loop {
        propagate(map, &prop_pos, width, height, true, &mut rng);

        let mut min_poss = NUM_TILES + 1;
        let mut min_poss_pos = Pos{x: -1, y: -1};
        for y in 0 .. height {
            for x in 0 .. width {
                let pos = Pos{x: x as i32, y: y as i32};
                let cell = &mut map[pos.y as usize][pos.x as usize];
                cell.visited = false;
                if cell.id != 0 { continue; }
                let possibilities = get_possibilities(map, &pos, width, height).len();
                if possibilities < min_poss {
                    min_poss = possibilities;
                    min_poss_pos = pos;
                } 
            }
        }

        if min_poss == NUM_TILES + 1 { break; }

        prop_pos = min_poss_pos;
    }
}

fn propagate(map: &mut Vec<Vec<Cell>>, pos: &Pos, width: usize, height: usize, hard: bool, rng: &mut ThreadRng) { // propagates change, if hard then choosing random possible tile
    if get_id(map, &pos, width, height) != 0 { return; }
    if !check_bounds(&pos, width, height) { return; }
    if map[pos.y as usize][pos.x as usize].visited { return; }
    map[pos.y as usize][pos.x as usize].visited = true;

    let possibilities = get_possibilities(map, &pos, width, height);
    let poss_count = possibilities.len();
    if poss_count == 0 { panic!("Invalid tile set"); }
    if poss_count == 1 { 
        map[pos.y as usize][pos.x as usize].id = possibilities[0];
    } else if hard {
        map[pos.y as usize][pos.x as usize].id = *possibilities.choose(rng).unwrap();
    }

    for offset in OFFSETS {
        let neighbour = add_pos(&pos, &offset);
        propagate(map, &neighbour, width, height, false, rng);
    }
}

fn get_possibilities(map: &Vec<Vec<Cell>>, pos: &Pos, width: usize, height: usize) -> Vec<usize> { // returns vector of possible tile ids at pos
    let mut possibilities_mask: [bool; NUM_TILES] = [true; NUM_TILES];
    let mut possibilities:Vec<usize> = Vec::new();
    for tile in 0 .. NUM_TILES {
        for offset in OFFSETS {
            let new_pos: &Pos = &add_pos(pos, &offset);
            let neighbour: usize = get_id(map, new_pos, width, height);
            if neighbour == 0 { continue; }
            if !TILES[tile].rules[neighbour - 1] {
                possibilities_mask[tile] = false;
                break;
            }
        }
        if possibilities_mask[tile] {
            possibilities.push(tile + 1);
        }
    }
    return possibilities;
}

fn get_id(map: &Vec<Vec<Cell>>, pos: &Pos, width: usize, height: usize) -> usize { // Returns tile id at pos
    if !check_bounds(pos, width, height) { return 0; }
    return map[pos.y as usize][pos.x as usize].id;
}

fn check_bounds(pos: &Pos, width: usize, height: usize) -> bool { // Returns true if pos is in bounds
    return !(pos.x >= width as i32 || pos.x < 0 || pos.y >= height as i32 || pos.y < 0);
}

fn print_map(map: &Vec<Vec<Cell>>) { // Print map
    print!("\x1b[H\x1b[0J");
    for row in map {
        for rid in row {
            let id = rid.id;
            if id == 0 {
                print!("- ");
            } else {
                print!("{} ", TILES[id - 1].str);
            }
        }
        println!();
    }
}

fn main() {
    // Parse args
    let args: Vec<String> = env::args().collect();
    if args.len() < 3 {
        panic!("Invalid arguments provided.\nPlease provide width and height.");
    }
    let width: usize = args[1].parse::<usize>().unwrap();
    let height: usize = args[2].parse::<usize>().unwrap();

    // Initialise maps
    let mut map: Vec<Vec<Cell>> = vec![vec![Cell{id: 0, visited: false}; width]; height];
    generate(&mut map, width, height);
    print_map(&map);
}
