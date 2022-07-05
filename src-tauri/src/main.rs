#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use rand::{prelude::ThreadRng, Rng};
use serde::{Deserialize, Serialize};
use std::sync::{Mutex, MutexGuard};
use tauri::State;

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
enum TileValue {
    BOMB,
    FLAG,
    NUMBER,
    EMPTY,
}

trait TV {

}

#[derive(Debug, Serialize, Deserialize, Clone)]
struct Tile {
    x: i16,
    y: i16,
    value: TileValue,
	bombs: u16,
    clicked: bool,
}

#[derive(Debug, Serialize, Deserialize, Default)]
struct Board {
    size: i16,
    tiles: Vec<Tile>,
}

struct BoardState(Mutex<Board>);

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![init_board, tile_clicked])
        .manage(BoardState(Default::default()))
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

/**
 * Checks if there are any bombs in a 3x3 square around the tile and
 * puts a value accordingly.
 * 
 * 0,0  1,0  2,0
 * 0,1  1,1  2,1
 * 0,2  1,2  2,2
 */
fn set_tile_bombs(board: &mut Board) {

	let clone = board.tiles.clone();

	for mut tile in board.tiles.iter_mut() {

		// if this is a bomb, skip
		if tile.value == TileValue::BOMB { 
			continue;
		};

		// Get the tiles around
		// let tx = tile.x;
		// let ty = tile.y;

		println!("Checking tile {};{}", tile.x, tile.y);

		let x_start = tile.x - 1;
		let x_stop = tile.x + 1;

		let y_start = tile.y - 1;
		let y_stop = tile.y + 1;

		for _row in x_start..x_stop {
			for _col in y_start..y_stop {

				print!("Is there a bomb at {};{}", _row, _col);

				match clone.iter().find(|t| t.x == tile.x && t.y == tile.y) {
					Some(t) => {

						print!(" {:?} ", t.value);

						if t.value == TileValue::BOMB {
							println!("Yes!");
							tile.bombs += 1;	
						} else {
							println!("No!");
						}
					},
					None => {
						println!("No tile at {};{}", _row, _col);
					}
				}
	
			}
		}
	}


}

/**
 * Generate the full board.
 */
#[tauri::command]
fn init_board(
    state: State<BoardState>,
    board_size: i16,
    bombs_density: f32,
) -> Result<Board, String> {
    let mut board: MutexGuard<Board> = state.0.lock().unwrap();

    board.size = board_size;
    board.tiles.clear();

    // Initialize it with default values
    for row in 0..board.size {
        for col in 0..board_size {
            let mut tile: Tile = Tile {
                x: col,
                y: row,
                value: TileValue::EMPTY,
                clicked: false,
				bombs: 0
            };

            board.tiles.push(tile);
        }
    }

    let nbombs: f32 = i16::pow(board.size, 2) as f32 * bombs_density;
    let mut rng: ThreadRng = rand::thread_rng();

    // pick a random position
    for _i in 0..nbombs as i32 {
        let rand_x: i16 = rng.gen_range(0..board.size);
        let rand_y: i16 = rng.gen_range(0..board.size);

        // find the associated tile in the board and place a bomb
        board
            .tiles
            .iter_mut()
            .filter(|tile: &&mut Tile| tile.x == rand_x && tile.y == rand_y)
            .for_each(|mut match_tile: &mut Tile| match_tile.value = TileValue::BOMB);
    }

    print_board(&board);
	set_tile_bombs(&mut board);
	

    println!("Returning board: ");

    Ok(Board {
        size: board_size,
        tiles: board.tiles.to_vec(),
    })
}

#[tauri::command]
fn tile_clicked(state: State<BoardState>, tile: Tile) -> Result<Board, String> {

	
	if tile.value == TileValue::BOMB {
		return Err("You clicked on a bomb !".into())
    }
	
	let mut board: MutexGuard<Board> = state.0.lock().unwrap();
    
	board.tiles.iter_mut().find(|t| t.x == tile.x && t.y == tile.y).unwrap().clicked = true;

    Ok(Board {
		size: board.size,
		tiles: board.tiles.to_vec()
	})
}

/**
 * Prints the board to the console
 */
fn print_board(board: &Board) {
    let mut current: i16 = 0;

    for tile in board.tiles.iter() {
        if tile.y > current {
            println!("");
            current = tile.y;
        }

        match tile.value {
            TileValue::BOMB => print!("B "),
            TileValue::FLAG => print!("F "),
            TileValue::EMPTY => print!("E "),
            TileValue::NUMBER => print!("V "),
        }
    }

    println!("");
}
