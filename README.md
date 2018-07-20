
# maze generator
Creates a maze using the [depth first search](https://en.wikipedia.org/wiki/Maze_generation_algorithm) algortihm.
The start of the maze will be colored red, and the end blue.

### Install
First you need to install rust, on arch linux just run
```
$ sudo pacman -S rust
```
To build it run
```
$ cargo build --release
```
Then you will have an executable in the target/release/ directory.

### Usage
```
$ cd target/release/
$ ./maze-generator --width=30 --heigth=50 -o maze.png
```
