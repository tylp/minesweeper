import { table } from 'console';
import React, { ChangeEvent, useState } from 'react';
import './App.css';

interface Tile {
	x: number;
	y: number;
	value: "1" | "2" | "3" | "4" | "bomb" | "flag";
	clicked: boolean;
}

function App() {

	const [boardSise, setBoardSize] = useState<number>(10);
	const [data, setData] = useState<Array<Tile>>([]);

	function handleBoardSize(event: ChangeEvent<HTMLInputElement>) {
		setBoardSize(+event.target.value);
		generateData();
	}

	function displayTable() {
		generateData();
	}

	/**
	 * Generate data for each of the tiles in the board.
	 */
	function generateData() {

		let data: Array<Tile> = [];

		for (let row = 0; row < boardSise; row++) {
			for (let col = 0; col < boardSise; col++) {
				data.push({
					x: col,
					y: row,
					value: "flag",
					clicked: false
				});
			}
		}
	}

	return (
		<div className="App">
			<div className="header">
				<input type={"number"} placeholder="Board size" value={boardSise} onChange={handleBoardSize}></input>
			</div>
			<table className='board'>
				<tr>
				{
					data.map((tile: Tile) =>
						<td>{tile.value}</td>
					)
				}
				</tr>
			</table>
		</div>
	);
}

export default App;
