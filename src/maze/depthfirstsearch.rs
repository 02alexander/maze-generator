
use super::maze::Maze;
use super::maze::Coordinate;

use rand::{thread_rng, Rng};

/// Generates a maze using the depth first search algorithm.
pub fn depth_first(mut maze: Maze) -> Maze {
	maze.init();
	let mut back_tracker = Vec::<Coordinate>::new();
	let mut last = maze.start;
	'start: while {	
		/*let cur = if let Some(cell) = get_unvisited_neighbor(&maze, last) {
			back_tracker.push(cell);
			cell
		} else {
			back_tracker.pop().unwrap()
		};
		println!("({}, {}), ({}, {})", last.x,last.y,cur.x,cur.y);
		maze.connect(cur,last);
		!back_tracker.is_empty()
		*/

		if let Some(cell) = get_unvisited_neighbor(&maze, last) {
			back_tracker.push(cell);
			maze.connect(cell, last);
			last = cell;
		} else {
			last = back_tracker.pop().unwrap();
		}
		!back_tracker.is_empty()
	} {}
	maze
}

fn is_dead(maze: &Maze, cell: Coordinate) -> bool {
	get_unvisited_neighbor(&maze, cell).is_none()
}

fn get_unvisited_neighbor(maze: &Maze, cell: Coordinate) -> Option<Coordinate> {
	let mut rnd = thread_rng();
	let mut unvisited_neighbours = Vec::<Coordinate>::new();

	if maze.is_inside(cell.x as i32 - 2, cell.y as i32) && !maze.is_visited(Coordinate::new(cell.x-2,cell.y)) {
		unvisited_neighbours.push(Coordinate::new(cell.x-2,cell.y));
	}
	if maze.is_inside(cell.x as i32 + 2, cell.y as i32) && !maze.is_visited(Coordinate::new(cell.x+2,cell.y)) {
		unvisited_neighbours.push(Coordinate::new(cell.x+2,cell.y));
	}
	if maze.is_inside(cell.x as i32, cell.y as i32 - 2) && !maze.is_visited(Coordinate::new(cell.x,cell.y-2)) {
		unvisited_neighbours.push(Coordinate::new(cell.x, cell.y-2));
	}
	if maze.is_inside(cell.x as i32, cell.y as i32 + 2) && !maze.is_visited(Coordinate::new(cell.x,cell.y+2)) {
		unvisited_neighbours.push(Coordinate::new(cell.x,cell.y+2));
	}

	if unvisited_neighbours.is_empty() {
		return None
	}

	Some(unvisited_neighbours[rnd.gen_range(0,unvisited_neighbours.len())])

}

mod test {
	use super::*;

	#[test]
	fn unvisited_neighbor() {
		let mut maze = Maze::new(3,3);
		maze.grid = vec![
			vec![1,0,1],
			vec![1,0,0],
			vec![1,1,1]
		];
		// 1 1 1
		// 0 0 1
		// 1 0 1

		assert_eq!(Some(Coordinate::new(0,2)), get_unvisited_neighbor(&maze, Coordinate::new(0,0)));
		assert_eq!(Some(Coordinate::new(0,2)), get_unvisited_neighbor(&maze, Coordinate::new(2,2)));
		assert_eq!(None, get_unvisited_neighbor(&maze, Coordinate::new(2,0)));
		assert_eq!(None, get_unvisited_neighbor(&maze, Coordinate::new(0,2)));
	}
}