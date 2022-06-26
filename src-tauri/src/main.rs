#![cfg_attr(
	all(not(debug_assertions), target_os = "windows"),
	windows_subsystem = "windows"
)]

use std::sync::{Mutex, MutexGuard};
use rand::{Rng, prelude::ThreadRng};
use tauri::State;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
enum TileValue {
	BOMB,
	FLAG,
	NUMBER,
	EMPTY
}

#[derive(Debug, Serialize, Deserialize)]
struct Tile {
	x: u16,
	y: u16,
	value: TileValue,
	clicked: bool
}


#[derive(Debug, Serialize, Deserialize, Default)]
struct Board {
	size: u16,
	tiles: Vec<Tile>
}

struct BoardState(Mutex<Board>);

fn main() {
	tauri::Builder::default()
		.invoke_handler(tauri::generate_handler![
			init_board,
			tile_clicked
		])
		.manage(BoardState(Default::default()))
		.run(tauri::generate_context!())
		.expect("error while running tauri application");
}

/**
 * Generate the full board.
 */
#[tauri::command]
fn init_board(board_size: u16, bombs_density: f32) -> Board {

	let mut board = Board {
		size: board_size,
		tiles: Vec::new()
	};

	// let mut board: MutexGuard<Board> = boardState.0.lock().unwrap();

	// Initialize it with default values
	for row in 0..board.size {
		for col in 0..board_size {

			let tile: Tile = Tile {
				x: col,
				y: row,
				value: TileValue::EMPTY,
				clicked: false
			};

			board.tiles.push(tile);
		}
	}

	let nbombs: f32 = u16::pow(board.size, 2) as f32 * bombs_density;
	let mut rng: ThreadRng = rand::thread_rng();

	// pick a random position
	for _i in 0..nbombs as i32 {
		let rand_x: u16 = rng.gen_range(0..board.size);
		let rand_y: u16 = rng.gen_range(0..board.size);

		// find the associated tile in the board and place a bomb
		board.tiles.iter_mut().filter(|tile: &&mut Tile| tile.x == rand_x && tile.y == rand_y)
			.for_each(|mut match_tile: &mut Tile| match_tile.value = TileValue::BOMB);
	}

	board
}

#[tauri::command]
fn tile_clicked(_state: State<Board>, _tile: Tile) -> Result<(), String> {
	
	println!("click");

	let _res: &str = "clicked";
	Ok(())
}

/**
 * Prints the board to the console
 */
fn print_board(board: &Board) {
	
	let mut current: u16 = 0;

	for tile in board.tiles.iter() {

		if tile.y > current {
			println!("");
			current = tile.y;
		}

		match tile.value {
			TileValue::BOMB => print!("B "),
			TileValue::FLAG => print!("F "),
			TileValue::EMPTY => print!("E "),
			TileValue::NUMBER => print!("V ")
		}
	}

	println!("");
}
