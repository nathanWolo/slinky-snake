// Welcome to
// __________         __    __  .__                               __
// \______   \_____ _/  |__/  |_|  |   ____   ______ ____ _____  |  | __ ____
//  |    |  _/\__  \\   __\   __\  | _/ __ \ /  ___//    \\__  \ |  |/ // __ \
//  |    |   \ / __ \|  |  |  | |  |_\  ___/ \___ \|   |  \/ __ \|    <\  ___/
//  |________/(______/__|  |__| |____/\_____>______>___|__(______/__|__\\_____>
//
// This file can be a nice home for your Battlesnake logic and helper functions.
//
// To get you started we've included code to prevent your Battlesnake from moving backwards.
// For more info see docs.battlesnake.com

use log::info;
use rand::seq::SliceRandom;
use serde_json::{json, Value};
use std::collections::HashMap;
use std::collections::HashSet;
use crate::{Battlesnake, Board, Game, Coord};

// info is called when you create your Battlesnake on play.battlesnake.com
// and controls your Battlesnake's appearance
// TIP: If you open your Battlesnake URL in a browser you should see this data
pub fn info() -> Value {
    info!("INFO");

    return json!({
        "apiversion": "1",
        "author": "nathanWolo", // TODO: Your Battlesnake Username
        "color": "#888888", // TODO: Choose color
        "head": "default", // TODO: Choose head
        "tail": "default", // TODO: Choose tail
    });
}

// start is called when your Battlesnake begins a game
pub fn start(_game: &Game, _turn: &u32, _board: &Board, _you: &Battlesnake) {
    info!("GAME START");
}

// end is called when your Battlesnake finishes a game
pub fn end(_game: &Game, _turn: &u32, _board: &Board, _you: &Battlesnake) {
    info!("GAME OVER");
}

// move is called on every turn and returns your next move
// Valid moves are "up", "down", "left", or "right"
// See https://docs.battlesnake.com/api/example-move for available data
pub fn get_move(_game: &Game, turn: &u32, _board: &Board, you: &Battlesnake) -> Value {

    // TODO: Step 4 - Move towards food instead of random, to regain health and survive longer
    //let food = &_board.food;
    
    let safe_moves = remove_suicide(_board, you);
    let safe_moves_copy = safe_moves.clone();
    let food_moves = check_food(_board, you); 
    let food_moves_copy = food_moves.clone();
    let chosen = safe_moves_copy.choose(&mut rand::thread_rng()).unwrap();
    if !find_intersection(vec![safe_moves, food_moves]).is_empty() {
        let food_moves = find_intersection(vec![safe_moves_copy, food_moves_copy]);
        let food_chosen = food_moves.choose(&mut rand::thread_rng()).unwrap();
        info!("MOVE {}: {}", turn, food_chosen);
        return json!({ "move": food_chosen });
    }
    //check if food moves is empty

    info!("MOVE {}: {}", turn, chosen);
    return json!({ "move": chosen });
}



pub fn check_food<'a>(_board: &Board, you: &Battlesnake) -> Vec<&'a str> {
    let my_head = &you.body[0];
    let food = &_board.food;
    let mut food_moves: HashMap<_, _> = vec![
        ("up", true),
        ("down", true),
        ("left", true),
        ("right", true),
    ].into_iter().collect();
    let food_moves_copy = food_moves.clone();
    //remove the move from food_moves if no food on that tile 
    for mv in food_moves_copy {
        if mv.0 == "up" {
            if !food.contains(&Coord{ x: my_head.x, y: my_head.y + 1 }) {
                food_moves.insert("up", false);
            }
        }
        if mv.0 == "down" && my_head.y != 0{
            if !food.contains(&Coord{ x: my_head.x, y: my_head.y - 1 }) {
                food_moves.insert("down", false);
            }
        }
        if mv.0 == "right" {
            if !food.contains(&Coord{ x: my_head.x + 1, y: my_head.y }) {
                food_moves.insert("right", false);
            }
        }
        if mv.0 == "left" && my_head.x != 0 {
            if !food.contains(&Coord{ x: my_head.x - 1, y: my_head.y }) {
                food_moves.insert("left", false);
            }
        }
    }
    return food_moves.into_iter().filter(|&(_, v)| v).map(|(k, _)| k).collect::<Vec<_>>();
}

