extern crate imagefmt;
extern crate rand;
extern crate clap;

use clap::{Arg, App};
use imagefmt::{ColFmt, ColType};
use maze::maze::{Maze, Coordinate};
use maze::depthfirstsearch::depth_first;
use std::str::FromStr;
use std::process::exit;
use std::error::Error;

mod maze;

fn main() {
	let matches = App::new("maze_generator")
		.about("The length and width must always be an odd number. 
			If an even number is given as input it will round it down to an odd number. 
			The same rules apply for the start and end coordinates of the Maze,")
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
		.arg(Arg::with_name("Start")
			.short("s")
			.long("start")
			.takes_value(true)
			.help("The end coordinates in a '(x,y)' format"))
		.arg(Arg::with_name("End")
			.short("e")
			.long("end")
			.takes_value(true)
			.help("The start coordinates in a '(x,y)' format"))
		.get_matches();
	
	let std_width  = 19;
	let std_height = 19;

	let width = if let Some(width_str) = matches.value_of("Width") {
		width_str.parse::<usize>().unwrap_or(std_width)
	} else { std_width };
	let height = if let Some(height_str) = matches.value_of("Height") {
		height_str.parse::<usize>().unwrap_or(std_height)
	} else { std_height };

	let std_start = Coordinate::new(0,0);
	let std_end   = Coordinate::new(width-1,height-1);
		
	let start = if let Some(cords) = matches.value_of("Start") {
		match Coordinate::from_str(cords) {
			Ok(c) => c,
			Err(e) => {
				eprintln!("Error reading start coordinates, Cause: {:?}", e);
				exit(1);
			}
		}
	} else {
		std_start
	};
	let end = if let Some(cords) = matches.value_of("End") {
		match Coordinate::from_str(cords) {
			Ok(c) => c,
			Err(e) => {
				eprintln!("Error reading end coordinates, Cause: {:?}", e);
				exit(1);
			}
		}
	} else {
		std_end
	};

	let file_name = matches.value_of("OUTPUT").unwrap();

	let mut maze = Maze::new(width,height);
	maze.set_end(end);
	maze.set_start(start);
	let maze = depth_first(maze);
	let image = maze.as_image();
	if let Err(e) = imagefmt::write(file_name, maze.width, maze.height, ColFmt::RGB, &image, ColType::Auto) {
		println!("Failed to save image: {:?}", e);
	}
}

