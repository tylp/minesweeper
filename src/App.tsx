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
	clicked: boolean,
	bombs: number
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

	const [rows, setRows] = useState<Array<Array<ITile>>>([]);
	const [boardSize, setBoardSize] = useState<number>(10);
	const [bombsDensity, setBombsDensity] = useState<number>(0.2);
	const [lost, setLost] = useState<boolean>(false);

	useEffect(() => {
		// initBoard();
	}, []);

	function handleBoardSize(event: ChangeEvent<HTMLInputElement>) {
		setBoardSize(+event.target.value);
	}

	function handleBombsDensity(event: ChangeEvent<HTMLInputElement>) {
		setBombsDensity(+event.target.value);
	}

	async function initBoard() {
		// First clear the board
		clearBoard();
		setLost(false);

		try {
			const board: IBoard = await invoke('init_board', { boardSize: boardSize, bombsDensity: bombsDensity }) as IBoard;
			processTiles(board);
		} catch (err) {
			console.error(err);
		}
	}

	function clearBoard() {
		setRows([]);
	}

	/**
	 * Split the tiles into multiple rows
	 */
	async function processTiles(board: IBoard) {

		for (let row = 0; row < board.size; row ++) {
			// Find the tiles with y = 0
			const filteredTiles = board.tiles.filter(tile => tile.x === row);
			setRows(current => [...current, filteredTiles]);
		}
	}

	async function handleTileClick(tile: ITile) {
		try {
			const board: IBoard = await invoke('tile_clicked', {tile: tile});
			clearBoard();
			processTiles(board);
		} catch (err) {
			setLost(true);
			clearBoard();
		}
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
					lost ? <p>LOOOOOST !!!!!!!!!!!!!</p> : ""
				}
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
												className={`tile`}
											>
												<Typography variant='caption'>
													{/* {tile.clicked ? tile.bombs : ''} */}
													{tile.bombs + "_" + tile.value.charAt(0)}
												</Typography>
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
