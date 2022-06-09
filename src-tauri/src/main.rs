#![cfg_attr(
	all(not(debug_assertions), target_os = "windows"),
	windows_subsystem = "windows"
)]

use rand::Rng;
use tauri::State;
use tauri::Manager;

#[derive(Debug)]
enum TileValue {
	BOMB,
	FLAG,
	NUMBER,
	EMPTY
}

struct Tile {
	x: u16,
	y: u16,
	value: TileValue,
	clicked: bool
}

struct Board {
	size: u16,
	tiles: Vec<Tile>
}

fn main() {
	tauri::Builder::default()
		.setup(|app| {
			let id = app.listen_global("board-size", |event| {
				println!("Board size: {:?}", event.payload());
			});

			// app.unlisten(id);
			Ok(())
		})
		.run(tauri::generate_context!())
		.expect("error while running tauri application");

	let mut board: Board = init_board(10);

	println!("Initializing empty board...");
	print_board(&board);
	init_board_bombs(&mut board, 0.2);
	print_board(&board);
}

/**
 * Generate the full board.
 */
fn init_board(board_size: u16) -> Board {
	let mut board = Board {
		size: board_size,
		tiles: Vec::new()
	};

	// Initialize it with default values
	for row in 0..board.size {
		for col in 0..board_size {

			let tile = Tile {
				x: col,
				y: row,
				value: TileValue::EMPTY,
				clicked: false
			};

			board.tiles.push(tile);
		}
	}

	return board;
}

/**
 * Randomly place bombs accross the board.
 * Density represents the percentage of bombs among the tiles.
 */
fn init_board_bombs(board: &mut Board, density: f32) {
	let nbombs = u16::pow(board.size, 2) as f32 * density;
	let mut rng = rand::thread_rng();

	println!("Putting {} bombs among {} tiles.", nbombs, u16::pow(board.size, 2));

	// pick a random position
	for _i in 0..nbombs as i32 {
		let rand_x: u16 = rng.gen_range(0..board.size);
		let rand_y: u16 = rng.gen_range(0..board.size);

		// find the associated tile in the board and place a bomb
		board.tiles.iter_mut().filter(|tile| tile.x == rand_x && tile.y == rand_y)
			.for_each(|mut match_tile| match_tile.value = TileValue::BOMB);
	}
}

/**
 * Prints the board to the console
 */
fn print_board(board: &Board) {
	
	let mut current = 0;

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
