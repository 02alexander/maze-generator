extern crate imagefmt;
extern crate rand;
extern crate clap;

use clap::{Arg, App};
use imagefmt::{ColFmt, ColType};
use maze::maze::Maze;
use maze::depthfirstsearch::depth_first;

mod maze;

fn main() {
	let matches = App::new("maze_generator")
		.author("Alexander Berntsson. <alexande.berntsson@gmail.com>")
		.arg(Arg::with_name("Width")
			.short("w")
			.long("width")
			.takes_value(true)
			.help("Sets the width of the maze"))
		.arg(Arg::with_name("Height")
			.short("h")
			.long("height")
			.takes_value(true)
			.help("Sets the height of the maze"))
		.arg(Arg::with_name("OUTPUT")
			.short("o")
			.long("output")
			.takes_value(true)
			.required(true)
			.help("The output file"))
		.get_matches();
	
	let std_width  = 19;
	let std_height = 19;

	let width = if let Some(width_str) = matches.value_of("Width") {
		width_str.parse::<usize>().unwrap_or(std_width)
	} else { std_width };
	let height = if let Some(height_str) = matches.value_of("Height") {
		height_str.parse::<usize>().unwrap_or(std_height)
	} else { std_height };
	let file_name = matches.value_of("OUTPUT").unwrap();

	let maze = Maze::new(width,height);
	let maze = depth_first(maze);
	let image = maze.as_image();
	if let Err(e) = imagefmt::write(file_name, maze.width, maze.height, ColFmt::RGB, &image, ColType::Auto) {
		println!("Failed to save image: {:?}", e);
	}
}
