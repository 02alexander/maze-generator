
use std::str::FromStr;
use std::num::ParseIntError;

pub struct Maze {
	pub width: usize,
	pub height: usize,
	pub grid: Vec<Vec<u8>>,
	pub start: Coordinate,
	pub end: Coordinate,
}

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub struct Coordinate {
	pub x: usize,
	pub y: usize,
}

impl Maze {
	pub fn new(width: usize, height: usize) -> Self {
		let width  = width  - (width +1)%2;
		let height = height - (height+1)%2;

		let grid = vec![vec![0;height]; width]; 
		Maze { 
			width:width,
			height:height, 
			grid:grid, 
			start:Coordinate::new(0,0),
			end:Coordinate::new(width-1, height-1)
		}
	}

	///Creates all the cells.
	pub fn init(&mut self) {
		for i in (0..self.width).filter(|x|x%2==0) {
			for j in (0..self.height).filter(|x|x%2==0) {
				self.grid[i][j] = 1; // 1 means the tile is a path
			}
		}
	}

	///Creates an RGB image, where the walls are black, 
	///the path is white, the start is red and the end is blue
	pub fn as_image(&self) -> Vec<u8> {
		let mut image = Vec::with_capacity(self.width*self.height*3);
		for y in 0..self.height {
			for x in 0..self.width {
				if self.start.x == x && self.start.y == y {
					// red
					image.push(255); image.push(0); image.push(0);
				} else if self.end.x == x && self.end.y == y {
					// blue
					image.push(0); image.push(0); image.push(255);
				} else if self.grid[x][y] == 1 {
					// white
					image.push(255); image.push(255); image.push(255); 
				} else {
					// black
					image.push(0); image.push(0); image.push(0);
				}
			}
		}
		image
	}

	/// Rounds the coordinates to the nearest even number, then sets it as the end.
	pub fn set_end(&mut self, end: Coordinate) {
		self.end = Coordinate::new(end.x-end.x%2, end.y-end.y%2);
	}

	/// Rounds the coordinates to the nearest even number, then sets it as the end.
	pub fn set_start(&mut self, start: Coordinate) {
		self.start = Coordinate::new(start.x-start.x%2, start.y-start.y%2);
	}

	/// Returns true if any neighbouring tile is a path. 
	pub fn is_visited(&self, cell: Coordinate) -> bool {

		let mut is_visited = false; // set to true when we find an adjacent tile

		// if left cell is connected
		if self.is_inside(cell.x as i32 - 1, cell.y as i32) && self.grid[cell.x-1][cell.y] == 1 {
			is_visited = true;
		}
		if self.is_inside(cell.x as i32 + 1, cell.y as i32) && self.grid[cell.x+1][cell.y] == 1 {
			is_visited = true;
		}
		if self.is_inside(cell.x as i32, cell.y as i32 - 1) && self.grid[cell.x][cell.y-1] == 1 {
			is_visited = true;
		}
		if self.is_inside(cell.x as i32, cell.y as i32 + 1) && self.grid[cell.x][cell.y+1] == 1 {
			is_visited = true;
		}
		is_visited
	}

	///Returns true if the x, y coordinates are inside the maze. 
	pub fn is_inside(&self, x: i32, y: i32) -> bool {
		x >= 0 && y >= 0 && x < self.width as i32 && y < self.height as i32
	}

	// Connects 2 cells that are right beside each other.
	pub fn connect(&mut self, cell_a: Coordinate, cell_b: Coordinate) {
		if cell_b.x < cell_a.x {
			self.grid[cell_a.x-1][cell_a.y] = 1;
		} else if cell_b.x > cell_a.x {
			self.grid[cell_a.x+1][cell_a.y] = 1;
		} else if cell_b.y < cell_a.y {
			self.grid[cell_a.x][cell_a.y-1] = 1;
		} else if cell_b.y > cell_a.y {
			self.grid[cell_a.x][cell_a.y+1] = 1;
		}
	}
}

impl Coordinate {
	pub fn new(x: usize, y: usize) -> Self {
		Coordinate { x:x, y:y }
	}
}

impl FromStr for Coordinate {
	type Err = ParseIntError;

	fn from_str(s: &str) -> Result<Self, Self::Err> {
		let coords: Vec<&str> = s.trim_matches(|p| p == '(' || p == ')')
								 .split(",")
								 .collect();
		let x = coords[0].parse::<usize>()?;
		let y = coords[1].parse::<usize>()?;
		Ok(Coordinate::new(x,y))
	}
}

#[cfg(test)]
mod test {
	use super::{Maze, Coordinate};

	#[test]
	fn constructor() {
		let maze = Maze::new(6,3);
		assert_eq!(3, maze.height);
		assert_eq!(5,maze.width);
	}

	#[test]
	fn inside() {
		let maze = Maze::new(3, 5);
		assert!(maze.is_inside(2,0));
		assert!(!maze.is_inside(2,5));
		assert!(maze.is_inside(2,4));


	}

	#[test]
	fn connect() {
		let mut maze = Maze::new(3, 3);
		maze.grid = vec![
			vec![1,0,1],
			vec![0,0,0],
			vec![1,0,1]
		];
		maze.connect(Coordinate::new(0,0), Coordinate::new(0,0));
		assert_eq!(0, maze.grid[1][0]);
		assert_eq!(0, maze.grid[0][1]);
		maze.connect(Coordinate::new(0,0), Coordinate::new(2,0));
		assert_eq!(1, maze.grid[1][0]);
		assert_eq!(0, maze.grid[0][1]);
		maze.connect(Coordinate::new(0,0), Coordinate::new(0,2));
		assert_eq!(1, maze.grid[0][1]);
		assert_eq!(0, maze.grid[1][2]);
	}

	#[test]
	fn visited() {
		let mut maze = Maze::new(3,3);
		maze.grid = vec![
			vec![1,0,1],
			vec![1,0,0],
			vec![1,1,1]
		];
		// 1 1 1
		// 0 0 1
		// 1 0 1

		assert_eq!(true, maze.is_visited(Coordinate::new(0,0)));
		assert_eq!(true, maze.is_visited(Coordinate::new(2,0)));
		assert_eq!(false, maze.is_visited(Coordinate::new(0,2)));

	}
}