pub fn remove_suicide<'a>(_board: &Board, you: &Battlesnake) -> Vec<&'a str> {
    let mut is_move_safe: HashMap<_, _> = vec![
        ("up", true),
        ("down", true),
        ("left", true),
        ("right", true),
    ].into_iter().collect();

    let my_head = &you.body[0]; // Coordinates of your head
    let my_neck = &you.body[1]; // Coordinates of your "neck"
    
    if my_neck.x < my_head.x { // Neck is left of head, don't move left
        is_move_safe.insert("left", false);

    } else if my_neck.x > my_head.x { // Neck is right of head, don't move right
        is_move_safe.insert("right", false);

    } else if my_neck.y < my_head.y { // Neck is below head, don't move down
        is_move_safe.insert("down", false);
    
    } else if my_neck.y > my_head.y { // Neck is above head, don't move up
        is_move_safe.insert("up", false);
    }

    // TODO: Step 1 - Prevent your Battlesnake from moving out of bounds
    let board_width = &_board.width;
    let board_height = &_board.height;
    println!("head_x: {}, head_y: {}", my_head.x, my_head.y);
    if my_head.x == 0 {
        is_move_safe.insert("left", false);
    }
    if my_head.x == board_width - 1 {
        is_move_safe.insert("right", false);
    }
    if my_head.y == 0 {
        is_move_safe.insert("down", false);
    }
    if my_head.y == board_height - 1 {
        is_move_safe.insert("up", false);
    }


    // TODO: Step 2 - Prevent your Battlesnake from colliding with itself
    let my_body = &you.body;
    for segment in my_body {
        if my_head.x != 0 { //prevent underflow
            if segment.x == my_head.x - 1 && segment.y == my_head.y {
                is_move_safe.insert("left", false);
            }
        }
        if segment.x == my_head.x + 1 && segment.y == my_head.y {
            is_move_safe.insert("right", false);
        }
        if my_head.y != 0 { // prevent underflow
            if segment.x == my_head.x && segment.y == my_head.y - 1 {
                is_move_safe.insert("down", false);
            }
        }
        if segment.x == my_head.x && segment.y == my_head.y + 1 {
            is_move_safe.insert("up", false);
        }

    }

    // TODO: Step 3 - Prevent your Battlesnake from colliding with other Battlesnakes
    let opponents = &_board.snakes;
    for opp in opponents {
        if opp.id == you.id {
            continue;
        }
        for segment in &opp.body {
            if my_head.x != 0 { //prevent underflow
                if segment.x == my_head.x - 1 && segment.y == my_head.y {
                    is_move_safe.insert("left", false);
                }
            }
            if segment.x == my_head.x + 1 && segment.y == my_head.y {
                is_move_safe.insert("right", false);
            }
            if my_head.y != 0 { // prevent underflow
                if segment.x == my_head.x && segment.y == my_head.y - 1 {
                    is_move_safe.insert("down", false);
                }
            }
            if segment.x == my_head.x && segment.y == my_head.y + 1 {
                is_move_safe.insert("up", false);
            }
        }
    }
    let safe_moves = is_move_safe.into_iter().filter(|&(_, v)| v).map(|(k, _)| k).collect::<Vec<_>>();
    return safe_moves;
}

// pub fn find_intersection<'a>(v1: Vec<&str>, v2: Vec<&str>) -> Vec<&'a str> {
//     let set1: HashSet<&str> = v1.into_iter().collect();
//     let set2: HashSet<&str> = v2.into_iter().collect();

//     let inter = set1.intersection(&set2).clone().into_iter().collect();
//     return inter;
// }

pub fn find_intersection(nums: Vec<Vec<&str>>) -> Vec<&str> {
    let mut intersect_result: Vec<&str> = nums[0].clone();

    for temp_vec in nums {
        let unique_a: HashSet<&str> = temp_vec.into_iter().collect();
        intersect_result = unique_a
            .intersection(&intersect_result.into_iter().collect())
            .map(|i| *i)
            .collect::<Vec<_>>();
    }
    intersect_result
}