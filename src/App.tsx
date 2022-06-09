import { table } from 'console';
import React, { ChangeEvent, useEffect, useState } from 'react';
import './App.css';
import {emit, listen} from '@tauri-apps/api/event';

interface Tile {
	x: number,
	y: number,
	value: number,
	clicked: boolean
}

function App() {

	const [tiles, setTiles] = useState<Array<Tile>>([]);
	const [boardSise, setBoardSize] = useState<number>(10);

	function handleBoardSize(event: ChangeEvent<HTMLInputElement>) {
		setBoardSize(+event.target.value);
		emit("board-size", {
			boardSize: boardSise
		});
	}

	function renderTable() {
		return(
			<table>
				
			</table>
		);
	}

	return (
		<div className="App">
			<div className="header">
				<input type={"number"} placeholder="Board size" value={boardSise} onChange={handleBoardSize}></input>
			</div>
			<div className='board'>
				
			</div>
		</div>
	);
}

export default App;
