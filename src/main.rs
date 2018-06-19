extern crate imagefmt;
extern crate rand;

use imagefmt::{ColFmt, ColType};

use maze::maze::Maze;
use maze::depthfirstsearch::depth_first;

mod maze;

fn main() {
	let maze = Maze::new(101,101);
	let maze = depth_first(maze);
	let image = maze.as_image();
	imagefmt::write("test.bmp", maze.width, maze.height, ColFmt::RGB, &image, ColType::Auto).unwrap();
}
