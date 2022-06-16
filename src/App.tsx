import { clear, table } from 'console';
import React, { ChangeEvent, ReactNode, useEffect, useState } from 'react';
import { invoke } from '@tauri-apps/api';
import { AppBar, Toolbar, Box, Button, Grid, Input, TableContainer, Paper, Table, TableBody, TableCell, TableHead, TableRow, Typography, createTheme, ThemeProvider, CssBaseline } from '@mui/material';


enum TILE_VALUE {
	BOMB,
	FLAG,
	NUMBER,
	EMPTY
}

interface ITile {
	x: number,
	y: number,
	value: string,
	clicked: boolean
}

interface IBoard {
	size: number,
	tiles: Array<ITile>
}

const darkTheme = createTheme({
	palette: {
		mode: 'dark'
	}
});

function App() {

	const [tiles, setTiles] = useState<Array<ITile>>([]);
	const [rows, setRows] = useState<Array<Array<ITile>>>([]);
	const [boardSize, setBoardSize] = useState<number>(10);
	const [bombsDensity, setBombsDensity] = useState<number>(0.2);

	useEffect(() => {
		// initBoard();
	});

	function handleBoardSize(event: ChangeEvent<HTMLInputElement>) {
		setBoardSize(+event.target.value);
	}

	function handleBombsDensity(event: ChangeEvent<HTMLInputElement>) {
		setBombsDensity(+event.target.value);
	}

	function initBoard() {
		// First clear the board
		clearBoard();

		// Then, invoke rust api
		invoke('init_board', { boardSize: boardSize, bombsDensity: bombsDensity }).then((result) => {
			let board = result as IBoard;
			setTiles(board.tiles);
			processTiles();
		}).catch((error) => {
			console.log(error);
		})
	}

	function clearBoard() {
		setTiles([]);
		setRows([]);
	}

	/**
	 * Split the tiles into multiple rows
	 */
	function processTiles() {

		for (let row = 0; row < boardSize; row ++) {
			// Find the tiles with y = 0
			const filteredTiles = tiles.filter(tile => tile.x === row);
			setRows(current => [...current, filteredTiles]);
		}
	}

	function handleTileClick(tile: ITile) {
		console.log(`Clicked on tile [${tile.x};${tile.y}]`);
	}

	return (
		<ThemeProvider theme={darkTheme}>
			<CssBaseline/>
			<Box sx={{ flexGrow: 1 }}>
				<AppBar position='static'>
					<Toolbar>
						<Input type={"number"} placeholder="Board size" value={boardSize} onChange={handleBoardSize} />
						<Input type={"number"} placeholder="Bombs density" value={bombsDensity} onChange={handleBombsDensity} />
						<Button onClick={initBoard} >Init board</Button>
					</Toolbar>
				</AppBar>

			</Box>
			<Box className="container">
				{
					rows.map((row, index) => {
						return(
							<Box key={`row${index}`} className="col">
								{
									row.map((tile) => {
										return(
											<div
												onClick={() => handleTileClick(tile)}
												key={`tile${tile.x}${tile.y}`} 
												className={`tile ${tile.value === "BOMB" ? "bkg-red" : ""}`}
											>
												<Typography variant='caption'>{tile.value.charAt(0)}</Typography>
												
											</div>
										)
									})
								}
							</Box>
						)
					})
				}
			</Box>
		</ThemeProvider>
	);
}

export default App;
